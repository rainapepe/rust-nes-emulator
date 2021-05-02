extern crate graphics;
extern crate opengl_graphics;

use graphics::Context;
use piston_window::G2d;

use super::pixel::{Pixel, BLACK_PIXEL};

const PIXEL_SIZE: f64 = 2.3;

#[derive(Clone)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    data: Vec<Pixel>,
} // Matrix de pixels linhasxcolunas

impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        Frame {
            width,
            height,
            data: vec![BLACK_PIXEL; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        let position: usize = (y * self.width) + x;
        if position < self.data.len() {
            self.data[position] = pixel;
        }
    }

    pub fn set_border(&mut self, color: Pixel) {
        for i in 0..self.height {
            self.set_pixel(0, i, color);
            self.set_pixel(self.width - 1, i, color);
        }

        for i in 0..self.width {
            self.set_pixel(i, 0, color);
            self.set_pixel(i, self.height - 1, color);
        }
    }

    pub fn render(&self, x: usize, y: usize, pixel_size: f64, context: Context, gl: &mut G2d) {
        for pixel_y in 0..self.height {
            for pixel_x in 0..self.width {
                let position: usize = (pixel_y * self.width) + pixel_x;
                if position < self.data.len() {
                    let color = self.data[position];
                    let pixel = graphics::rectangle::square(
                        x as f64 + ((pixel_x as f64) * pixel_size),
                        y as f64 + ((pixel_y as f64) * pixel_size),
                        pixel_size,
                    );
                    graphics::rectangle(color.get_color(), pixel, context.transform, gl);
                }
            }
        }
    }
}
