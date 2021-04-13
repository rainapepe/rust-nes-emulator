use crate::cpu::Cpu6502;

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
    // ppu,
    pub cpu: Option<Cpu6502>,
    ram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Bus {
        let mut bus = Bus {
            cpu: None,
            ram: [0; 2048],
        };

        let cpu = Cpu6502::new(&mut bus);
        bus.cpu = Some(cpu);

        bus
    }

    pub fn read(&self, addres: u16, b_read_only: bool) -> u8 {
        if addres <= 0x1FFF {
            return self.ram[addres as usize & 0x07FF];
        }

        0
    }

    pub fn write(&mut self, addres: u16, data: u8) {
        if addres <= 0x1FFF {
            self.ram[addres as usize & 0x07FF] = data;
        }
    }
}
