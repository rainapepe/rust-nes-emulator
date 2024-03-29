mod code;

use graphics::{clear, Context};
use piston::Key;
use piston_window::{G2d, G2dTextureContext, Glyphs};
use rand::Rng;

use std::{thread, time::Duration};

use crate::video::{Frame, Pixel, Video, BLACK_PIXEL};
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
    history: Vec<String>,
    ram_offset: u16,
    screen: Frame,
}

// Draws
impl SnakeGame {
    fn update_screen(&mut self) {
        for color_y in 0..32 {
            for color_x in 0..32 {
                let offset = 0x0200 + (color_y * 32) + color_x;
                let pixel = self.cpu.read(offset as u16);

                self.screen.set_pixel(color_x, color_y, color(pixel));
            }
        }
    }
}

impl Video for SnakeGame {
    fn main_loop(&mut self) {
        if self.running {
            // if self.cpu.pc == 0x06D0 {
            //     self.running = false;
            //     return;
            // }

            let mut rng = rand::thread_rng();
            let result = rng.gen_range(1, 16);
            self.cpu.write(0xfe, result);

            loop {
                let end_cycle = self.cpu.pc == 0x734;
                self.cpu.cpu_clock();
                while self.cpu.cycles > 0 {
                    self.cpu.cpu_clock();
                }
                if end_cycle {
                    break;
                }
            }

            if self.history.len() == 5 {
                self.history.remove(0);
            }
            self.history.push(self.cpu.disassemble_instruction());
            self.update_screen();

            thread::sleep(Duration::from_millis(100));
        }
    }

    fn update_textures(&mut self, texture_context: &mut G2dTextureContext) {
        self.screen.update_texture(texture_context);
    }

    fn draw(&mut self, context: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        clear(BLACK_PIXEL.get_color(), gl);

        // Draws
        self.screen.render_image(50, 50, 10.0, context, gl);
        draw_cpu(550, 50, &mut self.cpu, context, gl, glyphs);
        draw_code(550, 200, &self.history, context, gl, glyphs);
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
                self.history.push(self.cpu.disassemble_instruction());
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
            history: vec![],
            ram_offset: 0,
            screen: Frame::new(32, 32),
        }
    }

    pub fn start(&mut self) {
        self.cpu.reset();
        self.cpu.load(0x0600, Vec::from(GAME_CODE));
        self.cpu.pc = 0x0600;

        self.start_loop("Snake Game");
    }
}
