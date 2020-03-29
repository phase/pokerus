extern crate bitbit;
extern crate logos;
extern crate png;

use std::borrow::Cow;
use std::env::args;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

use crate::tileset::{parse_metatile_config, Tile, TileStorage};

mod rom;
mod tileset;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0); // remove program name
    match inner_main(args) {
        Ok(message) => {
            println!("{}", message);
            exit(0);
        }
        Err(message) => {
            print_help();
            println!("Failed previous command:\n{}", message);
            exit(1);
        }
    }
}

fn inner_main(args: Vec<String>) -> Result<String, String> {
    if let Some(arg) = args.get(0) {
        match arg.as_str() {
            "tileset" => {
                let primary = match args.get(1) {
                    Some(arg) => {
                        match arg.as_str() {
                            "primary" => { true }
                            "secondary" => { false }
                            _ => return Err("missing primary/secondary argument".to_string())
                        }
                    }
                    None => return Err("missing primary/secondary argument".to_string())
                };

                let output_path = match args.get(2) {
                    Some(arg) => { arg }
                    None => return Err("missing output folder".to_string())
                }.clone();

                // parse metatiles from file
                let metatile_definitions = match args.get(3) {
                    Some(arg) => {
                        let file = File::open(arg).expect("no such file");
                        let buf = BufReader::new(file);
                        let lines: Vec<String> = buf.lines()
                            .map(|l| l.expect("Could not parse line"))
                            .collect();
                        parse_metatile_config(lines)
                    }
                    None => return Err("missing metatile file".to_string())
                }.clone();

                if args.len() < 5 {
                    return Err("missing input tilesets".to_string());
                }
                let inputs = &args[4..];

                // add the tilesets to our storage
                let mut storage = TileStorage::new(output_path.clone(), primary);
                for tileset in inputs {
                    storage.add_image(tileset.clone()).expect("failed to add tileset to storage");
                }
                storage.output();
                // build the metatiles
                let mut metatiles: Vec<u8> = Vec::new();
                for (metatile_file_name, metatile_id) in metatile_definitions {
                    let metatile = storage.encoded_metatiles.get(&(metatile_file_name.clone(), metatile_id))
                        .expect(&format!("failed to get encoded metatile: {} {}", metatile_file_name, metatile_id)).clone();
                    metatiles.append(&mut metatile.clone());
                }

                let path = format!("{}/metatiles.bin", storage.output_folder);
                fs::remove_file(&path); // ignore
                let mut file = File::create(path).expect("failed to create metatiles.bin file");
                file.write_all(&metatiles).expect("failed to write metatiles to file");
                return Ok(format!("Tileset and palettes written to {}", output_path).to_string());
            }
            "palette" => {
                let image = match args.get(1) {
                    Some(arg) => { arg }
                    None => return Err("missing image file".to_string())
                }.clone();

                let output = match args.get(2) {
                    Some(arg) => { arg }
                    None => return Err("missing output file".to_string())
                }.clone();

                return match TileStorage::read_palette(image) {
                    Ok(palette) => {
                        TileStorage::output_palette(&palette, output.clone());
                        Ok(format!("Palette file written to {}", output).to_string())
                    }
                    Err(error) => Err(format!("error reading palette: {}", error.description()))
                };
            }
            _ => {
                print_help();
            }
        }
    } else {
        // no args passed, run gui
        // main_ui()?;
        print_help();
    }
    Ok("".to_string())
}

fn print_help() {
    println!("*.*.*.* Pokerus v{} *.*.*.* ", VERSION);
    println!("Available Commands:");
    println!("- pokerus");
    println!("    Launches the GUI. (WIP)");
    println!("- pokerus tileset <primary/secondary> <output_folder> <metatile_definitions> <input_images...>");
    println!("    Merges tilesets and their palettes into one image.");
    println!("    Useful for importing into Porymap.");
    println!("- pokerus palette <image> <output.pal>");
    println!("    Extract the palette of an image to a .pal file.");
}
