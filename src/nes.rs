use graphics::{clear, text, text::Text, CharacterCache, Context};
use opengl_graphics::GlGraphics;
use piston::Key;
use piston_window::{G2d, Glyphs};

use crate::cartridge::Cartridge;
use crate::pad::PadButton;
use crate::video::{Video, BLACK_PIXEL};
use crate::{bus::Bus, cpu::Cpu6502};

pub struct Nes {
    pub cpu: Cpu6502,
    palette_table: u8,
    cartridge: String,
    running: bool,
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
            .render(720, 420, 1.8, context, gl);
        ppu.get_pattern_table(1, self.palette_table)
            .render(960, 420, 1.8, context, gl);
    }

    fn draw_screen(&mut self, context: Context, gl: &mut G2d) {
        self.cpu
            .bus
            .ppu
            .sprite_screen
            .render(0, 0, 2.7, context, gl);
    }
}

impl Video for Nes {
    fn draw(&mut self, context: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        clear(BLACK_PIXEL.get_color(), gl);
        if self.running {
            loop {
                self.cpu.clock();
                if self.cpu.bus.ppu.frame_complete {
                    break;
                }
            }

            self.cpu.bus.ppu.frame_complete = false;
        }

        // Draws
        self.draw_screen(context, gl);
        self.draw_palette(context, gl);
        self.draw_patterns(context, gl);
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
                self.cpu.cpu_clock();
                while self.cpu.cycles > 0 {
                    self.cpu.cpu_clock();
                }
            }
            Key::T => {
                if self.palette_table == 7 {
                    self.palette_table = 0;
                } else {
                    self.palette_table += 1;
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
        }
    }

    pub fn start(&mut self) {
        if self.cartridge.is_empty() {
            panic!("[nes] No cartridge selected!");
        }

        self.cpu.reset();
        let cartridge = self.cartridge.to_string();

        // self.running = true;
        self.start_loop(&cartridge);
    }
}
