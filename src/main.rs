mod bus;
mod cartridge;
mod cpu;
mod mapper;
mod nes;
mod pad;
mod ppu;
mod video;

use bus::Bus;
use video::{Pixel, BLACK_PIXEL};

#[derive(Debug)]
struct Test {
    name: [u8; 5],
    value1: [u8; 2],
    value2: u8,
    value3: u8,
}

fn main() {
    let mut nes = Bus::new();

    nes.write(0x0101, 10);
    println!("nes: {}", nes.read(0x0101, false));

    // 256x240
    nes.ppu
        .sprite_screen
        .set_pixel(100, 100, Pixel::new(255, 0, 0));
    nes.ppu
        .sprite_screen
        .set_pixel(254, 239, Pixel::new(0, 255, 0));
}
