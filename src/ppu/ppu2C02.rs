


struct Pixel {}

struct Sprite {}

pub struct Ppu2C02 {
    table_name: [[u8; 1024]; 2],
    table_pattern: [[u8; 4096]; 2],
    table_palette: [u8; 32],

    // olc::Pixel  palScreen[0x40];
	// olc::Sprite sprScreen          =   olc::Sprite(256, 240);
	// olc::Sprite sprNameTable[2]    = { olc::Sprite(256, 240), olc::Sprite(256, 240) };
	// olc::Sprite sprPatternTable[2] = { olc::Sprite(128, 128), olc::Sprite(128, 128) };
    // Test
}


impl Ppu2C02 {

    pub fn new() -> Ppu2C02 {
        Ppu2C02 {
            table_name: [[0; 1024]; 2],
        }
    }
}
