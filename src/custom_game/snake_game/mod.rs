mod code;

use graphics::{clear, CharacterCache, Context};
use piston::Key;
use piston_window::{G2d, Glyphs};
use rand::Rng;

use std::{collections::HashMap, thread, time::Duration};

use crate::video::{draw_text, Frame, Pixel, Video, BLACK_PIXEL};
use crate::{bus::Bus, video::draw_cpu};
use crate::{cartridge::Cartridge, video::draw_code};
use crate::{cpu::Cpu6502, video::draw_ram};

use code::GAME_CODE;

fn color(byte: u8) -> Pixel {
    match byte {
        0 => Pixel::new(50, 50, 50),        // sdl2::pixels::Color::BLACK,
        1 => Pixel::new(255, 255, 255),     //  sdl2::pixels::Color::WHITE,
        2 | 9 => Pixel::new(100, 100, 100), //  sdl2::pixels::Color::GREY,
        3 | 10 => Pixel::new(255, 0, 0),    //  sdl2::pixels::Color::RED,
        4 | 11 => Pixel::new(0, 255, 0),    //  sdl2::pixels::Color::GREEN,
        5 | 12 => Pixel::new(0, 0, 255),    //  sdl2::pixels::Color::BLUE,
        6 | 13 => Pixel::new(255, 0, 255),  //  sdl2::pixels::Color::MAGENTA,
        7 | 14 => Pixel::new(255, 255, 0),  //  sdl2::pixels::Color::YELLOW,
        _ => Pixel::new(0, 255, 255),       //  sdl2::pixels::Color::CYAN,
    }
}

pub struct SnakeGame {
    pub cpu: Cpu6502,
    running: bool,
    map_assemble: HashMap<u16, String>,
    history: Vec<u16>,
    ram_offset: u16,
}

// Draws
impl SnakeGame {
    fn draw_screen(&mut self, context: Context, gl: &mut G2d) {
        let mut frame = Frame::new(32, 32);

        for color_y in 0..32 {
            for color_x in 0..32 {
                let offset = 0x0200 + (color_y * 32) + color_x;
                let pixel = self.cpu.read(offset as u16);

                frame.set_pixel(color_x, color_y, color(pixel));
            }
        }

        frame.render(50, 50, 10.0, context, gl);
    }
}

impl Video for SnakeGame {
    fn draw(&mut self, context: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        clear(BLACK_PIXEL.get_color(), gl);
        if self.running {
            // if self.cpu.pc == 0x06D0 {
            //     self.running = false;
            //     return;
            // }

            let mut rng = rand::thread_rng();
            let result = rng.gen_range(1, 16);
            self.cpu.write(0xfe, result);

            for _ in 0..10 {
                self.cpu.cpu_clock();
                while self.cpu.cycles > 0 {
                    self.cpu.cpu_clock();
                }
            }

            if self.history.len() == 5 {
                self.history.remove(0);
            }
            self.history.push(self.cpu.pc);

            // thread::sleep(Duration::new(0, 70_000));
        }

        // Draws
        self.draw_screen(context, gl);
        draw_cpu(550, 50, &mut self.cpu, context, gl, glyphs);
        draw_code(
            550,
            200,
            &self.history,
            &self.map_assemble,
            context,
            gl,
            glyphs,
        );
        draw_ram(
            550,
            400,
            self.ram_offset,
            &mut self.cpu,
            10,
            context,
            gl,
            glyphs,
        );
    }

    fn on_buttom_press(&mut self, key: Key) {
        match key {
            Key::Up => self.cpu.write(0xFF, 0x77),
            Key::Down => self.cpu.write(0xFF, 0x73),
            Key::Left => self.cpu.write(0xFF, 0x61),
            Key::Right => self.cpu.write(0xFF, 0x64),
            Key::P => self.running = !self.running,
            Key::PageDown => {
                if self.ram_offset < (0xFFFE - 100) {
                    self.ram_offset += 100;
                } else {
                    self.ram_offset = 0xFFFF;
                }
            }
            Key::PageUp => {
                if self.ram_offset > 100 {
                    self.ram_offset -= 100;
                } else {
                    self.ram_offset = 0;
                }
            }
            Key::NumPadPlus => {
                if self.ram_offset < 0xFFFE {
                    self.ram_offset += 1;
                }
            }
            Key::NumPadMinus => {
                if self.ram_offset > 0 {
                    self.ram_offset -= 1;
                }
            }
            Key::N => {
                self.cpu.cpu_clock();
                while self.cpu.cycles > 0 {
                    self.cpu.cpu_clock();
                }

                if self.history.len() == 5 {
                    self.history.remove(0);
                }
                self.history.push(self.cpu.pc);
            }
            _ => {}
        }
    }
}

impl SnakeGame {
    pub fn new() -> SnakeGame {
        let cartridge = Cartridge::empty();
        let bus = Bus::new(cartridge);
        SnakeGame {
            cpu: Cpu6502::new_with_bus(bus),
            running: false,
            map_assemble: HashMap::new(),
            history: vec![],
            ram_offset: 0,
        }
    }

    pub fn start(&mut self) {
        self.cpu.reset();
        self.cpu.load(0x0600, Vec::from(GAME_CODE));
        self.cpu.pc = 0x0600;

        self.map_assemble = self.cpu.disassemble(0x0600, 0x0735);
        self.start_loop("Snake Game");
    }
}
