use std::borrow::Cow;
use std::fs::File;
use std::{fs, io};
use std::io::{Write, BufWriter, ErrorKind};
use png::HasParameters;

#[derive(Eq, PartialEq)]
pub struct Tile {
    pub data: [[u8; 8]; 8]
}

impl Tile {
    pub fn blank() -> Tile {
        Tile {
            data: Default::default()
        }
    }

    pub fn new(data: [[u8; 8]; 8]) -> Tile {
        Tile {
            data
        }
    }

    pub fn flip_y(&self) -> Tile {
        Tile {
            data: [
                self.data[7],
                self.data[6],
                self.data[5],
                self.data[4],
                self.data[3],
                self.data[2],
                self.data[1],
                self.data[0]
            ]
        }
    }

    pub fn flip_x(&self) -> Tile {
        Tile {
            data: [
                Tile::reverse_row(self.data[0]),
                Tile::reverse_row(self.data[1]),
                Tile::reverse_row(self.data[2]),
                Tile::reverse_row(self.data[3]),
                Tile::reverse_row(self.data[4]),
                Tile::reverse_row(self.data[5]),
                Tile::reverse_row(self.data[6]),
                Tile::reverse_row(self.data[7]),
            ]
        }
    }

    fn reverse_row(row: [u8; 8]) -> [u8; 8] {
        [
            row[7],
            row[6],
            row[5],
            row[4],
            row[3],
            row[2],
            row[1],
            row[0]
        ]
    }

    /// returns (equivalent, flip_x, flip_y)
    pub fn is_equivalent(&self, other: &Tile) -> (bool, bool, bool) {
        if self.eq(&other) { return (true, false, false); }
        let other = other.flip_x();
        if self.eq(&other) { return (true, true, false); }
        let other = other.flip_y();
        if self.eq(&other) { return (true, true, true); }
        let other = other.flip_x();
        (self.eq(&other), false, true)
    }
}

pub struct TileStorage {
    pub tiles: Vec<Tile>,
    pub palettes: Vec<[[u8; 3]; 16]>,
    pub output_folder: String,
}

impl TileStorage {
    pub fn new(output_folder: String) -> TileStorage {
        fs::create_dir_all(&output_folder);
        let mut tiles = Vec::new();
        tiles.push(Tile::blank());
        TileStorage {
            tiles,
            palettes: Vec::new(),
            output_folder,
        }
    }

    pub fn add_image(&mut self, path: Cow<str>) -> Result<(), String> {
        let decoder = png::Decoder::new(match File::open(path.into_owned()) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string())
        });

        let (info, mut reader) = match decoder.read_info() {
            Ok(f) => f,
            Err(e) => return Err(e.to_string())
        };

        let mut buf = vec![0; info.buffer_size()];
        match reader.next_frame(&mut buf) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string())
        }

        let info = reader.info();
        let width = info.width;
        let height = info.height;
        if let Some(palette) = &info.palette {
            let mut indexed_palette: Vec<&[u8]> = Vec::with_capacity(palette.len() / 3);
            for color in palette.chunks(3) {
                indexed_palette.push(color);
            }

            // copy the values from the palette into storage
            let mut formatted_palette: [[u8; 3]; 16] = Default::default();
            for i in 0..16usize {
                if let Some(colors) = indexed_palette.get(i) {
                    let r = colors[0];
                    let g = colors[1];
                    let b = colors[2];
                    formatted_palette[i] = [r, g, b];
                }
            }
            self.add_palette(formatted_palette);

            // index the image by splitting it into chunks of (r, g, b) and finding it in the palette
            let mut indexed_image: Vec<u8> = Vec::with_capacity(buf.len());
            for color in buf.chunks(3) {
                let index = indexed_palette.iter().position(|&c| c == color).unwrap();
                indexed_image.push(index as u8);
            }

            let sections: Vec<&[u8]> = indexed_image.chunks(8).collect();
            let max_y = height / 8;
            let max_x = width / 8;

            for y in 0..max_y {
                for x in 0..max_x {
                    let mut tile_sections: [[u8; 8]; 8] = Default::default();
                    let start = x + y * max_x * 8;
                    for s in 0..8 {
                        let mut row: [u8; 8] = Default::default();
                        let row_index = (start + s * max_x) as usize;
                        let row_slice = sections[row_index];

                        row.copy_from_slice(row_slice);
                        tile_sections[s as usize] = row;
                    }
                    self.push(Tile::new(tile_sections));
                }
            }
            Ok(())
        } else {
            Err("Failed to find palette in png file".to_string())
        }
    }

    /// returns the id/index of the tile
    pub fn push(&mut self, tile: Tile) -> (usize, bool, bool) {
        for (i, other) in self.tiles.iter().enumerate() {
            let (equivalent, flip_x, flip_y) = other.is_equivalent(&tile);
            if equivalent {
                return (i, flip_x, flip_y);
            }
        }
        self.tiles.push(tile);
        return (self.tiles.len() - 1, false, false);
    }

    pub fn add_palette(&mut self, palette: [[u8; 3]; 16]) {
        self.palettes.push(palette);
    }

    pub fn read_palette(file_path: String) -> io::Result<[[u8; 3]; 16]> {
        let decoder = png::Decoder::new(File::open(file_path)?);
        let (info, mut reader) = decoder.read_info()?;
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf)?;
        let info = reader.info();
        if let Some(palette) = &info.palette {
            let mut indexed_palette: Vec<&[u8]> = Vec::with_capacity(palette.len() / 3);
            for color in palette.chunks(3) {
                indexed_palette.push(color);
            }

            // copy the values from the palette into storage
            let mut formatted_palette: [[u8; 3]; 16] = Default::default();
            for i in 0..16usize {
                if let Some(colors) = indexed_palette.get(i) {
                    let r = colors[0];
                    let g = colors[1];
                    let b = colors[2];
                    formatted_palette[i] = [r, g, b];
                }
            }
            return Ok(formatted_palette);
        }
        io::Result::Err(io::Error::new(ErrorKind::Other, "failed to extract palette"))
    }

    /// Output palette in .pal format
    pub fn output_palette(palette: &[[u8; 3]; 16], path: String) {
        fs::remove_file(&path); // ignore if fail
        let mut pal_file = File::create(path).unwrap();

        // this must be crlf for gbagfx
        let mut buffer = String::from("JASC-PAL\r\n0100\r\n16\r\n");
        for colors in palette.iter() {
            buffer.push_str(&format!("{} {} {}\r\n", colors[0], colors[1], colors[2]));
        }

        pal_file.write_all(buffer.as_ref());
    }

    pub fn output(&self) {
        for (i, palette) in self.palettes.iter().enumerate() {
            let pal_path = format!("{}/{}.pal", self.output_folder, i);
            TileStorage::output_palette(palette, pal_path);
        }
        let palette = self.palettes.get(0).unwrap(); // will never fail

        let width = 128u32;
        let height = 256u32;
        let max_x = width / 8;
        let max_y = height / 8;

        let tileset_path = format!("{}/tileset.png", self.output_folder);
        fs::remove_file(&tileset_path);
        let tileset_file = File::create(tileset_path).unwrap();
        let ref mut w = BufWriter::new(tileset_file);

        let mut buffer = Vec::with_capacity((width * height) as usize);
        let mut bit_writer = bitbit::BitWriter::new(&mut buffer);
        for y in 0..max_y {
            for row_index in 0..8 {
                for x in 0..max_x {
                    let tile_index = (y * max_x + x) as usize;
                    if let Some(tile) = self.tiles.get(tile_index) {
                        let row = tile.data[row_index];
                        for palette_index in row.iter() {
                            bit_writer.write_bits(*palette_index as u32, 4usize).unwrap();
                        }
                    } else {
                        // blank tile
                        for _ in 0..8 {
                            bit_writer.write_bits(0, 4usize).unwrap();
                        }
                    }
                }
            }
        }
        bit_writer.pad_to_byte();
        drop(bit_writer);

        let mut encoded_palette: Vec<u8> = Vec::with_capacity(palette.len() * 3);
        for [r, g, b] in palette.iter() {
            encoded_palette.push(*r);
            encoded_palette.push(*g);
            encoded_palette.push(*b);
        }

        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set(png::ColorType::Indexed).set(png::BitDepth::Four);
        encoder.set_palette(encoded_palette);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(buffer.as_slice());
    }

    pub fn dump_tiles(&self) {
        let mut i = 0usize;
        for tile in self.tiles.iter() {
            println!("Tile {}", i);
            for row in tile.data.iter() {
                println!("{:X?}", row)
            }
            i += 1;
        }
    }
}
