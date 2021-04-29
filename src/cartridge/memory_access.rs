use super::{Cartridge, ChrRom, PrgRom};

impl PrgRom {
    pub fn read(&mut self, addr: u16) -> (bool, u8) {
        let (result, mapped_addr) = self.mapper.cpu_map_read(addr);

        if result {
            return (true, self.prg_memory[mapped_addr as usize]);
        };

        return (false, 0);
    }

    pub fn write(&mut self, addr: u16, data: u8) -> bool {
        let (result, mapped_addr) = self.mapper.cpu_map_write(addr, data);

        if result {
            self.prg_memory[mapped_addr as usize] = data;
            return true;
        };

        return false;
    }

    pub fn reset(&self) {
        self.mapper.reset();
    }
}

impl ChrRom {
    pub fn read(&mut self, addr: u16) -> (bool, u8) {
        let (result, mapped_addr) = self.mapper.ppu_map_read(addr);

        if result {
            return (true, self.chr_memory[mapped_addr as usize]);
        };

        return (false, 0);
    }

    pub fn write(&mut self, addr: u16, data: u8) -> bool {
        let (result, mapped_addr) = self.mapper.ppu_map_write(addr);

        if result {
            self.chr_memory[mapped_addr as usize] = data;
            return true;
        };

        return false;
    }

    pub fn reset(&self) {
        self.mapper.reset();
    }
}

impl Cartridge {
    pub fn get_roms(&self) -> (PrgRom, ChrRom) {
        let prg_rom = PrgRom {
            mapper: self.mapper,
            prg_banks: self.prg_banks,
            prg_memory: self.prg_memory.clone(),
            mapper_id: self.mapper_id,
            mirror: self.mirror,
        };

        let chr_rom = ChrRom {
            mapper: self.mapper,
            chr_banks: self.chr_banks,
            chr_memory: self.chr_memory.clone(),
            mapper_id: self.mapper_id,
            mirror: self.mirror,
        };

        (prg_rom, chr_rom)
    }
}
