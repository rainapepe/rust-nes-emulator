use super::{Cartridge, Header, Mirror};
use crate::mapper::Mapper;

use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem::{forget, size_of, MaybeUninit};
use std::slice;
use std::{fs::File, usize};

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

    pub fn empty() -> Cartridge {
        let mut cart = Cartridge {
            image_valid: false,
            mirror: Mirror::Horizontal,
            mapper_id: 0,
            prg_banks: 1,
            chr_banks: 0,
            prg_memory: vec![],
            chr_memory: vec![],
            mapper: Mapper::create_mapper_000(1, 0),
        };

        cart.prg_memory.resize(16384, 0);
        cart.chr_memory.resize(8192, 0);

        cart
    }
}

/**
 * Função que le uma struct de um arquivo
 * T => struct que será usada para ler o arquivo
 */
pub fn read_struct<T, R: Read>(read: &mut R) -> std::io::Result<T> {
    // calculando o tamanho da struct em bytes
    let num_bytes = size_of::<T>();
    unsafe {
        // Aqui vamos criar uma instancia da nossa struct não inicializada
        // para fazer isso precisamos utilizar o escopo unsafe
        let mut data = MaybeUninit::<T>::uninit().assume_init();
        // Vamos criar um buffer dividindo nossa struct em um array de bytes
        // esse buffer é um espelho da nossa instancia, ou seja, eles compartilham o mesmo endereço na memória
        let mut buffer = slice::from_raw_parts_mut(&mut data as *mut T as *mut u8, num_bytes);

        // Depois de ler os bytes do arquivo para nosso buffer a nossa variavel data estava com os dados corretos
        match read.read_exact(buffer) {
            Ok(()) => Ok(data),
            Err(e) => {
                forget(data);
                Err(e)
            }
        }
    }
}

pub fn print_buffer_hex(buffer: &Vec<u8>, size: usize) {
    let len = size / 16;
    for i in 0..len {
        let offset = i * 16;
        println!("[{:#06x}]: {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x}", 
        offset, buffer[offset], buffer[offset + 1], buffer[offset + 2], buffer[offset + 3], buffer[offset + 4], buffer[offset + 5], buffer[offset + 6], buffer[offset + 7], buffer[offset + 8], buffer[offset + 9], buffer[offset + 10], buffer[offset + 11], buffer[offset + 12], buffer[offset + 13], buffer[offset + 14], buffer[offset + 15]);
    }
}

// Função para ler um vetor de bytes do arquivo
pub fn read_vec(reader: &mut BufReader<File>, size: usize) -> std::io::Result<Vec<u8>> {
    // Criando vetor e configurando seu tamanho
    let mut data: Vec<u8> = vec![];
    data.resize(size, 0);

    // Calculando o numero de offsets, pois vamos ler 16 bytes de cada vez
    let offsets = size / 16;
    for y in 0..offsets {
        let offset = y * 16;
        let mut buffer: [u8; 16] = [0; 16];

        // lendo o arquivo e salvando em um buffer
        reader.read(&mut buffer)?;

        // Passar os dados do buffer para nosso vetor
        for i in 0..16 {
            data[offset + i] = buffer[i];
        }
    }

    Ok(data)
}

impl Cartridge {
    pub fn load_file(&mut self, file_name: String) -> std::io::Result<()> {
        let file = File::open(file_name).unwrap();
        let mut reader = BufReader::new(file);

        // Ler o header do arquivo
        let header = read_struct::<Header, BufReader<File>>(&mut reader)?;

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
                // numero de chunks para armazenar códigos (instruções)
                self.prg_banks = header.prg_rom_chunks;
                // lendo todos os bytes de instruções com base no número de chunks
                self.prg_memory = read_vec(&mut reader, (self.prg_banks as usize) * 16384)?;

                // lendo quantos chunks para armazenar sprites
                // nesse caso sempre que for 0 o tamanho do chunk para armazenar os bytes será 8192
                self.chr_banks = header.chr_rom_chunks;
                let chr_memory_size: usize = if self.chr_banks == 0 {
                    // Criando o CHR RAM
                    8192
                } else {
                    // Alocando para ROM
                    (self.chr_banks as usize) * 8192
                };
                // lendo todos os bytes de sprites
                self.chr_memory = read_vec(&mut reader, chr_memory_size)?;
            }
            _ => {}
        };

        // carregando o mapper correto
        match self.mapper_id {
            0 => self.mapper = Mapper::create_mapper_000(self.prg_banks, self.chr_banks),
            _ => {}
        };

        self.image_valid = true;

        // print_buffer_hex(&self.prg_memory, 1 * 16384);

        Ok(())
    }
}
