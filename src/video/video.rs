extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use graphics::Context;
use opengl_graphics::OpenGL;
use piston::{input::RenderEvent, Button};
use piston::{window::WindowSettings, ReleaseEvent};
use piston::{Key, PressEvent};
use piston_window::*;

pub trait Video {
    fn draw(&mut self, context: Context, gl: &mut G2d, glyphs: &mut Glyphs);

    fn main_loop(&mut self);

    fn on_buttom_press(&mut self, key: Key) {}

    fn on_buttom_release(&mut self, key: Key) {}

    fn start_loop(&mut self, title: &str) {
        let opengl = OpenGL::V3_2;

        let mut window: PistonWindow = WindowSettings::new(title, [1280, 720])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let font = assets.join("FiraSans-Regular.ttf");
        let mut glyphs = window.load_font(font).unwrap();

        while let Some(e) = window.next() {
            if let Some(_) = e.render_args() {
                self.main_loop();
                window.draw_2d(&e, |c, gl, device| {
                    self.draw(c, gl, &mut glyphs);
                    // Update glyphs before rendering.
                    glyphs.factory.encoder.flush(device);
                });
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.on_buttom_press(key);
            }

            if let Some(Button::Keyboard(key)) = e.release_args() {
                self.on_buttom_release(key);
            }
        }
    }
}
