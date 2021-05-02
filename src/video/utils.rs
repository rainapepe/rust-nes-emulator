use graphics::{text::Text, types::Color, Context};
use piston_window::*;

use crate::cpu::{Cpu6502, Flags6502};

const FONT_SIZE: usize = 18;
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn active_color(active: bool) -> Color {
    if active {
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

pub fn draw_cpu(
    x: usize,
    y: usize,
    cpu: &mut Cpu6502,
    c: Context,
    gl: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let mut i = 0;
    let mut j = 0;
    let mut draw_line = |text: &str, color: Color, endl: bool| {
        draw_text(
            x + j * FONT_SIZE,
            y + FONT_SIZE * i,
            text,
            color,
            c,
            gl,
            glyphs,
        );
        if endl {
            i += 1;
            j = 0;
        } else {
            j += text.len();
        }
    };

    draw_line(&format!("PC: {:#06x}", cpu.pc), WHITE, true);
    draw_line(&format!("A: {:#04x}", cpu.a), WHITE, true);
    draw_line(&format!("X: {:#04x}", cpu.x), WHITE, true);
    draw_line(&format!("Y: {:#04x}", cpu.y), WHITE, true);
    draw_line(&format!("STACK: {:#04x}", cpu.stkp), WHITE, true);

    draw_line("", WHITE, true);
    draw_line("STATUS: ", WHITE, false);
    draw_line("N", active_color(cpu.get_flag(Flags6502::N) == 1), false);
    draw_line("V", active_color(cpu.get_flag(Flags6502::V) == 1), false);
    draw_line("U", active_color(cpu.get_flag(Flags6502::U) == 1), false);
    draw_line("B", active_color(cpu.get_flag(Flags6502::B) == 1), false);
    draw_line("D", active_color(cpu.get_flag(Flags6502::D) == 1), false);
    draw_line("I", active_color(cpu.get_flag(Flags6502::I) == 1), false);
    draw_line("Z", active_color(cpu.get_flag(Flags6502::Z) == 1), false);
    draw_line("C", active_color(cpu.get_flag(Flags6502::C) == 1), false);
}
