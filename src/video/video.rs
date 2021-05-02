extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::Context;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventSettings, Events},
    Key, PressEvent,
};
use piston::{input::RenderEvent, Button};
use piston::{window::WindowSettings, ReleaseEvent};

pub trait Video {
    fn draw(&mut self, context: Context, gl: &mut GlGraphics);

    fn on_buttom_press(&mut self, key: Key) {}

    fn on_buttom_release(&mut self, key: Key) {}

    fn start_loop(&mut self, title: &str) {
        let opengl = OpenGL::V3_2;

        let mut window: Window = WindowSettings::new(title, [1280, 720])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut gl = GlGraphics::new(opengl);
        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                let view = args.viewport();
                gl.draw(view, |c, graph| {
                    self.draw(c, graph);
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

// match key {
//     Key::Up => {
//         y = y - 1;
//         nes.ppu.sprite_screen.set_pixel(
//             x as usize,
//             y as usize,
//             Pixel::new(255, 255, 255),
//         );
//     }
//     Key::Down => {
//         y = y + 1;
//         nes.ppu.sprite_screen.set_pixel(
//             x as usize,
//             y as usize,
//             Pixel::new(255, 255, 255),
//         );
//     }
//     Key::Right => {
//         x = x + 1;
//         nes.ppu.sprite_screen.set_pixel(
//             x as usize,
//             y as usize,
//             Pixel::new(255, 255, 255),
//         );
//     }
//     Key::Left => {
//         x = x - 1;
//         nes.ppu.sprite_screen.set_pixel(
//             x as usize,
//             y as usize,
//             Pixel::new(255, 255, 255),
//         );
//     }
//     _ => {}
// }
