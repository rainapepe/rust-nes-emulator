mod bus;
mod cartridge;
mod cpu;
mod mapper;
mod ppu;

use bus::Bus;

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
}
