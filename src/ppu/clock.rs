use super::Ppu2C02;

// This little lambda function "flips" a byte
// so 0b11100000 becomes 0b00000111. It's very
// clever, and stolen completely from here:
// https://stackoverflow.com/a/2602885
fn flip_byte(value: u8) -> u8 {
    let mut temp = value;

    temp = (temp & 0xF0) >> 4 | (temp & 0x0F) << 4;
    temp = (temp & 0xCC) >> 2 | (temp & 0x33) << 2;
    temp = (temp & 0xAA) >> 1 | (temp & 0x55) << 1;

    return temp;
}

impl Ppu2C02 {
    pub fn clock(&mut self) {
        // All but 1 of the secanlines is visible to the user. The pre-render scanline
        // at -1, is used to configure the "shifters" for the first visible scanline, 0.
        if self.scanline >= -1 && self.scanline < 240 {
            // Background Rendering ======================================================
            if self.scanline == 0
                && self.cycle == 0
                && self.odd_frame
                && (self.mask.get_render_background() || self.mask.get_render_sprites())
            {
                // "Odd Frame" cycle skip
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 1 {
                // Effectively start of new frame, so clear vertical blank flag
                self.status.set_vertical_blank(0);
                // Clear sprite overflow flag
                self.status.set_sprite_overflow(0);
                // Clear the sprite zero hit flag
                self.status.set_sprite_zero_hit(0);
                // Clear Shifters
                for i in 0..8 {
                    self.sprite_shifter_pattern_lo[i as usize] = 0;
                    self.sprite_shifter_pattern_hi[i as usize] = 0;
                }
            }

            if (self.cycle >= 2 && self.cycle < 258) || (self.cycle >= 321 && self.cycle < 338) {
                self.update_shifters();

                // In these cycles we are collecting and working with visible data
                // The "shifters" have been preloaded by the end of the previous
                // scanline with the data for the start of this scanline. Once we
                // leave the visible region, we go dormant until the shifters are
                // preloaded for the next scanline.

                // Fortunately, for background rendering, we go through a fairly
                // repeatable sequence of events, every 2 clock cycles.
                match (self.cycle - 1) % 8 {
                    0 => {
                        // Load the current background tile pattern and attributes into the "shifter"
                        self.load_background_shifters();

                        // Fetch the next background tile ID
                        // "(vram_addr.reg & 0x0FFF)" : Mask to 12 bits that are relevant
                        // "| 0x2000"                 : Offset into nametable space on PPU address bus
                        self.bg_next_tile_id =
                            self.ppu_read(0x2000 | (self.vram_addr.reg & 0x0FFF));

                        // Explanation:
                        // The bottom 12 bits of the loopy register provide an index into
                        // the 4 nametables, regardless of nametable mirroring configuration.
                        // nametable_y(1) nametable_x(1) coarse_y(5) coarse_x(5)
                        //
                        // Consider a single nametable is a 32x32 array, and we have four of them
                        //   0                1
                        // 0 +----------------+----------------+
                        //   |                |                |
                        //   |                |                |
                        //   |    (32x32)     |    (32x32)     |
                        //   |                |                |
                        //   |                |                |
                        // 1 +----------------+----------------+
                        //   |                |                |
                        //   |                |                |
                        //   |    (32x32)     |    (32x32)     |
                        //   |                |                |
                        //   |                |                |
                        //   +----------------+----------------+
                        //
                        // This means there are 4096 potential locations in this array, which
                        // just so happens to be 2^12!
                    }
                    2 => {
                        // Fetch the next background tile attribute. OK, so this one is a bit
                        // more involved :P

                        // Recall that each nametable has two rows of cells that are not tile
                        // information, instead they represent the attribute information that
                        // indicates which palettes are applied to which area on the screen.
                        // Importantly (and frustratingly) there is not a 1 to 1 correspondance
                        // between background tile and palette. Two rows of tile data holds
                        // 64 attributes. Therfore we can assume that the attributes affect
                        // 8x8 zones on the screen for that nametable. Given a working resolution
                        // of 256x240, we can further assume that each zone is 32x32 pixels
                        // in screen space, or 4x4 tiles. Four system palettes are allocated
                        // to background rendering, so a palette can be specified using just
                        // 2 bits. The attribute byte therefore can specify 4 distinct palettes.
                        // Therefore we can even further assume that a single palette is
                        // applied to a 2x2 tile combination of the 4x4 tile zone. The very fact
                        // that background tiles "share" a palette locally is the reason why
                        // in some games you see distortion in the colours at screen edges.

                        // As before when choosing the tile ID, we can use the bottom 12 bits of
                        // the loopy register, but we need to make the implementation "coarser"
                        // because instead of a specific tile, we want the attribute byte for a
                        // group of 4x4 tiles, or in other words, we divide our 32x32 address
                        // by 4 to give us an equivalent 8x8 address, and we offset this address
                        // into the attribute section of the target nametable.

                        // Reconstruct the 12 bit loopy address into an offset into the
                        // attribute memory

                        // "(vram_addr.coarse_x >> 2)"        : integer divide coarse x by 4,
                        //                                      from 5 bits to 3 bits
                        // "((vram_addr.coarse_y >> 2) << 3)" : integer divide coarse y by 4,
                        //                                      from 5 bits to 3 bits,
                        //                                      shift to make room for coarse x

                        // Result so far: YX00 00yy yxxx

                        // All attribute memory begins at 0x03C0 within a nametable, so OR with
                        // result to select target nametable, and attribute byte offset. Finally
                        // OR with 0x2000 to offset into nametable address space on PPU bus.
                        self.bg_next_tile_attrib = self.ppu_read(
                            0x23C0
                                | ((self.vram_addr.get_nametable_y() as u16) << 11)
                                | ((self.vram_addr.get_nametable_x() as u16) << 10)
                                | (((self.vram_addr.get_coarse_y() as u16) >> 2) << 3)
                                | ((self.vram_addr.get_coarse_x() as u16) >> 2),
                        );

                        // Right we've read the correct attribute byte for a specified address,
                        // but the byte itself is broken down further into the 2x2 tile groups
                        // in the 4x4 attribute zone.

                        // The attribute byte is assembled thus: BR(76) BL(54) TR(32) TL(10)
                        //
                        // +----+----+			    +----+----+
                        // | TL | TR |			    | ID | ID |
                        // +----+----+ where TL =   +----+----+
                        // | BL | BR |			    | ID | ID |
                        // +----+----+			    +----+----+
                        //
                        // Since we know we can access a tile directly from the 12 bit address, we
                        // can analyse the bottom bits of the coarse coordinates to provide us with
                        // the correct offset into the 8-bit word, to yield the 2 bits we are
                        // actually interested in which specifies the palette for the 2x2 group of
                        // tiles. We know if "coarse y % 4" < 2 we are in the top half else bottom half.
                        // Likewise if "coarse x % 4" < 2 we are in the left half else right half.
                        // Ultimately we want the bottom two bits of our attribute word to be the
                        // palette selected. So shift as required...
                    }
                    4 => {
                        // Fetch the next background tile LSB bit plane from the pattern memory
                        // The Tile ID has been read from the nametable. We will use this id to
                        // index into the pattern memory to find the correct sprite (assuming
                        // the sprites lie on 8x8 pixel boundaries in that memory, which they do
                        // even though 8x16 sprites exist, as background tiles are always 8x8).
                        //
                        // Since the sprites are effectively 1 bit deep, but 8 pixels wide, we
                        // can represent a whole sprite row as a single byte, so offsetting
                        // into the pattern memory is easy. In total there is 8KB so we need a
                        // 13 bit address.

                        // "(control.pattern_background << 12)"  : the pattern memory selector
                        //                                         from control register, either 0K
                        //                                         or 4K offset
                        // "((uint16_t)bg_next_tile_id << 4)"    : the tile id multiplied by 16, as
                        //                                         2 lots of 8 rows of 8 bit pixels
                        // "(vram_addr.fine_y)"                  : Offset into which row based on
                        //                                         vertical scroll offset
                        // "+ 0"                                 : Mental clarity for plane offset
                        // Note: No PPU address bus offset required as it starts at 0x0000
                        self.bg_next_tile_lsb = self.ppu_read(
                            ((self.control.get_pattern_background() as u16) << 12)
                                + ((self.bg_next_tile_id as u16) << 4)
                                + (self.vram_addr.get_fine_y() as u16)
                                + 0,
                        );
                    }
                    6 => {
                        // Fetch the next background tile MSB bit plane from the pattern memory
                        // This is the same as above, but has a +8 offset to select the next bit plane
                        self.bg_next_tile_msb = self.ppu_read(
                            ((self.control.get_pattern_background() as u16) << 12)
                                + ((self.bg_next_tile_id as u16) << 4)
                                + (self.vram_addr.get_fine_y() as u16)
                                + 8,
                        );
                    }
                    7 => {
                        // Increment the background tile "pointer" to the next tile horizontally
                        // in the nametable memory. Note this may cross nametable boundaries which
                        // is a little complex, but essential to implement scrolling
                        // IncrementScrollX();
                        self.increment_scroll_x();
                    }
                    _ => {}
                }
            }

            // End of a visible scanline, so increment downwards...
            if self.cycle == 256 {
                self.increment_scroll_y();
            }

            //...and reset the x position
            if self.cycle == 257 {
                self.load_background_shifters();
                self.transfer_address_x();
            }

            // Superfluous reads of tile id at end of scanline
            if self.cycle == 338 || self.cycle == 340 {
                self.bg_next_tile_id = self.ppu_read(0x2000 | (self.vram_addr.reg & 0x0FFF));
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305 {
                // End of vertical blank period so reset the Y address ready for rendering
                self.transfer_address_y();
            }

            // Foreground Rendering ========================================================
            // I'm gonna cheat a bit here, which may reduce compatibility, but greatly
            // simplifies delivering an intuitive understanding of what exactly is going
            // on. The PPU loads sprite information successively during the region that
            // background tiles are not being drawn. Instead, I'm going to perform
            // all sprite evaluation in one hit. THE NES DOES NOT DO IT LIKE THIS! This makes
            // it easier to see the process of sprite evaluation.

            if self.cycle == 257 && self.scanline >= 0 {
                // We've reached the end of a visible scanline. It is now time to determine
                // which sprites are visible on the next scanline, and preload this info
                // into buffers that we can work with while the scanline scans the row.

                // Firstly, clear out the sprite memory. This memory is used to store the
                // sprites to be rendered. It is not the OAM.
                self.clear_sprite_scanline();

                // The NES supports a maximum number of sprites per scanline. Nominally
                // this is 8 or fewer sprites. This is why in some games you see sprites
                // flicker or disappear when the scene gets busy.
                self.sprite_count = 0;

                // Secondly, clear out any residual information in sprite pattern shifters
                for i in 0..8 {
                    self.sprite_shifter_pattern_lo[i] = 0;
                    self.sprite_shifter_pattern_hi[i] = 0;
                }

                // Thirdly, Evaluate which sprites are visible in the next scanline. We need
                // to iterate through the OAM until we have found 8 sprites that have Y-positions
                // and heights that are within vertical range of the next scanline. Once we have
                // found 8 or exhausted the OAM we stop. Now, notice I count to 9 sprites. This
                // is so I can set the sprite overflow flag in the event of there being > 8 sprites.
                let mut oam_entry: u8 = 0;

                // New set of sprites. Sprite zero may not exist in the new set, so clear this
                // flag.
                self.sprite_zero_hit_possible = false;

                while oam_entry < 64 && self.sprite_count < 9 {
                    // Note the conversion to signed numbers here
                    let diff = self.scanline - self.oam[oam_entry as usize].y as i16;

                    // If the difference is positive then the scanline is at least at the
                    // same height as the sprite, so check if it resides in the sprite vertically
                    // depending on the current "sprite height mode"
                    // FLAGGED
                    if diff >= 0
                        && diff
                            < (if self.control.get_sprite_size() > 1 {
                                16
                            } else {
                                8
                            })
                        && self.sprite_count < 8
                    {
                        // Sprite is visible, so copy the attribute entry over to our
                        // scanline sprite cache. Ive added < 8 here to guard the array
                        // being written to.
                        if self.sprite_count < 8 {
                            // Is this sprite sprite zero?
                            if oam_entry == 0 {
                                // It is, so its possible it may trigger a
                                // sprite zero hit when drawn
                                self.sprite_zero_hit_possible = true;
                            }

                            self.sprite_scanline[self.sprite_count as usize] =
                                self.oam[oam_entry as usize];
                        }
                        self.sprite_count += 1;
                    }
                    oam_entry += 1;
                } // End of sprite evaluation for next scanline

                // Set sprite overflow flag
                self.status
                    .set_sprite_overflow((self.sprite_count >= 8) as u8);

                // Now we have an array of the 8 visible sprites for the next scanline. By
                // the nature of this search, they are also ranked in priority, because
                // those lower down in the OAM have the higher priority.

                // We also guarantee that "Sprite Zero" will exist in spriteScanline[0] if
                // it is evaluated to be visible.
            }

            if self.cycle == 340 {
                // Now we're at the very end of the scanline, I'm going to prepare the
                // sprite shifters with the 8 or less selected sprites.

                for i in 0..self.sprite_count {
                    // We need to extract the 8-bit row patterns of the sprite with the
                    // correct vertical offset. The "Sprite Mode" also affects this as
                    // the sprites may be 8 or 16 rows high. Additionally, the sprite
                    // can be flipped both vertically and horizontally. So there's a lot
                    // going on here :P
                    let mut sprite_pattern_bits_lo: u8 = 0;
                    let mut sprite_pattern_bits_hi: u8 = 0;
                    let mut sprite_pattern_addr_lo: u16 = 0;
                    let mut sprite_pattern_addr_hi: u16 = 0;

                    // Determine the memory addresses that contain the byte of pattern data. We
                    // only need the lo pattern address, because the hi pattern address is always
                    // offset by 8 from the lo address.
                    if !(self.control.get_sprite_size() > 0) {
                        // 8x8 Sprite Mode - The control register determines the pattern table
                        if !((self.sprite_scanline[i as usize].attribute & 0x80) > 0) {
                            // Sprite is NOT flipped vertically, i.e. normal
                            sprite_pattern_addr_lo = ((self.control.get_pattern_sprite() as u16) << 12)  // Which Pattern Table? 0KB or 4KB offset
                                | ((self.sprite_scanline[i as usize].id as u16)   << 4)  // Which Cell? Tile ID * 16 (16 bytes per tile)
                                | (self.scanline as u16 - self.sprite_scanline[i as usize].y as u16);
                        // Which Row in cell? (0->7)
                        } else {
                            // Sprite is flipped vertically, i.e. upside down
                            sprite_pattern_addr_lo = ((self.control.get_pattern_sprite() as u16) << 12  )  // Which Pattern Table? 0KB or 4KB offset
                            | ((self.sprite_scanline[i as usize].id as u16) << 4)  // Which Cell? Tile ID * 16 (16 bytes per tile)
                            | (7 - (self.scanline as u16 - self.sprite_scanline[i as usize].y as u16));
                            // Which Row in cell? (7->0)
                        }
                    } else {
                        // 8x16 Sprite Mode - The sprite attribute determines the pattern table
                        if !((self.sprite_scanline[i as usize].attribute & 0x80) > 0) {
                            // Sprite is NOT flipped vertically, i.e. normal
                            if self.scanline as u8 - self.sprite_scanline[i as usize].y < 8 {
                                // Reading Top half Tile
                                sprite_pattern_addr_lo = (((self.sprite_scanline[i as usize].id as u16) & 0x01) << 12)  // Which Pattern Table? 0KB or 4KB offset
                                    | (((self.sprite_scanline[i as usize].id as u16) & 0xFE) << 4 )  // Which Cell? Tile ID * 16 (16 bytes per tile)
                                    | ((self.scanline as u16 - self.sprite_scanline[i as usize].y as u16) & 0x07);
                            // Which Row in cell? (0->7)
                            } else {
                                // Reading Bottom Half Tile
                                sprite_pattern_addr_lo = (((self.sprite_scanline[i as usize].id as u16) & 0x01) << 12)  // Which Pattern Table? 0KB or 4KB offset
                                    | ((((self.sprite_scanline[i as usize].id as u16) & 0xFE) + 1) << 4 )  // Which Cell? Tile ID * 16 (16 bytes per tile)
                                    | ((self.scanline as u16 - self.sprite_scanline[i as usize].y as u16) & 0x07);
                                // Which Row in cell? (0->7)
                            }
                        } else {
                            // Sprite is flipped vertically, i.e. upside down
                            if self.scanline as u8 - self.sprite_scanline[i as usize].y < 8 {
                                // Reading Top half Tile
                                sprite_pattern_addr_lo = (((self.sprite_scanline[i as usize].id as u16) & 0x01)      << 12)    // Which Pattern Table? 0KB or 4KB offset
                                    | ((((self.sprite_scanline[i as usize].id as u16) & 0xFE) + 1) << 4 )    // Which Cell? Tile ID * 16 (16 bytes per tile)
                                    | (7 - (self.scanline as u16 - self.sprite_scanline[i as usize].y as u16) & 0x07);
                            // Which Row in cell? (0->7)
                            } else {
                                // Reading Bottom Half Tile
                                sprite_pattern_addr_lo = (((self.sprite_scanline[i as usize].id as u16) & 0x01)       << 12)    // Which Pattern Table? 0KB or 4KB offset
                                    | (((self.sprite_scanline[i as usize].id as u16) & 0xFE)       << 4 )    // Which Cell? Tile ID * 16 (16 bytes per tile)
                                    | (7 - (self.scanline as u16 - self.sprite_scanline[i as usize].y as u16) & 0x07);
                                // Which Row in cell? (0->7)
                            }
                        }
                    }

                    // Phew... XD I'm absolutely certain you can use some fantastic bit
                    // manipulation to reduce all of that to a few one liners, but in this
                    // form it's easy to see the processes required for the different
                    // sizes and vertical orientations

                    // Hi bit plane equivalent is always offset by 8 bytes from lo bit plane
                    sprite_pattern_addr_hi = sprite_pattern_addr_lo + 8;

                    // if sprite_pattern_addr_lo > 0 || sprite_pattern_addr_hi > 0 {
                    //     println!("sprite_pattern_addr_lo {}", sprite_pattern_addr_lo);
                    //     println!("sprite_pattern_addr_hi {}", sprite_pattern_addr_hi);
                    // }

                    // Now we have the address of the sprite patterns, we can read them
                    sprite_pattern_bits_lo = self.ppu_read(sprite_pattern_addr_lo);
                    sprite_pattern_bits_hi = self.ppu_read(sprite_pattern_addr_hi);

                    // If the sprite is flipped horizontally, we need to flip the
                    // pattern bytes.
                    if (self.sprite_scanline[i as usize].attribute & 0x40) > 0 {
                        // Flip Patterns Horizontally
                        sprite_pattern_bits_lo = flip_byte(sprite_pattern_bits_lo);
                        sprite_pattern_bits_hi = flip_byte(sprite_pattern_bits_hi);
                    }

                    // Finally! We can load the pattern into our sprite shift registers
                    // ready for rendering on the next scanline
                    self.sprite_shifter_pattern_lo[i as usize] = sprite_pattern_bits_lo;
                    self.sprite_shifter_pattern_hi[i as usize] = sprite_pattern_bits_hi;
                }
            }
        }

        if self.scanline == 240 {
            // Post Render Scanline - Do Nothing!
        }

        if self.scanline >= 241 && self.scanline < 261 {
            if self.scanline == 241 && self.cycle == 1 {
                // Effectively end of frame, so set vertical blank flag
                self.status.set_vertical_blank(1);

                // If the control register tells us to emit a NMI when
                // entering vertical blanking period, do it! The CPU
                // will be informed that rendering is complete so it can
                // perform operations with the PPU knowing it wont
                // produce visible artefacts
                if self.control.get_enable_nmi() > 0 {
                    self.nmi = true;
                }
            }
        }

        let (pixel, palette) = self.get_cycle_pixel();
        let pixel = self.get_colour_from_palette_ram(palette, pixel);
        self.sprite_screen
            .set_pixel((self.cycle - 1) as usize, self.scanline as usize, pixel);

        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
                self.odd_frame = !self.odd_frame;
            }
        }
    }
}
