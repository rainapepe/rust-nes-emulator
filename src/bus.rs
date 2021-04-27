use crate::{cartridge::Cartridge, ppu::Ppu2C02};
use crate::{cpu::Cpu6502, pad::Pad};

/*
     ___________________          __________________       _________________     _____________________      ____________________
    |                   |        |                  |     |                 |   |                     |    |                    |
    |       CPU         |        |       RAM        |     |       APU       |   |      CONTROLS       |    |      "STUFF"       |
    |___________________|        |__________________|     |_________________|   |_____________________|    |____________________|
             |                    0x0000  |   0x1FFF               |                      |                          |
             |----------------------------+---------------+--------+----------------+-----+--------------------------+---------------- BUS
             |                   _________________________|_________         _______|_________________
             |                  |                                   |       |                         |
             |                  |           PROGRAM ROM             |       |          MAPPER         |
             |                  |___________________________________|       |_________________________|
             |                   0x4020                       0xFFFF
             |
             |                   _________________          ______________________       _________________________
             |                  |       8KB       |        |         2KB          |     |                         |
             |                  |     PATTERN     |        |      NAMETABLE       |     |        PALETTE          |
             |                  |_________________|        |______________________|     |_________________________|
     ________|__________         0x0000  |  0x1FFF          0x2000     |    0x2FFF       0x3F00      |      0x3FFF
    |                   |                |                             |                             |
    |       PPU         |----------------+-----------------------------+-----------------------------+-------------------------------- BUS
    |___________________|

    OBS:
    - barramento de 16bits
    - memória Ram tem 8kb porém só 2kb são utilizados

    - PPU:
        - PATTERN MEMORY: local na memória onde será guardado os tiles(sprites), um tile tem 8x8 pixels, cada tile é composto por
            16 bytes divididos em dois planos de 8 bytes onde o primeiro plano controla o bit da posição 0 e o segundo plano a posição 1,
            juntando os dois planos cada posição tem valor de 0 a 3 representando a cor do pixel, onde 0 é transparente
        - PALETTE MEMORY: lista de paletas / cores
        - NAMETABLE MEMORY: Vram, controla os tiles que serão apresentados na tela

*/

pub struct Bus {
    pub ppu: Ppu2C02,
    pub cpu: Option<Cpu6502>,
    pub cartridge: Option<Cartridge>,
    pub pad1: Pad,
    pub pad2: Pad,
    ram: [u8; 2048],
    // A count of how many clocks have passed
    system_clock_counter: u32,

    // A simple form of Direct Memory Access is used to swiftly
    // transfer data from CPU bus memory into the OAM memory. It would
    // take too long to sensibly do this manually using a CPU loop, so
    // the program prepares a page of memory with the sprite info required
    // for the next frame and initiates a DMA transfer. This suspends the
    // CPU momentarily while the PPU gets sent data at PPU clock speeds.
    // Note here, that dma_page and dma_addr form a 16-bit address in
    // the CPU bus address space
    dma_page: u8,
    dma_addr: u8,
    dma_data: u8,

    // DMA transfers need to be timed accurately. In principle it takes
    // 512 cycles to read and write the 256 bytes of the OAM memory, a
    // read followed by a write. However, the CPU needs to be on an "even"
    // clock cycle, so a dummy cycle of idleness may be required
    dma_dummy: bool,

    // Finally a flag to indicate that a DMA transfer is happening
    dma_transfer: bool,
}

impl Bus {
    pub fn new() -> Bus {
        let mut bus = Bus {
            ppu: Ppu2C02::new(),
            cpu: None,
            cartridge: None,
            ram: [0; 2048],
            pad1: Pad::new(),
            pad2: Pad::new(),
            system_clock_counter: 0,
            dma_addr: 0,
            dma_data: 0,
            dma_page: 0,
            dma_dummy: true,
            dma_transfer: false,
        };

        let cpu = Cpu6502::new(&mut bus);
        bus.cpu = Some(cpu);

        bus
    }

    pub fn read(&mut self, addres: u16, read_only: bool) -> u8 {
        if let Some(cartridge) = &self.cartridge {
            let (read, data) = cartridge.cpu_read(addres);

            if read {
                return data;
            }
        }

        // Ram
        if addres <= 0x1FFF {
            return self.ram[addres as usize & 0x07FF];
        }

        if addres >= 0x2000 && addres <= 0x3FFF {
            // PPU Address range, mirrored every 8
            return self.ppu.cpu_read(addres & 0x0007, read_only);
        }

        // Pads
        if addres >= 0x4016 && addres <= 0x4017 {
            return match addres {
                0x4016 => self.pad1.get_reg(),
                0x4017 => self.pad2.get_reg(),
                _ => 0,
            };
        }

        0
    }

    pub fn write(&mut self, addres: u16, data: u8) {
        if let Some(cartridge) = &mut self.cartridge {
            if cartridge.cpu_write(addres, data) {
                return;
            }
        }
        if addres <= 0x1FFF {
            self.ram[addres as usize & 0x07FF] = data;
            return;
        }

        if addres >= 0x2000 && addres <= 0x3FFF {
            // PPU Address range. The PPU only has 8 primary registers
            // and these are repeated throughout this range. We can
            // use bitwise AND operation to mask the bottom 3 bits,
            // which is the equivalent of addr % 8.
            self.ppu.cpu_write(addres & 0x0007, data);
            return;
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
        if let Some(cart) = &mut self.cartridge {
            self.ppu.connect_cartridge(cart);
        }
    }

    pub fn clock(&mut self) {
        // Clocking. The heart and soul of an emulator. The running
        // frequency is controlled by whatever calls this function.
        // So here we "divide" the clock as necessary and call
        // the peripheral devices clock() function at the correct
        // times.

        // The fastest clock frequency the digital system cares
        // about is equivalent to the PPU clock. So the PPU is clocked
        // each time this function is called...
        self.ppu.clock();

        // The CPU runs 3 times slower than the PPU so we only call its
        // clock() function every 3 times this function is called. We
        // have a global counter to keep track of this.
        if self.system_clock_counter % 3 == 0 {
            // Is the system performing a DMA transfer form CPU memory to
            // OAM memory on PPU?...
            if self.dma_transfer {
                // ...Yes! We need to wait until the next even CPU clock cycle
                // before it starts...
                if self.dma_dummy {
                    // ...So hang around in here each clock until 1 or 2 cycles
                    // have elapsed...
                    if self.system_clock_counter % 2 == 1 {
                        // ...and finally allow DMA to start
                        self.dma_dummy = false;
                    }
                } else {
                    // DMA can take place!
                    if self.system_clock_counter % 2 == 0 {
                        // On even clock cycles, read from CPU bus
                        self.dma_data =
                            self.read((self.dma_page as u16) << 8 | self.dma_addr as u16, false);
                    } else {
                        // On odd clock cycles, write to PPU OAM
                        self.ppu.oam_write(self.dma_addr, self.dma_data);
                        // Increment the lo byte of the address
                        self.dma_addr += 1;
                        // If this wraps around, we know that 256
                        // bytes have been written, so end the DMA
                        // transfer, and proceed as normal
                        if self.dma_addr == 0x00 {
                            self.dma_transfer = false;
                            self.dma_dummy = true;
                        }
                    }
                }
            } else {
                // No DMA happening, the CPU is in control of its
                // own destiny. Go forth my friend and calculate
                // awesomeness for many generations to come...
                if let Some(cpu) = &mut self.cpu {
                    cpu.clock();
                }
            }
        }

        // The PPU is capable of emitting an interrupt to indicate the
        // vertical blanking period has been entered. If it has, we need
        // to send that irq to the CPU.
        if self.ppu.nmi {
            self.ppu.nmi = false;
            if let Some(cpu) = &mut self.cpu {
                cpu.nmi();
            }
        }

        self.system_clock_counter += 1;
    }
}
