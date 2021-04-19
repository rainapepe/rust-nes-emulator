use super::ppu2C02::Ppu2C02;

impl Ppu2C02 {
    pub fn cpu_read(&mut self, addr: u16, read_only: bool) -> u8 {
        if read_only {
            // Reading from PPU registers can affect their contents
            // so this read only option is used for examining the
            // state of the PPU without changing its state. This is
            // really only used in debug mode.

            match addr {
                // Control
                0x0000 => self.control.get_reg(),
                // Mask
                0x0001 => self.mask.get_reg(),
                // Status
                0x0002 => self.status.get_reg(),
                // OAM Address
                0x0003 => 0,
                // OAM Data
                0x0004 => 0,
                // Scroll
                0x0005 => 0,
                // PPU Address
                0x0006 => 0,
                // PPU Data
                0x0007 => 0,
                _ => 0,
            }
        } else {
            // These are the live PPU registers that repsond
            // to being read from in various ways. Note that not
            // all the registers are capable of being read from
            // so they just return 0x00
            match addr {
                // Control - Not readable
                0x0000 => 0,
                // Mask - Not Readable
                0x0001 => 0,
                // Status
                0x0002 => {
                    // Clear the vertical blanking flag
                    self.status.vertical_blank = 0;

                    // Reset Loopy's Address latch flag
                    self.address_latch = 0;

                    // Reading from the status register has the effect of resetting
                    // different parts of the circuit. Only the top three bits
                    // contain status information, however it is possible that
                    // some "noise" gets picked up on the bottom 5 bits which
                    // represent the last PPU bus transaction. Some games "may"
                    // use this noise as valid data (even though they probably
                    // shouldn't)
                    (self.status.get_reg() & 0xE0) | (self.ppu_data_buffer & 0x1F)
                }
                // OAM Address - Not Readable
                0x0003 => 0,
                // OAM Data
                0x0004 => {
                    // data = pOAM[oam_addr]; TODO: ???????????
                    0
                }
                // Scroll - Not Readable
                0x0005 => 0,
                // PPU Address - Not Readable
                0x0006 => 0,
                // PPU Data
                0x0007 => {
                    // Reads from the NameTable ram get delayed one cycle,
                    // so output buffer which contains the data from the
                    // previous read request
                    let data = self.ppu_data_buffer;

                    // then update the buffer for next time
                    self.ppu_data_buffer = self.ppu_read(self.vram_addr.get_reg());

                    // All reads from PPU data automatically increment the nametable
                    // address depending upon the mode set in the control register.
                    // If set to vertical mode, the increment is 32, so it skips
                    // one whole nametable row; in horizontal mode it just increments
                    // by 1, moving to the next column
                    self.vram_addr.set_reg(
                        self.vram_addr.get_reg()
                            + if self.control.increment_mode > 1 {
                                32
                            } else {
                                1
                            },
                    );

                    // However, if the address was in the palette range, the
                    // data is not delayed, so it returns immediately
                    if self.vram_addr.get_reg() >= 0x3F00 {
                        self.ppu_data_buffer
                    } else {
                        data
                    }
                }
                _ => 0,
            }
        }
    }

    pub fn ppu_read(&self, addr: u16) -> u8 {
        0
    }
}
