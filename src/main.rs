extern crate bitbit;
extern crate iui;
extern crate logos;
extern crate png;
extern crate fltk;
extern crate fltk_sys;

use std::borrow::Cow;
use std::env::args;
use std::error::Error;
use std::{fs, thread};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

// use iui::controls::{Button, Group, Label, VerticalBox};
// use iui::prelude::*;
use fltk::{button::*, frame::*, group::*, input::*, output::*, valuator::*, window::*};

use crate::tileset::{parse_metatile_config, Tile, TileStorage};
use fltk::dialog::{FileDialog, FileDialogType};

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
                    storage.add_image(Cow::from(tileset)).expect("failed to add tileset to storage");
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
                fs::remove_file(&path);
                let mut file = File::create(path).expect("failed to create metatiles.bin file");
                file.write_all(&metatiles);
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
        main_ui()?;
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

fn main_tileset() {
    let mut nature_storage = TileStorage::new("nature".to_string(), true);
    nature_storage.add_image(Cow::from("tree_tileset.png")).unwrap();
    nature_storage.add_image(Cow::from("tileset_input_red.png")).unwrap();
    nature_storage.add_image(Cow::from("water_tileset.png")).unwrap();
    nature_storage.add_image(Cow::from("grass_tileset.png")).unwrap();
    nature_storage.add_image(Cow::from("rock_tileset.png")).unwrap();
    nature_storage.output();

    let mut city_storage = TileStorage::new("city".to_string(), false);
    city_storage.add_image(Cow::from("houses_tileset.png")).unwrap();
    city_storage.add_image(Cow::from("gym_tileset.png")).unwrap();
    city_storage.output();

    println!("Success!\nNature Tiles: {}\nCity Tiles: {}", nature_storage.tiles.len(), city_storage.tiles.len());
}

fn main_ui() -> Result<(), String> {
    let app = fl::App::default().set_scheme(AppScheme::Base);
    let (screen_width, screen_height) = fl::screen_size();
    let (app_width, app_height) = (900, 700);
    let mut wind = Window::new(
        (screen_width / 2.0 - (app_width / 2) as f64) as i32,
        (screen_height / 2.0 - (app_height / 2) as f64) as i32,
        app_width,
        app_height,
        &format!("Pokerus v{}", VERSION),
    );
    let tab_set = Tabs::new(10, 10, app_width - 20, app_height - 20, "");

    {
        let tileset_tab = Group::new(10, 35, app_width - 20, app_height - 20, "Tileset Merger");
        tileset_tab.end();
    }

    {
        let palette_tab = Group::new(10, 35, app_width - 20, app_height - 20, "Palette Extractor");
        let mut button = Button::new(60, 60, app_width - 120, app_height - 120,
                                     "Big Button to Extract a Palette from an indexed .png to a .pal\n\n\
        really that's it\n\n\
        look, gimp doesn't have a button for it so I made one\n\n\
        amazing tool by phase");

        button.set_callback(Box::new(|| {
            thread::spawn(|| {
                let mut image_dialog = FileDialog::new(FileDialogType::BrowseFile);
                image_dialog.set_filter("*.png\x00"); // 0 byte is required
                image_dialog.show(); // waits for dialog box to close
                image_dialog.set_title("Choose an indexed image file");
                let image_file = image_dialog.filename();
                let mut palette_dialog = FileDialog::new(FileDialogType::BrowseSaveFile);
                palette_dialog.show(); // waits for dialog box to close
                let pallete_file = palette_dialog.filename();
                println!("{} -> {}", image_file, pallete_file);
            });
        }));
        palette_tab.end();
    }

    tab_set.end();
    wind.show();
    app.run().map_err(|e| e.to_string())
}

/*
fn main_ui_libui() -> Result<(), String> {
    // Initialize the UI library
    let ui = UI::init().map_err(|e| e.to_string())?;
    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, &format!("Pokerus v{}", VERSION), 700, 600, WindowType::HasMenubar);

    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let mut group_vbox = VerticalBox::new(&ui);
    let mut group = Group::new(&ui, "Group");

    // Create two buttons to place in the window
    let mut button = Button::new(&ui, "Button");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Clicked!");
        }
    });

    let mut quit_button = Button::new(&ui, "Quit");
    quit_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });

    // Create a new label. Note that labels don't auto-wrap!
    let mut label_text = String::new();
    label_text.push_str("There is a ton of text in this label.\n");
    label_text.push_str("Pretty much every unicode character is supported.\n");
    label_text.push_str("🎉 用户界面 사용자 인터페이스");
    let label = Label::new(&ui, &label_text);

    vbox.append(&ui, label, LayoutStrategy::Stretchy);
    group_vbox.append(&ui, button, LayoutStrategy::Compact);
    group_vbox.append(&ui, quit_button, LayoutStrategy::Compact);
    group.set_child(&ui, group_vbox);
    vbox.append(&ui, group, LayoutStrategy::Compact);

    // Actually put the button in the window
    win.set_child(&ui, vbox);
    // Show the window
    win.show(&ui);
    // Run the application
    ui.main();
    Ok(())
}
*/