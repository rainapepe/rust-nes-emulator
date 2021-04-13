pub trait Mapper {
    // fn new(prg_banks: u8, chr_banks: u8) -> Self;

    // Transform CPU bus address into PRG ROM offset
    fn cpu_map_read(&self, addr: u16) -> (bool, u32);
    fn cpu_map_write(&self, addr: u16, data: u8) -> (bool, u32);

    // Transform PPU bus address into CHR ROM offset
    fn ppu_map_read(&self, addr: u16) -> (bool, u32);
    fn ppu_map_write(&self, addr: u16) -> (bool, u32);

    fn reset(&self);
}
