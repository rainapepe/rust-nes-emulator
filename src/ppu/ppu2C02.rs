use super::registers::{LoopyRegister, Mask, ObjectAttributeEntry, PpuControl, Status};
use super::screen::Sprite;
use crate::cartridge::Cartridge;

pub struct Ppu2C02 {
    // controla os tiles/sprites que serão exibidos na tela
    pub table_name: [[u8; 1024]; 2],
    // memória dos sprites/tiles
    pub table_pattern: [[u8; 4096]; 2],
    // paletas/cores
    pub table_palette: [u8; 32],
    pub cartridge: Option<*mut Cartridge>,

    // auxiliares
    pub sprite_screen: Sprite,             // Tela final 256x240
    pub sprite_name_table: [Sprite; 2], // (não usado) visualização da nametables (tela final na memória) as duas são 256x240
    pub sprite_pattern_table: [Sprite; 2], // Visualização da tabela de sprites (background e foregrounds)

    pub frame_complete: bool,
    pub status: Status,
    pub mask: Mask,
    pub control: PpuControl,
    pub vram_addr: LoopyRegister, // Active "pointer" address into nametable to extract background tile info
    pub tram_addr: LoopyRegister, // Temporary store of information to be "transferred" into "pointer" at various times

    // Pixel offset horizontally
    pub fine_x: u8,

    // Internal communications
    pub address_latch: u8,
    pub ppu_data_buffer: u8,

    // Pixel "dot" position information
    pub scanline: i16,
    pub cycle: i16,
    pub odd_frame: bool,

    // Background rendering =========================================
    pub bg_next_tile_id: u8,
    pub bg_next_tile_attrib: u8,
    pub bg_next_tile_lsb: u8,
    pub bg_next_tile_msb: u8,
    pub bg_shifter_pattern_lo: u16,
    pub bg_shifter_pattern_hi: u16,
    pub bg_shifter_attrib_lo: u16,
    pub bg_shifter_attrib_hi: u16,

    // Foreground "Sprite" rendering ================================
    // The OAM is an additional memory internal to the PPU. It is
    // not connected via the any bus. It stores the locations of
    // 64off 8x8 (or 8x16) tiles to be drawn on the next frame.
    pub oam: [ObjectAttributeEntry; 64],

    // A register to store the address when the CPU manually communicates
    // with OAM via PPU registers. This is not commonly used because it
    // is very slow, and instead a 256-Byte DMA transfer is used. See
    // the Bus header for a description of this.
    pub oam_addr: u8,

    pub sprite_scanline: [ObjectAttributeEntry; 8],
    pub sprite_count: u8,
    pub sprite_shifter_pattern_lo: [u8; 8],
    pub sprite_shifter_pattern_hi: [u8; 8],

    // Sprite Zero Collision Flags
    pub sprite_zero_hit_possible: bool,
    pub sprite_zero_being_rendered: bool,

    // Interface
    pub nmi: bool,
}

impl Ppu2C02 {
    pub fn new() -> Ppu2C02 {
        Ppu2C02 {
            table_name: [[0; 1024]; 2],
            table_pattern: [[0; 4096]; 2],
            table_palette: [0; 32],
            cartridge: None,
            sprite_screen: Sprite::new(256, 240),
            sprite_name_table: [Sprite::new(256, 240); 2], // unused
            sprite_pattern_table: [Sprite::new(128, 128); 2],

            frame_complete: false,

            status: Status::new(),
            mask: Mask::new(),
            control: PpuControl::new(),
            vram_addr: LoopyRegister::new(),
            tram_addr: LoopyRegister::new(),

            fine_x: 0,
            address_latch: 0,
            ppu_data_buffer: 0,
            scanline: 0,
            cycle: 0,
            odd_frame: false,

            bg_next_tile_id: 0,
            bg_next_tile_attrib: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            bg_shifter_pattern_lo: 0,
            bg_shifter_pattern_hi: 0,
            bg_shifter_attrib_lo: 0,
            bg_shifter_attrib_hi: 0,

            oam: [ObjectAttributeEntry::new(); 64],
            oam_addr: 0,
            sprite_scanline: [ObjectAttributeEntry::new(); 8],
            sprite_count: 0,
            sprite_shifter_pattern_lo: [0; 8],
            sprite_shifter_pattern_hi: [0; 8],

            sprite_zero_hit_possible: false,
            sprite_zero_being_rendered: false,

            nmi: false,
        }
    }

    pub fn get_cartridge(&mut self) -> Option<&mut Cartridge> {
        if let Some(cartridge) = self.cartridge {
            unsafe {
                if let Some(cart) = cartridge.as_mut() {
                    return Some(cart);
                }
            }
            return None;
        }

        return None;
    }

    pub fn connect_cartridge(&mut self, cartridge: &mut Cartridge) {
        self.cartridge = Some(cartridge);
    }

    pub fn reset(&mut self) {
        self.fine_x = 0x00;
        self.address_latch = 0x00;
        self.ppu_data_buffer = 0x00;
        self.scanline = 0;
        self.cycle = 0;
        self.bg_next_tile_id = 0x00;
        self.bg_next_tile_attrib = 0x00;
        self.bg_next_tile_lsb = 0x00;
        self.bg_next_tile_msb = 0x00;
        self.bg_shifter_pattern_lo = 0x0000;
        self.bg_shifter_pattern_hi = 0x0000;
        self.bg_shifter_attrib_lo = 0x0000;
        self.bg_shifter_attrib_hi = 0x0000;
        self.status.reg = 0x00;
        self.mask.reg = 0x00;
        self.control.reg = 0x00;
        self.vram_addr.reg = 0x0000;
        self.tram_addr.reg = 0x0000;
    }
}
