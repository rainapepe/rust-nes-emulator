use crate::cartridge::Mirror;

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
                0x0000 => self.control.reg,
                // Mask
                0x0001 => self.mask.reg,
                // Status
                0x0002 => self.status.reg,
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
                    self.status.set_vertical_blank(0);

                    // Reset Loopy's Address latch flag
                    self.address_latch = 0;

                    // Reading from the status register has the effect of resetting
                    // different parts of the circuit. Only the top three bits
                    // contain status information, however it is possible that
                    // some "noise" gets picked up on the bottom 5 bits which
                    // represent the last PPU bus transaction. Some games "may"
                    // use this noise as valid data (even though they probably
                    // shouldn't)
                    (self.status.reg & 0xE0) | (self.ppu_data_buffer & 0x1F)
                }
                // OAM Address - Not Readable
                0x0003 => 0,
                // OAM Data
                0x0004 => self.oam_read(self.oam_addr),
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
                    self.ppu_data_buffer = self.ppu_read(self.vram_addr.reg);

                    // All reads from PPU data automatically increment the nametable
                    // address depending upon the mode set in the control register.
                    // If set to vertical mode, the increment is 32, so it skips
                    // one whole nametable row; in horizontal mode it just increments
                    // by 1, moving to the next column
                    self.vram_addr.reg = self.vram_addr.reg
                        + if self.control.get_increment_mode() > 1 {
                            32
                        } else {
                            1
                        };

                    // However, if the address was in the palette range, the
                    // data is not delayed, so it returns immediately
                    if self.vram_addr.reg >= 0x3F00 {
                        self.ppu_data_buffer
                    } else {
                        data
                    }
                }
                _ => 0,
            }
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            // Control
            0x0000 => {
                self.control.reg = data;
                self.tram_addr
                    .set_nametable_x(self.control.get_nametable_x());
                self.tram_addr
                    .set_nametable_y(self.control.get_nametable_y());
            }
            // Mask
            0x0001 => {
                self.mask.reg = data;
            }
            // Status
            0x0002 => {}
            // OAM Address
            0x0003 => {
                self.oam_addr = data;
            }
            // OAM Data
            0x0004 => {
                self.oam_write(self.oam_addr, data);
            }
            // Scroll
            0x0005 => {
                if self.address_latch == 0 {
                    // First write to scroll register contains X offset in pixel space
                    // which we split into coarse and fine x values
                    self.fine_x = data & 0x07;
                    self.tram_addr.set_coarse_x(data >> 3);
                    self.address_latch = 1;
                } else {
                    // First write to scroll register contains Y offset in pixel space
                    // which we split into coarse and fine Y values
                    self.tram_addr.set_fine_y(data & 0x07);
                    self.tram_addr.set_coarse_y(data >> 3);
                    self.address_latch = 0;
                }
            }
            // PPU Address
            0x0006 => {
                if self.address_latch == 0 {
                    // PPU address bus can be accessed by CPU via the ADDR and DATA
                    // registers. The fisrt write to this register latches the high byte
                    // of the address, the second is the low byte. Note the writes
                    // are stored in the tram register...
                    self.tram_addr.reg =
                        ((data as u16 & 0x3F) << 8) | (self.tram_addr.reg & 0x00FF);
                    self.address_latch = 1;
                } else {
                    // ...when a whole address has been written, the internal vram address
                    // buffer is updated. Writing to the PPU is unwise during rendering
                    // as the PPU will maintam the vram address automatically whilst
                    // rendering the scanline position.
                    self.tram_addr.reg = (self.tram_addr.reg & 0xFF00) | data as u16;
                    self.vram_addr = self.tram_addr;
                    self.address_latch = 0;
                }
            }
            // PPU Data
            0x0007 => {
                self.ppu_write(self.vram_addr.reg, data);
                // All writes from PPU data automatically increment the nametable
                // address depending upon the mode set in the control register.
                // If set to vertical mode, the increment is 32, so it skips
                // one whole nametable row; in horizontal mode it just increments
                // by 1, moving to the next column
                let increment = if self.control.get_increment_mode() > 0 {
                    32
                } else {
                    1
                };

                self.vram_addr.reg = self.vram_addr.reg + increment;
            }
            _ => {}
        }
    }

    pub fn ppu_read(&mut self, addr: u16) -> u8 {
        let mut address = addr & 0x3FFF;

        let (should_read, data) = self.chr_rom.read(address);
        if should_read {
            return data;
        }

        if address <= 0x1FFF {
            // If the cartridge cant map the address, have
            // a physical location ready here
            return self.table_pattern[((address & 0x1000) >> 12) as usize]
                [(address & 0x0FFF) as usize];
        }

        if address >= 0x2000 && address <= 0x3EFF {
            address &= 0x0FFF;

            if let Mirror::Vertical = self.chr_rom.mirror {
                // Vertical
                if address <= 0x03FF {
                    return self.table_name[0][(address & 0x03FF) as usize];
                }
                if address >= 0x0400 && address <= 0x07FF {
                    return self.table_name[1][(address & 0x03FF) as usize];
                }
                if address >= 0x0800 && address <= 0x0BFF {
                    return self.table_name[0][(address & 0x03FF) as usize];
                }
                if address >= 0x0C00 && address <= 0x0FFF {
                    return self.table_name[1][(address & 0x03FF) as usize];
                }
            }

            if let Mirror::Horizontal = self.chr_rom.mirror {
                // Horizontal
                if address <= 0x03FF {
                    return self.table_name[0][(address & 0x03FF) as usize];
                }
                if address >= 0x0400 && address <= 0x07FF {
                    return self.table_name[0][(address & 0x03FF) as usize];
                }
                if address >= 0x0800 && address <= 0x0BFF {
                    return self.table_name[1][(address & 0x03FF) as usize];
                }
                if address >= 0x0C00 && address <= 0x0FFF {
                    return self.table_name[1][(address & 0x03FF) as usize];
                }
            }

            return 0;
        }

        if address >= 0x3F00 && address <= 0x3FFF {
            address &= 0x001F;
            if address == 0x0010 {
                address = 0x0000;
            }
            if address == 0x0014 {
                address = 0x0004;
            }
            if address == 0x0018 {
                address = 0x0008;
            }
            if address == 0x001C {
                address = 0x000C;
            }

            return self.table_palette[address as usize]
                & (if self.mask.get_grayscale() {
                    0x30
                } else {
                    0x3F
                });
        }

        0
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) {
        let mut address = addr & 0x3FFF;

        if self.chr_rom.write(address, data) {
            return;
        }

        if address <= 0x1FFF {
            self.table_pattern[((address & 0x1000) >> 12) as usize][(address & 0x0FFF) as usize] =
                data;
            return;
        }

        if address >= 0x2000 && address <= 0x3EFF {
            address &= 0x0FFF;

            if let Mirror::Vertical = self.chr_rom.mirror {
                // Vertical
                if address <= 0x03FF {
                    self.table_name[0][(address & 0x03FF) as usize];
                }
                if address >= 0x0400 && address <= 0x07FF {
                    self.table_name[1][(address & 0x03FF) as usize] = data;
                }
                if address >= 0x0800 && address <= 0x0BFF {
                    self.table_name[0][(address & 0x03FF) as usize] = data;
                }
                if address >= 0x0C00 && address <= 0x0FFF {
                    self.table_name[1][(address & 0x03FF) as usize] = data;
                }
                return;
            }

            if let Mirror::Horizontal = self.chr_rom.mirror {
                // Horizontal
                if address <= 0x03FF {
                    self.table_name[0][(address & 0x03FF) as usize] = data;
                }
                if address >= 0x0400 && address <= 0x07FF {
                    self.table_name[0][(address & 0x03FF) as usize] = data;
                }
                if address >= 0x0800 && address <= 0x0BFF {
                    self.table_name[1][(address & 0x03FF) as usize] = data;
                }
                if address >= 0x0C00 && address <= 0x0FFF {
                    self.table_name[1][(address & 0x03FF) as usize] = data;
                }
            }
        }

        if address >= 0x3F00 && address <= 0x3FFF {
            address &= 0x001F;
            if address == 0x0010 {
                address = 0x0000;
            }
            if address == 0x0014 {
                address = 0x0004;
            }
            if address == 0x0018 {
                address = 0x0008;
            }
            if address == 0x001C {
                address = 0x000C;
            }

            self.table_palette[address as usize] = data;
        }
    }

    pub fn oam_read(&mut self, addr: u8) -> u8 {
        let prop = addr & 0x3; // a struct tem 4 propriedades, então vamos usar os dois ultimo bits (0x3 = 0b11)
        let index = (addr / 4) as usize; // obtendo a posicao no array

        match prop {
            // y
            0 => self.oam[index].y,
            // id
            1 => self.oam[index].id,
            // attribute
            2 => self.oam[index].attribute,
            // x
            3 => self.oam[index].x,
            _ => 0,
        }
    }

    pub fn oam_write(&mut self, addr: u8, data: u8) {
        let prop = addr & 0x3; // a struct tem 4 propriedades, então vamos usar os dois ultimo bits (0x3 = 0b11)
        let index = (addr / 4) as usize; // obtendo a posicao no array

        match prop {
            // y
            0 => {
                self.oam[index].y = data;
            }
            // id
            1 => {
                self.oam[index].id = data;
            }
            // attribute
            2 => {
                self.oam[index].attribute = data;
            }
            // x
            3 => {
                self.oam[index].x = data;
            }
            _ => {}
        }
    }
}
