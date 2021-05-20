use crate::video::Pixel;

use super::Ppu2C02;

impl Ppu2C02 {
    // Increment the background tile "pointer" one tile/column horizontally
    pub fn increment_scroll_x(&mut self) {
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
    pub fn increment_scroll_y(&mut self) {
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
    pub fn transfer_address_x(&mut self) {
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
    pub fn transfer_address_y(&mut self) {
        // Ony if rendering is enabled
        if self.mask.get_render_background() || self.mask.get_render_sprites() {
            self.vram_addr.set_fine_y(self.tram_addr.get_fine_y());
            self.vram_addr
                .set_nametable_y(self.tram_addr.get_nametable_y());
            self.vram_addr.set_coarse_y(self.tram_addr.get_coarse_y());
        }
    }

    pub fn load_background_shifters(&mut self) {
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
    pub fn update_shifters(&mut self) {
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

    fn get_backgroud_pixel(&self) -> (u8, u8) {
        // We only render backgrounds if the PPU is enabled to do so. Note if
        // background rendering is disabled, the pixel and palette combine
        // to form 0x00. This will fall through the colour tables to yield
        // the current background colour in effect
        if self.mask.get_render_background() {
            if self.mask.get_render_background_left() || self.cycle >= 9 {
                // Handle Pixel Selection by selecting the relevant bit
                // depending upon fine x scolling. This has the effect of
                // offsetting ALL background rendering by a set number
                // of pixels, permitting smooth scrolling
                let bit_mux: u16 = 0x8000 >> self.fine_x;

                // Select Plane pixels by extracting from the shifter
                // at the required location.
                let lsb_pixel = ((self.bg_shifter_pattern_lo & bit_mux) > 0) as u8;
                let msb_pixel = ((self.bg_shifter_pattern_hi & bit_mux) > 0) as u8;

                // Combine to form pixel index
                let bg_pixel = (msb_pixel << 1) | lsb_pixel;

                // Get palette
                let lsb_palette = ((self.bg_shifter_attrib_lo & bit_mux) > 0) as u8;
                let msb_pallete = ((self.bg_shifter_attrib_hi & bit_mux) > 0) as u8;

                let bg_palette = (msb_pallete << 1) | lsb_palette;

                return (bg_pixel, bg_palette);
            }
        }
        return (0, 0);
    }

    fn get_foreground_pixel(&mut self) -> (u8, u8, bool) {
        // Foreground =============================================================
        // uint8_t fg_pixel = 0x00;   // The 2-bit pixel to be rendered
        // uint8_t fg_palette = 0x00; // The 3-bit index of the palette the pixel indexes
        // uint8_t fg_priority = 0x00;// A bit of the sprite attribute indicates if its
        // more important than the background
        if self.mask.get_render_sprites() {
            // Iterate through all sprites for this scanline. This is to maintain
            // sprite priority. As soon as we find a non transparent pixel of
            // a sprite we can abort
            if self.mask.get_render_sprites_left() || (self.cycle >= 9) {
                self.sprite_zero_being_rendered = false;

                for i in 0..self.sprite_count as usize {
                    // Scanline cycle has "collided" with sprite, shifters taking over
                    if self.sprite_scanline[i].x == 0 {
                        // Note Fine X scrolling does not apply to sprites, the game
                        // should maintain their relationship with the background. So
                        // we'll just use the MSB of the shifter

                        // Determine the pixel value...
                        let pixel_lsb = ((self.sprite_shifter_pattern_lo[i] & 0x80) > 0) as u8;
                        let pixel_msb = ((self.sprite_shifter_pattern_hi[i] & 0x80) > 0) as u8;
                        let fg_pixel = (pixel_msb << 1) | pixel_lsb;

                        // If pixel is not transparent, we render it, and dont
                        // bother checking the rest because the earlier sprites
                        // in the list are higher priority
                        if fg_pixel != 0 {
                            if i == 0
                            // Is this sprite zero?
                            {
                                self.sprite_zero_being_rendered = true;
                            }

                            // Extract the palette from the bottom two bits. Recall
                            // that foreground palettes are the latter 4 in the
                            // palette memory.
                            let fg_palette = (self.sprite_scanline[i].attribute & 0x03) + 0x04;
                            let fg_priority = (self.sprite_scanline[i].attribute & 0x20) == 0;

                            return (fg_pixel, fg_palette, fg_priority);
                        }
                    }
                }
            }
        }

        return (0, 0, false);
    }

    pub fn get_cycle_pixel(&mut self) -> (u8, u8) {
        let (bg_pixel, bg_palette) = self.get_backgroud_pixel();
        let (fg_pixel, fg_palette, fg_priority) = self.get_foreground_pixel();

        if bg_pixel == 0 && fg_pixel == 0 {
            // The background pixel is transparent
            // The foreground pixel is transparent
            // No winner, draw "background" colour
            return (0, 0);
        }

        if bg_pixel == 0 && fg_pixel > 0 {
            // The background pixel is transparent
            // The foreground pixel is visible
            // Foreground wins!
            return (fg_pixel, fg_palette);
        }

        if bg_pixel > 0 && fg_pixel == 0 {
            // The background pixel is visible
            // The foreground pixel is transparent
            // Background wins!
            return (bg_pixel, bg_palette);
        }

        if bg_pixel > 0 && fg_pixel > 0 {
            // Sprite Zero Hit detection
            if self.sprite_zero_hit_possible && self.sprite_zero_being_rendered {
                // Sprite zero is a collision between foreground and background
                // so they must both be enabled
                if self.mask.get_render_background() && self.mask.get_render_sprites() {
                    // The left edge of the screen has specific switches to control
                    // its appearance. This is used to smooth inconsistencies when
                    // scrolling (since sprites x coord must be >= 0)
                    if !(self.mask.get_render_background_left()
                        || self.mask.get_render_sprites_left())
                    {
                        if self.cycle >= 9 && self.cycle < 258 {
                            self.status.set_sprite_zero_hit(1);
                        }
                    } else {
                        if self.cycle >= 1 && self.cycle < 258 {
                            self.status.set_sprite_zero_hit(1);
                        }
                    }
                }
            }

            // The background pixel is visible
            // The foreground pixel is visible
            // Hmmm...
            if fg_priority {
                // Foreground cheats its way to victory!
                return (fg_pixel, fg_palette);
            } else {
                // Background is considered more important!
                return (bg_pixel, bg_palette);
            }
        }

        return (0, 0);
    }

    pub fn clear_sprite_scanline(&mut self) {
        for i in 0..8 {
            let sprite = &mut self.sprite_scanline[i];
            sprite.x = 0xFF;
            sprite.y = 0xFF;
            sprite.id = 0xFF;
            sprite.attribute = 0xFF;
        }
    }
}
