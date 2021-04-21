#[derive(Clone, Copy)]
pub struct Pixel(u8, u8, u8);

impl Pixel {
    pub fn red(&self) -> u8 {
        self.0
    }

    pub fn green(&self) -> u8 {
        self.1
    }

    pub fn blue(&self) -> u8 {
        self.2
    }
}

#[derive(Clone, Copy)]
pub struct Sprite {
    width: u32,
    height: u32,
} // Matrix de pixels linhasxcolunas

impl Sprite {
    pub fn new(width: u32, height: u32) -> Sprite {
        Sprite { width, height }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {}
}

// Paleta de cores que o NES possui, são as 64 cores que o nes possui
// confira: http://wiki.nesdev.com/w/index.php/PPU_palettes/zh
pub const PALETTE_SCREEN: [Pixel; 0x40] = [
    // 0x00
    Pixel(84, 84, 84), // 0x00
    Pixel(0, 30, 116), // 0x01
    Pixel(8, 16, 144), // 0x02
    Pixel(48, 0, 136), // 0x03
    Pixel(68, 0, 100), // 0x04
    Pixel(92, 0, 48),  // 0x05
    Pixel(84, 4, 0),   // 0x06
    Pixel(60, 24, 0),  // 0x07
    Pixel(32, 42, 0),  // 0x08
    Pixel(8, 58, 0),   // 0x09
    Pixel(0, 64, 0),   // 0x0A
    Pixel(0, 60, 0),   // 0x0B
    Pixel(0, 50, 60),  // 0x0C
    Pixel(0, 0, 0),    // 0x0D
    Pixel(0, 0, 0),    // 0x0E
    Pixel(0, 0, 0),    // 0x0F
    // 0x10
    Pixel(152, 150, 152), // 0x10
    Pixel(8, 76, 196),    // 0x11
    Pixel(48, 50, 236),   // 0x12
    Pixel(92, 30, 228),   // 0x13
    Pixel(136, 20, 176),  // 0x14
    Pixel(160, 20, 100),  // 0x15
    Pixel(152, 34, 32),   // 0x16
    Pixel(120, 60, 0),    // 0x17
    Pixel(84, 90, 0),     // 0x18
    Pixel(40, 114, 0),    // 0x19
    Pixel(8, 124, 0),     // 0x1A
    Pixel(0, 118, 40),    // 0x1B
    Pixel(0, 102, 120),   // 0x1C
    Pixel(0, 0, 0),       // 0x1D
    Pixel(0, 0, 0),       // 0x1E
    Pixel(0, 0, 0),       // 0x1F
    // 0x20
    Pixel(236, 238, 236), // 0x20
    Pixel(76, 154, 236),  // 0x21
    Pixel(120, 124, 236), // 0x22
    Pixel(176, 98, 236),  // 0x23
    Pixel(228, 84, 236),  // 0x24
    Pixel(236, 88, 180),  // 0x25
    Pixel(236, 106, 100), // 0x26
    Pixel(212, 136, 32),  // 0x27
    Pixel(160, 170, 0),   // 0x28
    Pixel(116, 196, 0),   // 0x29
    Pixel(76, 208, 32),   // 0x2A
    Pixel(56, 204, 108),  // 0x2B
    Pixel(56, 180, 204),  // 0x2C
    Pixel(60, 60, 60),    // 0x2D
    Pixel(0, 0, 0),       // 0x2E
    Pixel(0, 0, 0),       // 0x2F
    // 0x30
    Pixel(236, 238, 236), // 0x30
    Pixel(168, 204, 236), // 0x31
    Pixel(188, 188, 236), // 0x32
    Pixel(212, 178, 236), // 0x33
    Pixel(236, 174, 236), // 0x34
    Pixel(236, 174, 212), // 0x35
    Pixel(236, 180, 176), // 0x36
    Pixel(228, 196, 144), // 0x37
    Pixel(204, 210, 120), // 0x38
    Pixel(180, 222, 120), // 0x39
    Pixel(168, 226, 144), // 0x3A
    Pixel(152, 226, 180), // 0x3B
    Pixel(160, 214, 228), // 0x3C
    Pixel(160, 162, 160), // 0x3D
    Pixel(0, 0, 0),       // 0x3E
    Pixel(0, 0, 0),       // 0x3F
];

pub fn get_color(color: u8) -> Pixel {
    PALETTE_SCREEN[(color & 0x3F) as usize]
}
