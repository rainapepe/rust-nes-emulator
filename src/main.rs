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
use std::env;

enum GAME_MODE {
    ROM,
    CUSTOM,
}

fn main() {
    let mut rom = "roms/ice_climbers.nes";
    // let mut rom = "roms/donkeykong.nes";
    // let mut rom = "roms/snowbros.nes";
    // let mut rom = "roms/helloworld.nes";
    // let mut rom = "roms/color_test.nes";
    // let mut rom = "roms/nestest.nes";

    let args: Vec<String> = env::args().collect();
    let mut mode = GAME_MODE::ROM;
    let mut debug = false;
    let mut game = "snake";

    for i in 0..args.len() {
        let arg = &args[i][..];

        if arg == "--rom" && i + 1 < args.len() {
            rom = &args[i + 1];
            mode = GAME_MODE::ROM;
        }

        if arg == "--debug" {
            debug = true;
        }

        if arg == "--custom" && i + 1 < args.len() {
            game = &args[i + 1];
            mode = GAME_MODE::CUSTOM;
        }
    }

    match mode {
        GAME_MODE::ROM => {
            let mut nes = Nes::new_with_cartridge(rom);
            nes.debug = debug;
            nes.start();
        }
        GAME_MODE::CUSTOM => {
            match game {
                "snake" => {
                    let mut nes = custom_game::SnakeGame::new();
                    nes.start();
                }
                _ => panic!("invalid custom game"),
            };
        }
    }
}
