pub struct Status {
    pub unused: u8,
    pub sprite_overflow: u8,
    pub sprite_zero_hit: u8,
    pub vertical_blank: u8,
}

impl Status {
    pub fn new() -> Status {
        Status {
            unused: 5,
            sprite_overflow: 1,
            sprite_zero_hit: 1,
            vertical_blank: 1,
        }
    }

    pub fn get_reg(&self) -> u8 {
        self.unused
    }

    pub fn set_reg(&mut self, value: u8) {
        self.unused = value;
    }
}

pub struct Mask {
    pub grayscale: u8,
    pub render_background_left: u8,
    pub render_sprites_left: u8,
    pub render_background: u8,
    pub render_sprites: u8,
    pub enhance_red: u8,
    pub enhance_green: u8,
    pub enhance_blue: u8,
}

impl Mask {
    pub fn new() -> Mask {
        Mask {
            grayscale: 1,
            render_background_left: 1,
            render_sprites_left: 1,
            render_background: 1,
            render_sprites: 1,
            enhance_red: 1,
            enhance_green: 1,
            enhance_blue: 1,
        }
    }

    pub fn get_reg(&self) -> u8 {
        self.grayscale
    }

    pub fn set_reg(&mut self, value: u8) {
        self.grayscale = value;
    }
}

pub struct PpuControl {
    pub nametable_x: u8,
    pub nametable_y: u8,
    pub increment_mode: u8,
    pub pattern_sprite: u8,
    pub pattern_background: u8,
    pub sprite_size: u8,
    pub slave_mode: u8, // unused
    pub enable_nmi: u8,
}

impl PpuControl {
    pub fn new() -> PpuControl {
        PpuControl {
            nametable_x: 1,
            nametable_y: 1,
            increment_mode: 1,
            pattern_sprite: 1,
            pattern_background: 1,
            sprite_size: 1,
            slave_mode: 1, // unused
            enable_nmi: 1,
        }
    }

    pub fn get_reg(&self) -> u8 {
        self.nametable_x
    }

    pub fn set_reg(&mut self, value: u8) {
        self.nametable_x = value;
    }
}

#[derive(Clone, Copy)]
pub struct LoopyRegister {
    pub coarse_x: u16,
    pub coarse_y: u16,
    pub nametable_x: u16,
    pub nametable_y: u16,
    pub fine_y: u16,
    pub unused: u16,
}

impl LoopyRegister {
    pub fn new() -> LoopyRegister {
        LoopyRegister {
            coarse_x: 5,
            coarse_y: 5,
            nametable_x: 1,
            nametable_y: 1,
            fine_y: 3,
            unused: 1,
        }
    }

    pub fn get_reg(&self) -> u16 {
        self.coarse_x
    }

    pub fn set_reg(&mut self, value: u16) {
        self.coarse_x = value;
    }
}

#[derive(Copy, Clone)]
pub struct ObjectAttributeEntry {
    pub y: u8,         // Y position of sprite
    pub id: u8,        // ID of tile from pattern memory
    pub attribute: u8, // Flags define how sprite should be rendered
    pub x: u8,         // X position of sprite
}

impl ObjectAttributeEntry {
    pub fn new() -> ObjectAttributeEntry {
        ObjectAttributeEntry {
            y: 0,
            id: 0,
            attribute: 0,
            x: 0,
        }
    }
}
