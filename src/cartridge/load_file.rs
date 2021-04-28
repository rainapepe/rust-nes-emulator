use super::{Cartridge, Header, Mirror};
use crate::mapper::Mapper;

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem::{forget, size_of, MaybeUninit};
use std::slice;

impl Cartridge {
    pub fn new(file_name: String) -> Cartridge {
        let mut cart = Cartridge {
            image_valid: false,
            mirror: Mirror::Horizontal,
            mapper_id: 0,
            prg_banks: 0,
            chr_banks: 0,
            prg_memory: vec![],
            chr_memory: vec![],
            mapper: Mapper::create_mapper_000(0, 0),
        };

        cart.load_file(file_name).unwrap_or_else(|err| {
            panic!("Failed to load file: {:?}", err);
        });

        cart
    }
}

pub fn read_struct<T, R: Read>(read: &mut R) -> std::io::Result<T> {
    let num_bytes = size_of::<T>();
    unsafe {
        let mut data2 = MaybeUninit::<T>::uninit().assume_init();
        let mut buffer = slice::from_raw_parts_mut(&mut data2 as *mut T as *mut u8, num_bytes);

        match read.read_exact(buffer) {
            Ok(()) => Ok(data2),
            Err(e) => {
                forget(data2);
                Err(e)
            }
        }
    }
}

impl Cartridge {
    pub fn load_file(&mut self, file_name: String) -> std::io::Result<()> {
        let file = File::open(file_name).unwrap();
        let mut reader = BufReader::new(file);

        // Ler o header do arquivo
        let header = read_struct::<Header, BufReader<File>>(&mut reader).unwrap();
        println!("header: {:?}", header);
        // Se existe um "trainer" vamos reposicionar o stream para lê-lo
        if (header.mapper1 & 0x04) > 0 {
            reader.seek(SeekFrom::Current(512))?;
        }

        // Determinar o Mapper Id
        self.mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
        self.mirror = if header.mapper1 & 0x01 > 0 {
            Mirror::Vertical
        } else {
            Mirror::Horizontal
        };

        let file_type: u8 = 1;

        match file_type {
            1 => {
                self.prg_banks = header.prg_rom_chunks;
                self.prg_memory
                    .resize(((self.prg_banks as u32) * 16384) as usize, 0);
                reader.read(&mut self.prg_memory)?;

                self.chr_banks = header.chr_rom_chunks;

                if self.chr_banks == 0 {
                    // Criando o CHR RAM
                    self.chr_memory.resize(8192 as usize, 0);
                } else {
                    // Alocando para ROM
                    self.chr_memory
                        .resize(((self.chr_banks as u32) * 8192) as usize, 0);
                }

                reader.read(&mut self.chr_memory)?;
            }
            _ => {}
        };

        match self.mapper_id {
            0 => self.mapper = Mapper::create_mapper_000(self.prg_banks, self.chr_banks),
            _ => {}
        };

        self.image_valid = true;

        println!("cartridge.prg_memory {:?}", self.prg_memory.len());
        println!("cartridge.chr_memory {:?}", self.chr_memory.len());

        println!("load mapper {}", self.mapper.get_type());

        Ok(())
    }
}
