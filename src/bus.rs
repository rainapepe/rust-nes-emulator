use crate::cpu::Cpu6502;
use crate::{cartridge::Cartridge, ppu::Ppu2C02};

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
             |                  |     PATTERN     |        |      NAMETABLE       |     |        PALETTES         |
             |                  |_________________|        |______________________|     |_________________________|
     ________|__________         0x0000  |  0x1FFF          0x2000     |    0x2FFF       0x3F00      |      0x3FFF
    |                   |                |                             |                             |
    |       PPU         |----------------+-----------------------------+-----------------------------+-------------------------------- BUS
    |___________________|

    OBS:
    - barramento de 16bits
    - memória Ram tem 8kb porém só 2kb são utilizados

*/

pub struct Bus {
    pub ppu: Ppu2C02,
    pub cpu: Option<Cpu6502>,
    pub cartridge: Option<Cartridge>,
    ram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Bus {
        let mut bus = Bus {
            ppu: Ppu2C02::new(),
            cpu: None,
            cartridge: None,
            ram: [0; 2048],
        };

        let cpu = Cpu6502::new(&mut bus);
        bus.cpu = Some(cpu);

        bus
    }

    pub fn read(&self, addres: u16, read_only: bool) -> u8 {
        if let Some(cartridge) = &self.cartridge {
            let (read, data) = cartridge.cpu_read(addres);

            if read {
                return data;
            }
        }
        if addres <= 0x1FFF {
            return self.ram[addres as usize & 0x07FF];
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
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
        if let Some(cart) = &mut self.cartridge {
            self.ppu.connect_cartridge(cart);
        }
    }
}
