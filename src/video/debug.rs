use std::collections::HashMap;

use graphics::{types::Color, Context};
use piston_window::*;

use super::utils::{DrawText, GREEN, RED, WHITE};
use crate::cpu::{Cpu6502, Flags6502};

fn active_color(active: u8) -> Color {
    if active == 1 {
        GREEN
    } else {
        RED
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
    text.draw_line(
        &format!("FECHETED: {:#04x}", cpu.fetched),
        WHITE,
        gl,
        glyphs,
    );

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
    x: usize,
    y: usize,
    history: &Vec<u16>,
    map_assemble: &HashMap<u16, String>,
    context: Context,
    gl: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let mut text = DrawText::new(x, y, context);

    for instruction in history {
        if let Some(line) = map_assemble.get(&instruction) {
            text.draw_line(line, WHITE, gl, glyphs);
        }
    }
}

pub fn draw_ram(
    x: usize,
    y: usize,
    offset: u16,
    cpu: &mut Cpu6502,
    lines: u16,
    context: Context,
    gl: &mut G2d,
    glyphs: &mut Glyphs,
) {
    let mut text = DrawText::new(x, y, context);
    text.draw_line("RAM-------------", WHITE, gl, glyphs);

    for line in 0..lines {
        let address = offset + line;
        let value = cpu.bus_read(address, true);
        text.draw_line(
            &format!("[{:#06x} ]: {:#04x} - {}", address, value, value),
            WHITE,
            gl,
            glyphs,
        );
    }
}
