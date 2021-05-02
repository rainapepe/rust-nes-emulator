use std::collections::HashMap;

use graphics::{text::Text, types::Color, Context};
use piston_window::*;

use crate::cpu::{Cpu6502, Flags6502};
const FONT_SIZE: usize = 18;
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn active_color(active: u8) -> Color {
    if active == 1 {
        GREEN
    } else {
        RED
    }
}

pub fn draw_text(
    x: usize,
    y: usize,
    text: &str,
    color: Color,
    c: Context,
    gl: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let transform = c.transform.trans(x as f64, y as f64);
    Text::new_color(color, FONT_SIZE as u32)
        .draw(text, glyphs, &c.draw_state, transform, gl)
        .unwrap();
}

struct DrawText {
    x: usize,
    y: usize,
    i: usize,
    j: usize,
    c: Context,
    font_size: usize,
}

impl DrawText {
    pub fn new(x: usize, y: usize, c: Context) -> DrawText {
        DrawText {
            x,
            y,
            c,
            i: 0,
            j: 0,
            font_size: FONT_SIZE,
        }
    }

    pub fn with_font_size(font_size: usize, x: usize, y: usize, c: Context) -> DrawText {
        DrawText {
            x,
            y,
            c,
            i: 0,
            j: 0,
            font_size,
        }
    }

    pub fn break_line(&mut self) {
        self.i += 1;
        self.j = 0;
    }

    pub fn draw_line(&mut self, text: &str, color: Color, gl: &mut G2d, glyphs: &mut Glyphs) {
        self.i += 1;
        self.j = 0;
        draw_text(
            self.x + self.j * self.font_size,
            self.y + self.font_size * self.i,
            text,
            color,
            self.c,
            gl,
            glyphs,
        );

        self.j += text.len();
    }

    pub fn draw(&mut self, text: &str, color: Color, gl: &mut G2d, glyphs: &mut Glyphs) {
        draw_text(
            self.x + self.j * self.font_size,
            self.y + self.font_size * self.i,
            text,
            color,
            self.c,
            gl,
            glyphs,
        );

        self.j += text.len();
    }
}

pub fn draw_cpu(
    x: usize,
    y: usize,
    cpu: &mut Cpu6502,
    c: Context,
    gl: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let mut text = DrawText::new(x, y, c);

    text.draw_line(&format!("PC: {:#06x}", cpu.pc), WHITE, gl, glyphs);
    text.draw_line(&format!("A: {:#04x}", cpu.a), WHITE, gl, glyphs);
    text.draw_line(&format!("X: {:#04x}", cpu.x), WHITE, gl, glyphs);
    text.draw_line(&format!("Y: {:#04x}", cpu.y), WHITE, gl, glyphs);
    text.draw_line(&format!("STACK: {:#04x}", cpu.stkp), WHITE, gl, glyphs);

    text.break_line();
    text.draw("STATUS: ", WHITE, gl, glyphs);
    text.draw("N", active_color(cpu.get_flag(Flags6502::N)), gl, glyphs);
    text.draw("V", active_color(cpu.get_flag(Flags6502::V)), gl, glyphs);
    text.draw("U", active_color(cpu.get_flag(Flags6502::U)), gl, glyphs);
    text.draw("B", active_color(cpu.get_flag(Flags6502::B)), gl, glyphs);
    text.draw("D", active_color(cpu.get_flag(Flags6502::D)), gl, glyphs);
    text.draw("I", active_color(cpu.get_flag(Flags6502::I)), gl, glyphs);
    text.draw("Z", active_color(cpu.get_flag(Flags6502::Z)), gl, glyphs);
    text.draw("C", active_color(cpu.get_flag(Flags6502::C)), gl, glyphs);
}

pub fn draw_code(
    pc: u16,
    map_assemble: &HashMap<u16, String>,
    x: usize,
    y: usize,
    lines: usize,
    context: Context,
    gl: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let it_a = map_assemble.get(&pc);
    // let mut it = map_assemble.iter();

    let mut text = DrawText::new(x, y, context);
    if let Some(value) = it_a {
        text.draw_line(value, WHITE, gl, glyphs);
    }

    // while let Some((key, value)) = it.next() {
    //     text.draw_line(value, WHITE, gl, glyphs);
    // }

    // if it_a != self.map_assemble.
    // {
    //     DrawString(x, nLineY, (*it_a).second, olc::CYAN);
    //     while (nLineY < (nLines * 10) + y)
    //     {
    //         nLineY += 10;
    //         if (++it_a != mapAsm.end())
    //         {
    //             DrawString(x, nLineY, (*it_a).second);
    //         }
    //     }
    // }

    // it_a = mapAsm.find(nes.cpu.pc);
    // nLineY = (nLines >> 1) * 10 + y;
    // if (it_a != mapAsm.end())
    // {
    //     while (nLineY > y)
    //     {
    //         nLineY -= 10;
    //         if (--it_a != mapAsm.end())
    //         {
    //             DrawString(x, nLineY, (*it_a).second);
    //         }
    //     }
    // }
}
