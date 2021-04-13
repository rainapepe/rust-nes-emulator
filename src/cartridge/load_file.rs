use super::Cartridge;

// iNES Format Header
pub struct Header {
    name: [char; 4],

    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [char; 5],
}

impl Cartridge {
    fn load_file(&self, file_name: String) {}
}
