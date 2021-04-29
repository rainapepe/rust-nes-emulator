use graphics::{clear, text, text::Text, Context};
use opengl_graphics::GlGraphics;
use piston::Key;

use crate::cartridge::Cartridge;
use crate::pad::PadButton;
use crate::video::{Pixel, Video, BLACK_PIXEL};
use crate::{bus::Bus, cpu::Cpu6502};
pub struct Nes {
    pub cpu: Cpu6502,
    palette_table: u8,
    cartridge: String,
    running: bool,
}

impl Video for Nes {
    fn draw(&mut self, context: Context, gl: &mut GlGraphics) {
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

        let ppu = &mut self.cpu.bus.ppu;
        // Draw screen
        ppu.sprite_screen.render(0, 0, 2.7, context, gl);

        // Draw Palettes & Pattern Tables ==============================================
        let swatch_size = 8;
        for p in 0..8 {
            for s in 0..4 {
                let x: u64 = 720 + p * (swatch_size * 5) + s * swatch_size;
                let y: u64 = 380;

                let pixel = graphics::rectangle::square(x as f64, y as f64, swatch_size as f64);

                let color = ppu.get_colour_from_palette_ram(p as u8, s as u8);
                graphics::rectangle(color.get_color(), pixel, context.transform, gl);
            }
        }

        // const int nSwatchSize = 6;
        // for (int p = 0; p < 8; p++) // For each palette
        // 	for (int s = 0; s < 4; s++) // For each index
        // 		FillRect(516 + p * (nSwatchSize * 5) + s * nSwatchSize, 340,
        // 			nSwatchSize, nSwatchSize, nes.ppu.GetColourFromPaletteRam(p, s));

        // Draw pattern
        ppu.get_pattern_table(0, self.palette_table)
            .render(720, 420, 1.8, context, gl);
        ppu.get_pattern_table(1, self.palette_table)
            .render(960, 420, 1.8, context, gl);

        // ppu.sprite_pattern_table[0].render(720, 420, 1.8, context, gl);
        // ppu.sprite_pattern_table[1].render(960, 420, 1.8, context, gl);
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
            Key::N => self.cpu.clock(),
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
            // Key::P => self.running = !self.running,
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

        let cartridge = self.cartridge.to_string();
        // self.running = true;
        self.start_loop(&cartridge);
    }

    // fn draw_cpu(x: u32, y: u32, context: Context, gl: &mut GlGraphics) {
    //     let status = "STATUS: ";
    //     let font_size = 12;
    //     let green: [f32; 4] = [0.0, 255.0, 0.0, 1.0];
    //     let red: [f32; 4] = [255.0, 0.0, 0.0, 1.0];
    //     let white: [f32; 4] = [255.0, 255.0, 255.0, 1.0];

    //     Text::new(14).draw(text, cache, draw_state, transform, g)
    //     text(red, font_size, "Statis", cache, context.transform, gl);
    //     text(color, font_size, text, cache, context.transform, gl);
    //     DrawString(x , y , "STATUS:", olc::WHITE);
    //     DrawString(x  + 64, y, "N", nes.cpu.status & olc6502::N ? olc::GREEN : olc::RED);
    //     DrawString(x  + 80, y , "V", nes.cpu.status & olc6502::V ? olc::GREEN : olc::RED);
    //     DrawString(x  + 96, y , "-", nes.cpu.status & olc6502::U ? olc::GREEN : olc::RED);
    //     DrawString(x  + 112, y , "B", nes.cpu.status & olc6502::B ? olc::GREEN : olc::RED);
    //     DrawString(x  + 128, y , "D", nes.cpu.status & olc6502::D ? olc::GREEN : olc::RED);
    //     DrawString(x  + 144, y , "I", nes.cpu.status & olc6502::I ? olc::GREEN : olc::RED);
    //     DrawString(x  + 160, y , "Z", nes.cpu.status & olc6502::Z ? olc::GREEN : olc::RED);
    //     DrawString(x  + 178, y , "C", nes.cpu.status & olc6502::C ? olc::GREEN : olc::RED);
    //     DrawString(x , y + 10, "PC: $" + hex(nes.cpu.pc, 4));
    //     DrawString(x , y + 20, "A: $" +  hex(nes.cpu.a, 2) + "  [" + std::to_string(nes.cpu.a) + "]");
    //     DrawString(x , y + 30, "X: $" +  hex(nes.cpu.x, 2) + "  [" + std::to_string(nes.cpu.x) + "]");
    //     DrawString(x , y + 40, "Y: $" +  hex(nes.cpu.y, 2) + "  [" + std::to_string(nes.cpu.y) + "]");
    //     DrawString(x , y + 50, "Stack P: $" + hex(nes.cpu.stkp, 4));
    // }
}
