fn get_bits_u8(value: u8, position: u8, bits: u8) -> u8 {
    (value >> position) & (0xFF >> (8 - bits))
}

fn set_bits_u8(value: u8, position: u8, bits: u8, data: u8) -> u8 {
    let sub = get_bits_u8(value, position, bits) << position;
    let add = (data & (0xFF >> (8 - bits))) << position;

    (value - sub) + add
}

fn get_bits_u16(value: u16, position: u8, bits: u8) -> u16 {
    (value >> position) & (0xFFFF >> (16 - bits))
}

fn set_bits_u16(value: u16, position: u8, bits: u8, data: u16) -> u16 {
    let sub = get_bits_u16(value, position, bits) << position;
    let add = (data & (0xFFFF >> (16 - bits))) << position;

    (value - sub) + add
}

pub struct Status {
    pub reg: u8,
}

impl Status {
    // unused: 5
    pub fn get_unused(&self) -> u8 {
        get_bits_u8(self.reg, 0, 5)
    }

    pub fn set_unused(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 0, 5, data);
    }

    // sprite_overflow: 1,
    pub fn get_sprite_overflow(&self) -> u8 {
        get_bits_u8(self.reg, 5, 1)
    }

    pub fn set_sprite_overflow(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 5, 1, data);
    }

    // sprite_zero_hit: 1,
    pub fn get_sprite_zero_hit(&self) -> u8 {
        get_bits_u8(self.reg, 6, 1)
    }

    pub fn set_sprite_zero_hit(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 6, 1, data);
    }

    // vertical_blank: 1,
    pub fn get_vertical_blank(&self) -> u8 {
        get_bits_u8(self.reg, 7, 1)
    }

    pub fn set_vertical_blank(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 7, 1, data);
    }

    pub fn new() -> Status {
        Status { reg: 0 }
    }
}

pub struct Mask {
    pub reg: u8,
}

impl Mask {
    // grayscale: 1,
    pub fn get_grayscale(&self) -> bool {
        get_bits_u8(self.reg, 0, 1) > 0
    }

    pub fn set_grayscale(&mut self, data: bool) {
        self.reg = set_bits_u8(self.reg, 0, 1, data as u8);
    }

    // render_background_left: 1,
    pub fn get_render_background_left(&self) -> bool {
        get_bits_u8(self.reg, 1, 1) > 0
    }

    pub fn set_render_background_left(&mut self, data: bool) {
        self.reg = set_bits_u8(self.reg, 1, 1, data as u8);
    }

    // render_sprites_left: 1,
    pub fn get_render_sprites_left(&self) -> bool {
        get_bits_u8(self.reg, 2, 1) > 0
    }

    pub fn set_render_sprites_left(&mut self, data: bool) {
        self.reg = set_bits_u8(self.reg, 2, 1, data as u8);
    }

    // render_background: 1,
    pub fn get_render_background(&self) -> bool {
        get_bits_u8(self.reg, 3, 1) > 0
    }

    pub fn set_render_background(&mut self, data: bool) {
        self.reg = set_bits_u8(self.reg, 3, 1, data as u8);
    }

    // render_sprites: 1,
    pub fn get_render_sprites(&self) -> bool {
        get_bits_u8(self.reg, 4, 1) > 0
    }

    pub fn set_render_sprites(&mut self, data: bool) {
        self.reg = set_bits_u8(self.reg, 4, 1, data as u8);
    }

    // enhance_red: 1,
    pub fn get_enhance_red(&self) -> bool {
        get_bits_u8(self.reg, 5, 1) > 0
    }

    pub fn set_enhance_red(&mut self, data: bool) {
        self.reg = set_bits_u8(self.reg, 5, 1, data as u8);
    }

    // enhance_green: 1,
    pub fn get_enhance_green(&self) -> bool {
        get_bits_u8(self.reg, 6, 1) > 0
    }

    pub fn set_enhance_green(&mut self, data: bool) {
        self.reg = set_bits_u8(self.reg, 6, 1, data as u8);
    }

    // enhance_blue: 1,
    pub fn get_enhance_blue(&self) -> bool {
        get_bits_u8(self.reg, 7, 1) > 0
    }

    pub fn set_enhance_blue(&mut self, data: bool) {
        self.reg = set_bits_u8(self.reg, 7, 1, data as u8);
    }

    pub fn new() -> Mask {
        Mask { reg: 0 }
    }
}

pub struct PpuControl {
    pub reg: u8,
}

impl PpuControl {
    // nametable_x: 1,
    pub fn get_nametable_x(&self) -> u8 {
        get_bits_u8(self.reg, 0, 1)
    }

    pub fn set_nametable_x(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 0, 1, data);
    }

    // nametable_y: 1,
    pub fn get_nametable_y(&self) -> u8 {
        get_bits_u8(self.reg, 1, 1)
    }

    pub fn set_nametable_y(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 1, 1, data);
    }

    // increment_mode: 1,
    pub fn get_increment_mode(&self) -> u8 {
        get_bits_u8(self.reg, 2, 1)
    }

    pub fn set_increment_mode(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 2, 1, data);
    }

    // pattern_sprite: 1,
    pub fn get_pattern_sprite(&self) -> u8 {
        get_bits_u8(self.reg, 3, 1)
    }

    pub fn set_pattern_sprite(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 3, 1, data);
    }

    // pattern_background: 1,
    pub fn get_pattern_background(&self) -> u8 {
        get_bits_u8(self.reg, 4, 1)
    }

    pub fn set_pattern_background(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 4, 1, data);
    }

    // sprite_size: 1,
    pub fn get_sprite_size(&self) -> u8 {
        get_bits_u8(self.reg, 5, 1)
    }

    pub fn set_sprite_size(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 5, 1, data);
    }

    // slave_mode: 1 // unused,
    pub fn get_slave_mode(&self) -> u8 {
        get_bits_u8(self.reg, 6, 1)
    }

    pub fn set_slave_mode(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 6, 1, data);
    }

    // enable_nmi: 1,
    pub fn get_enable_nmi(&self) -> u8 {
        get_bits_u8(self.reg, 7, 1)
    }

    pub fn set_enable_nmi(&mut self, data: u8) {
        self.reg = set_bits_u8(self.reg, 7, 1, data);
    }

    pub fn new() -> PpuControl {
        PpuControl { reg: 0 }
    }
}

#[derive(Clone, Copy)]
pub struct LoopyRegister {
    pub reg: u16,
}

impl LoopyRegister {
    /* coarse_x: 5 bits */
    pub fn get_coarse_x(&self) -> u8 {
        get_bits_u16(self.reg, 0, 5) as u8
    }

    /* coarse_x: 5 bits */
    pub fn set_coarse_x(&mut self, data: u8) {
        self.reg = set_bits_u16(self.reg, 0, 5, data as u16);
    }

    /* coarse_y: 5 bits */
    pub fn get_coarse_y(&self) -> u8 {
        get_bits_u16(self.reg, 5, 5) as u8
    }

    /* coarse_y: 5 bits */
    pub fn set_coarse_y(&mut self, data: u8) {
        self.reg = set_bits_u16(self.reg, 5, 5, data as u16);
    }

    // nametable_x: 1,
    pub fn get_nametable_x(&self) -> u8 {
        get_bits_u16(self.reg, 10, 1) as u8
    }

    pub fn set_nametable_x(&mut self, data: u8) {
        self.reg = set_bits_u16(self.reg, 10, 1, data as u16);
    }

    // nametable_y: 1,
    pub fn get_nametable_y(&self) -> u8 {
        get_bits_u16(self.reg, 11, 1) as u8
    }

    pub fn set_nametable_y(&mut self, data: u8) {
        self.reg = set_bits_u16(self.reg, 11, 1, data as u16);
    }

    /* fine_y: 3 bits */
    pub fn get_fine_y(&self) -> u8 {
        get_bits_u16(self.reg, 12, 3) as u8
    }

    /* fine_y: 3 bits */
    pub fn set_fine_y(&mut self, data: u8) {
        self.reg = set_bits_u16(self.reg, 12, 3, data as u16);
    }

    // unused: 1,
    pub fn get_unused(&self) -> u8 {
        get_bits_u16(self.reg, 15, 1) as u8
    }

    pub fn set_unused(&mut self, data: u8) {
        self.reg = set_bits_u16(self.reg, 15, 1, data as u16);
    }

    pub fn new() -> LoopyRegister {
        LoopyRegister { reg: 0 }
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
