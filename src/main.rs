mod bus;
mod cartridge;
mod cpu;
mod custom_game;
mod mapper;
mod nes;
mod pad;
mod ppu;
mod video;

use nes::Nes;

fn main() {
    // let mut nes = Nes::new_with_cartridge("roms/donkeykong.nes");
    // let mut nes = Nes::new_with_cartridge("roms/helloworld.nes");
    let mut nes = custom_game::SnakeGame::new();

    // nes.bus.ppu.sprite_screen.set_border(Pixel::new(255, 0, 0));
    // nes.bus.ppu.sprite_pattern_table[0].set_border(Pixel::new(255, 0, 0));
    // nes.bus.ppu.sprite_pattern_table[1].set_border(Pixel::new(255, 0, 0));

    nes.start();
}
