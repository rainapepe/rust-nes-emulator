mod bus;
mod cartridge;
mod cpu;
mod custom_game;
mod debug;
mod mapper;
mod nes;
mod pad;
mod ppu;
mod video;

use nes::Nes;

fn main() {
    let mut nes = Nes::new_with_cartridge("roms/donkeykong.nes");
    // let mut nes = Nes::new_with_cartridge("roms/helloworld.nes");
    // let mut nes = Nes::new_with_cartridge("roms/nestest.nes");
    // let mut nes = custom_game::SnakeGame::new();

    nes.start();
}
