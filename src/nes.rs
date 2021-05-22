use super::debug;
use std::collections::HashMap;

use graphics::{clear, text, text::Text, CharacterCache, Context};
use opengl_graphics::GlGraphics;
use piston::Key;
use piston_window::{G2d, G2dTextureContext, Glyphs};

use crate::{
    bus::Bus,
    cpu::{Cpu6502, Opcode},
};
use crate::{
    cartridge::Cartridge,
    video::{draw_code, draw_cpu},
};
use crate::{
    cpu::Instruction,
    video::{Video, BLACK_PIXEL},
};
use crate::{pad::PadButton, video::draw_ram};

pub struct Nes {
    pub cpu: Cpu6502,
    palette_table: u8,
    cartridge: String,
    running: bool,
    history: Vec<String>,
    ram_offset: u16,
}

// Draws
impl Nes {
    fn draw_palette(&mut self, context: Context, gl: &mut G2d) {
        // Draw Palettes & Pattern Tables ==============================================
        let swatch_size = 8;
        for p in 0..8 {
            for s in 0..4 {
                let x: u64 = 720 + p * (swatch_size * 5) + s * swatch_size;
                let y: u64 = 380;

                let pixel = graphics::rectangle::square(x as f64, y as f64, swatch_size as f64);

                let color = self
                    .cpu
                    .bus
                    .ppu
                    .get_colour_from_palette_ram(p as u8, s as u8);
                graphics::rectangle(color.get_color(), pixel, context.transform, gl);
            }
        }
    }

    fn draw_patterns(&mut self, context: Context, gl: &mut G2d) {
        let ppu = &mut self.cpu.bus.ppu;
        // Draw pattern
        ppu.get_pattern_table(0, self.palette_table)
            .render_image(720, 420, 1.8, context, gl);
        ppu.get_pattern_table(1, self.palette_table)
            .render_image(960, 420, 1.8, context, gl);
    }

    fn draw_screen(&mut self, context: Context, gl: &mut G2d) {
        self.cpu
            .bus
            .ppu
            .sprite_screen
            .render_image(0, 0, 2.7, context, gl);
    }

    fn push_history(&mut self) {
        if self.history.len() == 5 {
            self.history.remove(0);
        }
        self.history.push(self.cpu.disassemble_instruction());
    }
}

impl Video for Nes {
    fn main_loop(&mut self) {
        if self.running {
            loop {
                self.cpu.clock();
                if self.cpu.complete() && self.cpu.bus.system_clock_counter % 3 == 0 {
                    self.push_history();
                    // let opcode = self.cpu.read(self.cpu.pc);
                    // let instruction = Instruction::from(opcode);

                    // if self.cpu.pc == 0xEB9E {
                    //     self.running = false;
                    //     break;
                    // }

                    // if opcode != debug::get_good_log(self.cpu.pc) {
                    //     self.running = false;
                    //     break;
                    // }

                    // if let Opcode::XXX = instruction.opcode {
                    //     self.running = false;
                    //     break;
                    // }
                }

                if self.cpu.bus.ppu.frame_complete {
                    break;
                }
            }
            self.cpu.bus.ppu.frame_complete = false;
        }
    }

    fn update_textures(&mut self, texture_context: &mut G2dTextureContext) {
        self.cpu
            .bus
            .ppu
            .sprite_screen
            .update_texture(texture_context);

        let ppu = &mut self.cpu.bus.ppu;
        // Draw pattern
        ppu.get_pattern_table(0, self.palette_table)
            .update_texture(texture_context);
        ppu.get_pattern_table(1, self.palette_table)
            .update_texture(texture_context);
    }

    fn draw(&mut self, context: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        clear(BLACK_PIXEL.get_color(), gl);

        // Draws
        self.draw_screen(context, gl);
        self.draw_palette(context, gl);
        self.draw_patterns(context, gl);
        draw_cpu(720, 10, &mut self.cpu, context, gl, glyphs);
        draw_code(720, 150, &self.history, context, gl, glyphs);
        draw_ram(
            1020,
            10,
            self.ram_offset,
            &mut self.cpu,
            10,
            context,
            gl,
            glyphs,
        );
    }

    fn on_buttom_press(&mut self, key: Key) {
        let pad1 = &mut self.cpu.bus.pad1;
        match key {
            Key::Z => pad1.press_button(PadButton::B),
            Key::X => pad1.press_button(PadButton::A),
            Key::Up => pad1.press_button(PadButton::Up),
            Key::Down => pad1.press_button(PadButton::Down),
            Key::Right => pad1.press_button(PadButton::Right),
            Key::Left => pad1.press_button(PadButton::Left),
            Key::Space => pad1.press_button(PadButton::Start),
            Key::C => pad1.press_button(PadButton::Select),
            Key::P => self.running = !self.running,
            Key::N => {
                // self.cpu.clock();
                while !self.cpu.complete() {
                    self.cpu.clock();
                }

                self.cpu.clock();
                self.cpu.clock();
                self.cpu.clock();

                self.push_history();
            }
            Key::T => {
                if self.palette_table == 7 {
                    self.palette_table = 0;
                } else {
                    self.palette_table += 1;
                }
            }
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

            _ => {}
        }
    }

    fn on_buttom_release(&mut self, key: Key) {
        let pad1 = &mut self.cpu.bus.pad1;

        match key {
            Key::Z => pad1.release_button(PadButton::B),
            Key::X => pad1.release_button(PadButton::A),
            Key::Up => pad1.release_button(PadButton::Up),
            Key::Down => pad1.release_button(PadButton::Down),
            Key::Right => pad1.release_button(PadButton::Right),
            Key::Left => pad1.release_button(PadButton::Left),
            Key::Space => pad1.release_button(PadButton::Start),
            Key::C => pad1.release_button(PadButton::Select),
            _ => {}
        }
    }
}

impl Nes {
    pub fn new_with_cartridge(file_name: &str) -> Nes {
        let cartridge = Cartridge::new(file_name.to_string());
        let bus = Bus::new(cartridge);
        Nes {
            cpu: Cpu6502::new_with_bus(bus),
            cartridge: file_name.to_string(),
            running: false,
            palette_table: 0,
            history: vec![],
            ram_offset: 0,
        }
    }

    pub fn start(&mut self) {
        if self.cartridge.is_empty() {
            panic!("[nes] No cartridge selected!");
        }

        self.cpu.reset();
        let cartridge = self.cartridge.to_string();
        // self.cpu.pc = 0xC000;
        self.history.push(self.cpu.disassemble_instruction());

        // self.running = true;
        self.start_loop(&cartridge);
    }
}
