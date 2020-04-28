mod config;
mod errors;
mod flags;
mod hsv;
mod io;
mod palette;

use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let flags = flags::parse();

    // guard against bad flags first, before doing anything
    if flags.delete && flags.set {
        eprintln!("delete and set at the same time is not an operation arpeggio can do");
        return;
    }

    let config_dir = dirs::config_dir().expect("couldn't get config directory");
    let dir_path = config_dir.join("arpeggio");
    if !dir_path.exists() {
        if let Err(e) = fs::create_dir_all(&dir_path) {
            eprintln!("couldn't create arpeggio's config directory: {}", e);
            return;
        }
    }

    let palettes_path = dir_path.join("palettes.toml");

    let mut palettes = if palettes_path.exists() {
        read_palettes(&palettes_path)
    } else {
        Default::default()
    };


    if flags.delete {
        delete_palettes(&flags.image_files, &mut palettes);
    } else {
        make_palettes(&flags, &mut palettes);

        if flags.set {
            match flags.image_files.len().cmp(&1) {
                Ordering::Equal => {
                    let wallpaper = &flags.image_files[0]; // should be safe, since len is guaranteed to be 1 here
                    set_colors(dir_path, wallpaper.clone(), &palettes);
                }
                Ordering::Greater => {
                    println!("warning: multiple image files were passed using --set.");
                    println!(
                        "instead of just giving up, the last image file passed in will be used."
                    );
                    println!(
                        "this can always be changed by passing a single image file using --set."
                    );
                }
                _ => eprintln!("using --set requires an image"),
            }
        }
    }

    // finish
    write_palettes(&palettes_path, palettes);
}

fn read_palettes<P: AsRef<Path>>(path: P) -> config::Palettes {
    let raw = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("couldn't read palettes: {}", e);
            process::exit(1);
        }
    };

    match toml::from_slice::<config::Palettes>(&raw) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("couldn't parse palettes: {}", e);
            process::exit(1);
        }
    }
}

#[allow(dead_code)]
fn write_sequences<P: AsRef<Path>>(to: P, palette: &palette::Palette) {
    // write out the colors to a sequences file
    println!("writing sequences");
    if let Err(e) = io::write_sequences(palette, to) {
        eprintln!("couldn't write sequences: {}", e);
        process::exit(1);
    }
}

fn write_palettes<P: AsRef<Path>>(path: P, palettes: config::Palettes) {
    let serialized = match toml::to_string(&palettes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("couldn't serialize palettes: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(&path, serialized) {
        eprintln!("couldn't write palettes: {}", e);
        process::exit(1);
    }

    println!("palettes saved to {}", path.as_ref().display());
}

fn delete_palettes(image_files: &[String], palettes: &mut config::Palettes) {
    for file in image_files {
        if palettes.remove(file).is_some() {
            println!("removed palette for {}", file);
        } else {
            println!("no palette for {}; nothing done", file);
        }
    }
}

fn make_palettes(flags: &flags::Flags, palettes: &mut config::Palettes) {
    // keep track of any errors during the process. we do this so that any errors don't completely
    // stop the program, causing any remaining images to be skipped without trying.
    let mut errors = Vec::new();

    if flags.force {
        println!("forcing!");
    }

    // run all picture processing
    for file in &flags.image_files {
        // if palette is already made, skip it (unless forcing)
        if !flags.force && palettes.get(file).is_some() {
            errors.push(format!("palette for {} already exists", file));
            continue;
        }

        // make the palette from the image
        let palette = match palette::Palette::from_file(PathBuf::from(file)) {
            Ok(p) => p,
            Err(e) => {
                let error_str = format!("{}", e);
                errors.push(error_str);
                continue;
            }
        };

        palettes.insert(palette.file_path.clone(), palette.clone());
    }

    if errors.is_empty() {
        // no errors!
        println!("palettes were made successfully!");
    } else if errors.len() == flags.image_files.len() {
        // all errors
        eprintln!("not a single palette was generated:");

        for e in errors {
            eprintln!("\t{}", e);
        }

        // if only one file was passed in, display a suggestion
        if flags.image_files.len() == 1 && !flags.set && !flags.force {
            println!("\ndid you mean to --set? or --force? maybe --delete?");
        }
    } else {
        // some errors
        eprintln!("there were some errors during palette creation:");

        for e in errors {
            eprintln!("\t{}", e);
        }

        println!("\neverything else went smoothly");
    }
}

fn set_colors<P: AsRef<Path>>(dir_path: P, wallpaper_path: String, palettes: &config::Palettes) {
    let seq_path = dir_path.as_ref().join("sequences");
    write_sequences(seq_path, &palettes[&wallpaper_path]);
}
