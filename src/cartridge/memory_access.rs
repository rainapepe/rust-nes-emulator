use super::Cartridge;

impl Cartridge {
    pub fn cpu_read(&self, addr: u16) -> (bool, u8) {
        let (result, mapped_addr) = self.mapper.cpu_map_read(addr);

        if result {
            return (true, self.prg_memory[mapped_addr as usize]);
        };

        return (false, 0);
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        let (result, mapped_addr) = self.mapper.cpu_map_write(addr, data);

        if result {
            self.prg_memory[mapped_addr as usize] = data;
            return true;
        };

        return false;
    }

    pub fn ppu_read(&self, addr: u16) -> (bool, u8) {
        let (result, mapped_addr) = self.mapper.ppu_map_read(addr);

        if result {
            return (true, self.chr_memory[mapped_addr as usize]);
        };

        return (false, 0);
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
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
