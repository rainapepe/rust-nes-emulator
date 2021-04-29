use super::super::Mapper;

pub fn cpu_map_read(mapper: &mut Mapper, addr: u16) -> (bool, u32) {
    // if PRGROM is 16KB
    //     CPU Address Bus          PRG ROM
    //     0x8000 -> 0xBFFF: Map    0x0000 -> 0x3FFF
    //     0xC000 -> 0xFFFF: Mirror 0x0000 -> 0x3FFF
    // if PRGROM is 32KB
    //     CPU Address Bus          PRG ROM
    //     0x8000 -> 0xFFFF: Map    0x0000 -> 0x7FFF
    // println!("mapper->read({:#06x})", addr);

    if addr >= 0x8000 && addr <= 0xFFFF {
        return (
            true,
            (addr & (if mapper.prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32,
        );
    }

    return (false, 0);
}

pub fn cpu_map_write(mapper: &mut Mapper, addr: u16, data: u8) -> (bool, u32) {
    if addr >= 0x8000 && addr <= 0xFFFF {
        return (
            true,
            (addr & (if mapper.prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32,
        );
    }

    return (false, 0);
}

pub fn ppu_map_read(mapper: &mut Mapper, addr: u16) -> (bool, u32) {
    // There is no mapping required for PPU
    // PPU Address Bus          CHR ROM
    // 0x0000 -> 0x1FFF: Map    0x0000 -> 0x1FFF
    if addr >= 0 && addr <= 0x1FFF {
        return (true, addr as u32);
    }

    return (false, 0);
}

pub fn ppu_map_write(mapper: &mut Mapper, addr: u16) -> (bool, u32) {
    if addr >= 0x0000 && addr <= 0x1FFF {
        if mapper.chr_banks == 0 {
            // Treat as RAM
            return (true, addr as u32);
        }
    }

    return (false, 0);
}
