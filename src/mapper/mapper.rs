use super::mappers;

pub enum MapperType {
    Mapper000,
}

pub struct Mapper {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub mapper_type: MapperType,
}

impl Mapper {
    // Transform CPU bus address into PRG ROM offset
    pub fn cpu_map_read(&mut self, addr: u16) -> (bool, u32) {
        match self.mapper_type {
            MapperType::Mapper000 => mappers::cpu_map_read(self, addr),
        }
    }

    pub fn cpu_map_write(&mut self, addr: u16, data: u8) -> (bool, u32) {
        match self.mapper_type {
            MapperType::Mapper000 => mappers::cpu_map_write(self, addr, data),
        }
    }

    // Transform PPU bus address into CHR ROM offset
    pub fn ppu_map_read(&mut self, addr: u16) -> (bool, u32) {
        match self.mapper_type {
            MapperType::Mapper000 => mappers::ppu_map_read(self, addr),
        }
    }
    pub fn ppu_map_write(&mut self, addr: u16) -> (bool, u32) {
        match self.mapper_type {
            MapperType::Mapper000 => mappers::ppu_map_write(self, addr),
        }
    }

    pub fn reset(&self) {}
}

impl Mapper {
    pub fn create_mapper_000(prg_banks: u8, chr_banks: u8) -> Mapper {
        Mapper {
            prg_banks,
            chr_banks,
            mapper_type: MapperType::Mapper000,
        }
    }

    pub fn get_type(&self) -> &'static str {
        match self.mapper_type {
            MapperType::Mapper000 => "Mapper000",
        }
    }
}
