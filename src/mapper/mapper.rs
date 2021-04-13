pub struct Mapper {
    n_prg_banks: u8,
    n_chr_banks: u8,
}

impl Mapper {
    pub fn new(prg_banks: u8, chr_banks: u8) -> Mapper {
        let mapper = Mapper {
            n_prg_banks: prg_banks,
            n_chr_banks: chr_banks,
        };

        mapper.reset();
        mapper
    }

    pub fn reset(&self) {}
}
