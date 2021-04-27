pub enum PadButton {
    A,
    B,
    Up,
    Down,
    Right,
    Left,
    Start,
    Select,
}

pub struct Pad {
    reg: u8,
}

impl Pad {
    pub fn new() -> Pad {
        Pad { reg: 0 }
    }

    pub fn reset(&mut self) {
        self.reg = 0;
    }

    pub fn press_button(&mut self, button: PadButton) {
        match button {
            PadButton::A => self.reg |= 0x80,
            PadButton::B => self.reg |= 0x40,
            PadButton::Select => self.reg |= 0x20,
            PadButton::Start => self.reg |= 0x10,
            PadButton::Up => self.reg |= 0x08,
            PadButton::Down => self.reg |= 0x04,
            PadButton::Left => self.reg |= 0x02,
            PadButton::Right => self.reg |= 0x01,
        }
    }

    pub fn release_button(&mut self, button: PadButton) {
        match button {
            PadButton::A => self.reg &= !0x80,
            PadButton::B => self.reg &= !0x40,
            PadButton::Select => self.reg &= !0x20,
            PadButton::Start => self.reg &= !0x10,
            PadButton::Up => self.reg &= !0x08,
            PadButton::Down => self.reg &= !0x04,
            PadButton::Left => self.reg &= !0x02,
            PadButton::Right => self.reg &= !0x01,
        }
    }

    pub fn get_reg(&mut self) -> u8 {
        self.reg
    }
}
