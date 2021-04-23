use super::Ppu2C02;

impl Ppu2C02 {
    pub fn clock(&mut self) {}

    // Increment the background tile "pointer" one tile/column horizontally
    fn increment_scroll_x(&self) {
        // Note: pixel perfect scrolling horizontally is handled by the
        // data shifters. Here we are operating in the spatial domain of
        // tiles, 8x8 pixel blocks.

        // Ony if rendering is enabled
        if self.mask.get_render_background() || self.mask.get_render_sprites() {
            // A single name table is 32x30 tiles. As we increment horizontally
            // we may cross into a neighbouring nametable, or wrap around to
            // a neighbouring nametable
            if self.vram_addr.get_coarse_x() == 31 {
                // Leaving nametable so wrap address round
                self.vram_addr.set_coarse_x(0);

                // Flip target nametable bit
                self.vram_addr
                    .set_nametable_x(!self.vram_addr.get_nametable_x());
            } else {
                // Staying in current nametable, so just increment
                self.vram_addr
                    .set_coarse_x(self.vram_addr.get_coarse_x() + 1);
            }
        }
    }

    // Increment the background tile "pointer" one scanline vertically
    fn increment_scroll_y(&mut self) {
        // Incrementing vertically is more complicated. The visible nametable
        // is 32x30 tiles, but in memory there is enough room for 32x32 tiles.
        // The bottom two rows of tiles are in fact not tiles at all, they
        // contain the "attribute" information for the entire table. This is
        // information that describes which palettes are used for different
        // regions of the nametable.

        // In addition, the NES doesnt scroll vertically in chunks of 8 pixels
        // i.e. the height of a tile, it can perform fine scrolling by using
        // the fine_y component of the register. This means an increment in Y
        // first adjusts the fine offset, but may need to adjust the whole
        // row offset, since fine_y is a value 0 to 7, and a row is 8 pixels high

        // Ony if rendering is enabled
        if self.mask.get_render_background() || self.mask.get_render_sprites() {
            // If possible, just increment the fine y offset
            if self.vram_addr.get_fine_y() < 7 {
                self.vram_addr.set_fine_y(self.vram_addr.get_fine_y() + 1);
            } else {
                // If we have gone beyond the height of a row, we need to
                // increment the row, potentially wrapping into neighbouring
                // vertical nametables. Dont forget however, the bottom two rows
                // do not contain tile information. The coarse y offset is used
                // to identify which row of the nametable we want, and the fine
                // y offset is the specific "scanline"

                // Reset fine y offset
                self.vram_addr.set_fine_y(0);

                // Check if we need to swap vertical nametable targets
                if self.vram_addr.get_coarse_y() == 29 {
                    // We do, so reset coarse y offset
                    self.vram_addr.set_coarse_y(0);
                    // And flip the target nametable bit
                    self.vram_addr
                        .set_nametable_y(!self.vram_addr.get_nametable_y());
                } else if self.vram_addr.get_coarse_y() == 31 {
                    // In case the pointer is in the attribute memory, we
                    // just wrap around the current nametable
                    self.vram_addr.set_coarse_y(0);
                } else {
                    // None of the above boundary/wrapping conditions apply
                    // so just increment the coarse y offset
                    self.vram_addr
                        .set_coarse_y(self.vram_addr.get_coarse_y() + 1);
                }
            }
        }
    }

    // Transfer the temporarily stored horizontal nametable access information
    // into the "pointer". Note that fine x scrolling is not part of the "pointer"
    // addressing mechanism
    fn transfer_address_x(&mut self) {
        // Ony if rendering is enabled
        if self.mask.get_render_background() || self.mask.get_render_sprites() {
            self.vram_addr
                .set_nametable_x(self.tram_addr.get_nametable_x());
            self.vram_addr.set_coarse_x(self.tram_addr.get_coarse_x());
        }
    }

    // Transfer the temporarily stored vertical nametable access information
    // into the "pointer". Note that fine y scrolling is part of the "pointer"
    // addressing mechanism
    fn transfer_address_y(&mut self) {
        // Ony if rendering is enabled
        if self.mask.get_render_background() || self.mask.get_render_sprites() {
            self.vram_addr.set_fine_y(self.tram_addr.get_fine_y());
            self.vram_addr
                .set_nametable_y(self.tram_addr.get_nametable_y());
            self.vram_addr.set_coarse_y(self.tram_addr.get_coarse_y());
        }
    }

    fn load_background_shifters(&mut self) {
        // Each PPU update we calculate one pixel. These shifters shift 1 bit along
        // feeding the pixel compositor with the binary information it needs. Its
        // 16 bits wide, because the top 8 bits are the current 8 pixels being drawn
        // and the bottom 8 bits are the next 8 pixels to be drawn. Naturally this means
        // the required bit is always the MSB of the shifter. However, "fine x" scrolling
        // plays a part in this too, whcih is seen later, so in fact we can choose
        // any one of the top 8 bits.
        self.bg_shifter_pattern_lo =
            (self.bg_shifter_pattern_lo & 0xFF00) | self.bg_next_tile_lsb as u16;
        self.bg_shifter_pattern_hi =
            (self.bg_shifter_pattern_hi & 0xFF00) | self.bg_next_tile_msb as u16;

        // Attribute bits do not change per pixel, rather they change every 8 pixels
        // but are synchronised with the pattern shifters for convenience, so here
        // we take the bottom 2 bits of the attribute word which represent which
        // palette is being used for the current 8 pixels and the next 8 pixels, and
        // "inflate" them to 8 bit words.
        self.bg_shifter_attrib_lo = (self.bg_shifter_attrib_lo & 0xFF00)
            | (if self.bg_next_tile_attrib & 0b01 > 0 {
                0xFF
            } else {
                0x00
            });
        self.bg_shifter_attrib_hi = (self.bg_shifter_attrib_hi & 0xFF00)
            | (if self.bg_next_tile_attrib & 0b10 > 0 {
                0xFF
            } else {
                0x00
            });
    }

    // Every cycle the shifters storing pattern and attribute information shift
    // their contents by 1 bit. This is because every cycle, the output progresses
    // by 1 pixel. This means relatively, the state of the shifter is in sync
    // with the pixels being drawn for that 8 pixel section of the scanline.
    fn update_shifters(&mut self) {
        if self.mask.get_render_background() {
            // Shifting background tile pattern row
            self.bg_shifter_pattern_lo <<= 1;
            self.bg_shifter_pattern_hi <<= 1;

            // Shifting palette attributes by 1
            self.bg_shifter_attrib_lo <<= 1;
            self.bg_shifter_attrib_hi <<= 1;
        }

        if self.mask.get_render_sprites() && self.cycle >= 1 && self.cycle < 258 {
            for i in 0..self.sprite_count {
                if self.sprite_scanline[i as usize].x > 0 {
                    self.sprite_scanline[i as usize].x -= 1;
                } else {
                    self.sprite_shifter_pattern_lo[i as usize] <<= 1;
                    self.sprite_shifter_pattern_hi[i as usize] <<= 1;
                }
            }
        }
    }
}
