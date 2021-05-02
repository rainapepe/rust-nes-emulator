use graphics::{text::Text, Context};
use piston_window::*;

use super::Pixel;

pub fn draw_text(
    x: usize,
    y: usize,
    text: &str,
    color: Pixel,
    c: Context,
    gl: &mut G2d,
    char_cache: &mut Glyphs,
) {
    let transform = c.transform.trans(x as f64, y as f64);
    Text::new_color(color.get_color(), 32)
        .draw(text, char_cache, &c.draw_state, transform, gl)
        .unwrap();
}

// pub fn draw_text<C, G>(
//     x: usize,
//     y: usize,
//     text: &str,
//     color: Pixel,
//     c: Context,
//     gl: &mut G,
//     char_cache: &mut C,
// ) where
//     C: CharacterCache,
//     G: Graphics<Texture = <C as CharacterCache>::Texture>,
// {
//     let transform = c.transform.trans(x as f64, y as f64);
//     Text::new_color(color.get_color(), 32)
//         .draw(text, char_cache, &c.draw_state, transform, gl)
//         .unwrap_or_else(|_| {
//             panic!("Error on draw text!");
//         });
// }
