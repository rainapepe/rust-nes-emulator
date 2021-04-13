use crate::mapper::{Mapper, Mapper000};

pub enum Mirror {
    Horizontal,
    Vertical,
    OneScreenLo,
    OneScreenHi,
}

pub struct Cartridge {
    image_valid: bool,
    mirror: Mirror,

    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,

    prg_memory: Vec<u8>,
    chr_memory: Vec<u8>,

    mapper: Box<dyn Mapper + 'static>,
}

impl Cartridge {
    pub fn new(file_name: String) -> Cartridge {
        let map = Mapper000 {
            chr_banks: 0,
            prg_banks: 0,
        };

        Cartridge {
            image_valid: false,
            mirror: Mirror::Horizontal,
            mapper_id: 0,
            prg_banks: 0,
            chr_banks: 0,
            prg_memory: vec![],
            chr_memory: vec![],
            mapper: Box::new(map),
        }
    }
}
