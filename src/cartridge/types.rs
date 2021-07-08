use crate::mapper::Mapper;

#[derive(Clone, Copy)]
pub enum Mirror {
    Horizontal,
    Vertical,
    OneScreenLo,
    OneScreenHi,
}

// Program Rom
// Estrutura que utilizamos para guardar todas as instruções do jogo
pub struct PrgRom {
    pub mirror: Mirror,
    pub mapper_id: u8,
    pub prg_memory: Vec<u8>,
    pub prg_banks: u8,
    pub mapper: Mapper,
}

// Character Rom
// Estrutura que utilizamos para guardar todos os sprites do jogo
pub struct ChrRom {
    pub mirror: Mirror,
    pub mapper_id: u8,
    pub chr_banks: u8,
    pub chr_memory: Vec<u8>,
    pub mapper: Mapper,
}

pub struct Cartridge {
    pub image_valid: bool,
    pub mirror: Mirror,

    pub mapper_id: u8,
    /* Número de bancos de código */
    pub prg_banks: u8,
    /* Número de bancos de sprites */
    pub chr_banks: u8,

    /* Armazenamento do código lido do arquivo */
    pub prg_memory: Vec<u8>,
    /* Armazenamento dos sprites lidos do arquivo */
    pub chr_memory: Vec<u8>,

    /* Tipo de mapper do cartucho */
    pub mapper: Mapper,
}

// iNES Format Header
// Essa é a estrutura do Header de todo jogo nes
#[derive(Debug)]
pub struct Header {
    pub name: [u8; 4],
    pub prg_rom_chunks: u8,
    pub chr_rom_chunks: u8,
    pub mapper1: u8,
    pub mapper2: u8,
    pub prg_ram_size: u8,
    pub tv_system1: u8,
    pub tv_system2: u8,
    pub unused: [u8; 5],
}
