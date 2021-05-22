use crate::{
    cartridge::{Cartridge, PrgRom},
    ppu::Ppu2C02,
};
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
    pub version: &'static str,
    pub ppu: Ppu2C02,
    pub prg_rom: PrgRom,
    pub pad1: Pad,
    pub pad2: Pad,
    pub ram: [u8; 2048],
    // A count of how many clocks have passed
    pub system_clock_counter: u32,

    // A simple form of Direct Memory Access is used to swiftly
    // transfer data from CPU bus memory into the OAM memory. It would
    // take too long to sensibly do this manually using a CPU loop, so
    // the program prepares a page of memory with the sprite info required
    // for the next frame and initiates a DMA transfer. This suspends the
    // CPU momentarily while the PPU gets sent data at PPU clock speeds.
    // Note here, that dma_page and dma_addr form a 16-bit address in
    // the CPU bus address space
    pub dma_page: u8,
    pub dma_addr: u8,
    pub dma_data: u8,

    // DMA transfers need to be timed accurately. In principle it takes
    // 512 cycles to read and write the 256 bytes of the OAM memory, a
    // read followed by a write. However, the CPU needs to be on an "even"
    // clock cycle, so a dummy cycle of idleness may be required
    pub dma_dummy: bool,

    // Finally a flag to indicate that a DMA transfer is happening
    pub dma_transfer: bool,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Bus {
        let (prg_rom, chr_rom) = cartridge.get_roms();
        Bus {
            version: "v1",
            ppu: Ppu2C02::new(chr_rom),
            prg_rom,
            ram: [0; 2048],
            pad1: Pad::new(),
            pad2: Pad::new(),
            system_clock_counter: 0,
            dma_addr: 0,
            dma_data: 0,
            dma_page: 0,
            dma_dummy: true,
            dma_transfer: false,
        }
    }

    pub fn read(&mut self, addres: u16, read_only: bool) -> u8 {
        // println!("bus->read({:#06x})", addres);
        // println!("bus->read: {}", self.version);

        let (read, data) = self.prg_rom.read(addres);
        // println!("bus->read({:#06x}) - cart is true: {}", addres, read);
        if read {
            return data;
        }

        // Ram
        if addres <= 0x1FFF {
            // println!("bus->read({:#06x}) - ram", addres);
            return self.ram[addres as usize & 0x07FF];
        }

        if addres >= 0x2000 && addres <= 0x3FFF {
            // println!("bus->read({:#06x}) - ppu", addres);
            // PPU Address range, mirrored every 8
            return self.ppu.cpu_read(addres & 0x0007, read_only);
        }

        // Pads
        if addres >= 0x4016 && addres <= 0x4017 {
            return match addres {
                0x4016 => self.pad1.read(),
                0x4017 => self.pad2.read(),
                _ => 0,
            };
        }

        0
    }

    pub fn write(&mut self, addres: u16, data: u8) {
        if self.prg_rom.write(addres, data) {
            return;
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

        if addres == 0x4014 {
            // A write to this address initiates a DMA transfer
            self.dma_page = data;
            self.dma_addr = 0x00;
            self.dma_transfer = true;
        }

        // Pads
        if addres >= 0x4016 && addres <= 0x4017 {
            return match addres {
                0x4016 => self.pad1.write(data > 0),
                0x4017 => self.pad2.write(data > 0),
                _ => {}
            };
        }
    }
}
