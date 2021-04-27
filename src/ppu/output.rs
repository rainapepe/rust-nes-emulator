use super::Ppu2C02;
use crate::video::{get_color, Frame, Pixel};

impl Ppu2C02 {
    /* Essa função retorna a cor de um pixel em uma paleta especifica de cores */
    pub fn get_colour_from_palette_ram(&mut self, palette: u8, pixel: u8) -> Pixel {
        // "0x3F00"       - Offset do endereço na PPU que contém o range das paletas
        // "palette << 2" - Cada paleta tem 4 bytes de tamanho
        // "pixel"        - Cada pixel tem o index entre 0, 1, 2 or 3
        let color = self.ppu_read(0x3F00 + ((palette as u16) << 2) + pixel as u16);
        get_color(color)
    }

    pub fn get_screen<'a>(&'a self) -> &'a Frame {
        &self.sprite_screen
    }

    // This function draw the CHR ROM for a given pattern table into
    // an olc::Sprite, using a specified palette. Pattern tables consist
    // of 16x16 "tiles or characters". It is independent of the running
    // emulation and using it does not change the systems state, though
    // it gets all the data it needs from the live system. Consequently,
    // if the game has not yet established palettes or mapped to relevant
    // CHR ROM banks, the sprite may look empty. This approach permits a
    // "live" extraction of the pattern table exactly how the NES, and
    // ultimately the player would see it.

    // A tile consists of 8x8 pixels. On the NES, pixels are 2 bits, which
    // gives an index into 4 different colours of a specific palette. There
    // are 8 palettes to choose from. Colour "0" in each palette is effectively
    // considered transparent, as those locations in memory "mirror" the global
    // background colour being used. This mechanics of this are shown in
    // detail in ppuRead() & ppuWrite()

    // Characters on NES
    // ~~~~~~~~~~~~~~~~~
    // The NES stores characters using 2-bit pixels. These are not stored sequentially
    // but in singular bit planes. For example:
    //
    // 2-Bit Pixels       LSB Bit Plane     MSB Bit Plane
    // 0 0 0 0 0 0 0 0	  0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0
    // 0 1 1 0 0 1 1 0	  0 1 1 0 0 1 1 0   0 0 0 0 0 0 0 0
    // 0 1 2 0 0 2 1 0	  0 1 1 0 0 1 1 0   0 0 1 0 0 1 0 0
    // 0 0 0 0 0 0 0 0 =  0 0 0 0 0 0 0 0 + 0 0 0 0 0 0 0 0
    // 0 1 1 0 0 1 1 0	  0 1 1 0 0 1 1 0   0 0 0 0 0 0 0 0
    // 0 0 1 1 1 1 0 0	  0 0 1 1 1 1 0 0   0 0 0 0 0 0 0 0
    // 0 0 0 2 2 0 0 0	  0 0 0 1 1 0 0 0   0 0 0 1 1 0 0 0
    // 0 0 0 0 0 0 0 0	  0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0
    //
    // The planes are stored as 8 bytes of LSB, followed by 8 bytes of MSB
    pub fn get_pattern_table<'a>(&'a mut self, i: i8, palette: u8) -> &'a Frame {
        // Loop through all 16x16 tiles
        for tile_y in 0..16 {
            for tile_x in 0..16 {
                // Corverter a coordenada (x,y) do Tile para offset da pattern_table na memória
                // tile_y * 256 + tile_x * 16 = tile_y << 8 + tile_x << 4
                // o range de linhas de tile é 0x000..0xF00
                // 0 range de colunas de tile é 0x000..0x0FF
                let tile_offset: u16 = tile_y * 256 + tile_x * 16;

                // loop de 8 linhas de 8 pixels
                for row in 0..8 {
                    // criando o offset do pattern_table, são duas tabelas (0x0000 e 0x1000)
                    let table_offset: u16 = (i as u16) * 0x1000;

                    // Para cada linha vamos ler os dois planos de bit (LSB e MSB),
                    // como cada plano tem 8 bytes então o offset para o byte do plano MSB
                    // correspondente a linha que estamos lendo está 8 posições a frente
                    let mut tile_lsb = self.ppu_read(table_offset + tile_offset + row as u16);
                    let mut tile_msb =
                        self.ppu_read(table_offset + tile_offset + row as u16 + 0x0008); // add 8 para o MSB correspondente

                    // agora que temos a linha dos dois planos (LSB, MSB) podemos combinar para gerar o pixel final de 2bits (0,1,2,3)
                    for col in 0..8 {
                        // vamos calcular o pixel pelo LSB e mover 1 bit para a direita para cada leitura de coluna
                        // ou seja, estamos lendo da ultima coluna para a primeira
                        let pixel = (tile_msb & 0x01) << 1 | (tile_lsb & 0x01);

                        // movendo um bit para direita para a leitura da proxima coluna
                        tile_lsb >>= 1;
                        tile_msb >>= 1;

                        let color = self.get_colour_from_palette_ram(palette, pixel);

                        // Como sabemos que cada Tile é 8 pixel vamos multiplicar o tile_x e tile_y por 8
                        // Como estamos calculando da ultima coluna para primeira vamos inverter subtraindo o indece da coluna de 7
                        self.sprite_pattern_table[i as usize].set_pixel(
                            (tile_x as usize) * 8 + (7 - col),
                            (tile_y as usize) * 8 + row,
                            color,
                        )
                    }
                }
            }
        }

        &self.sprite_pattern_table[i as usize]
    }
}
