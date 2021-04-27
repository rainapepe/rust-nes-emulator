use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::Key;

use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::video::Video;

pub struct Nes {
    bus: Bus,
    cartridge: Option<String>,
}

impl Video for Nes {
    fn draw(&mut self, context: Context, gl: &GlGraphics) {}

    fn on_buttom_press(&mut self, key: Key) {}

    fn on_buttom_release(&mut self, key: Key) {}
}

impl Nes {
    pub fn new() -> Nes {
        Nes {
            bus: Bus::new(),
            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, file_name: String) {
        self.cartridge = Some(file_name.to_string());
        let cartridge = Cartridge::new(file_name);
        self.bus.insert_cartridge(cartridge);
    }

    pub fn start(&mut self) {}
}
