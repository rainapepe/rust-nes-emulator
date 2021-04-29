mod bus;
mod cartridge;
mod cpu;
mod mapper;
mod nes;
mod pad;
mod ppu;
mod video;

use nes::Nes;
use video::Pixel;

fn main() {
    let mut nes = Nes::new_with_cartridge("roms/donkeykong.nes");

    // nes.bus.ppu.sprite_screen.set_border(Pixel::new(255, 0, 0));
    // nes.bus.ppu.sprite_pattern_table[0].set_border(Pixel::new(255, 0, 0));
    // nes.bus.ppu.sprite_pattern_table[1].set_border(Pixel::new(255, 0, 0));

    nes.start();
}
