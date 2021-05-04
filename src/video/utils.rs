use std::collections::HashMap;

use graphics::{text::Text, types::Color, Context};
use piston_window::*;

use crate::cpu::{Cpu6502, Flags6502};
pub const FONT_SIZE: usize = 18;
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

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

pub struct DrawText {
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
