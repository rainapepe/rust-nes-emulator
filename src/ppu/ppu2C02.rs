use super::pixel::{Pixel, Sprite};
use super::types::{LoopyRegister, Mask, ObjectAttributeEntry, PpuControl, Status};
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
    pub pal_screen: [Pixel; 0x40], // pallete screen, são as 64 cores que o nes possui
    pub spr_screen: Sprite,        // Tela final 256x240
    pub spr_name_table: [Sprite; 2], // (não usado) visualização da nametables (tela final na memória) as duas são 256x240
    pub spr_pattern_table: [Sprite; 2], // Visualização da tabela de sprites (background e foregrounds)

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
    pub oma: [ObjectAttributeEntry; 64],

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

    // TODO: ???????????????
    // The OAM is conveniently package above to work with, but the DMA
    // mechanism will need access to it for writing one byute at a time
    // uint8_t* pOAM = (uint8_t*)OAM;

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

            pal_screen: [Pixel {}; 0x40],
            spr_screen: Sprite {},
            spr_name_table: [Sprite {}; 2],
            spr_pattern_table: [Sprite {}; 2],

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

            bg_next_tile_id: 0,
            bg_next_tile_attrib: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            bg_shifter_pattern_lo: 0,
            bg_shifter_pattern_hi: 0,
            bg_shifter_attrib_lo: 0,
            bg_shifter_attrib_hi: 0,

            oma: [ObjectAttributeEntry::new(); 64],
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

    fn get_cartridge(&self) -> Option<&Cartridge> {
        if let Some(cartridge) = self.cartridge {
            unsafe {
                if let Some(cart) = cartridge.as_ref() {
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
}
