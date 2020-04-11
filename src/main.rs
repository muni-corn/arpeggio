mod config;
mod hsv;
mod io;
mod palette;
mod errors;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let config_dir = dirs::config_dir().expect("couldn't get config directory");
    let dir_path = config_dir.join("arpeggio");
    if let Err(e) = fs::create_dir_all(&dir_path) {
        eprintln!("couldn't create arpeggio's config directory: {}", e);
        return;
    }

    let seq_path = dir_path.join("sequences");
    let palettes_path = dir_path.join("palettes.toml");

    let mut palettes = if palettes_path.exists() {
        read_palettes(&palettes_path)
    } else {
        Default::default()
    };

    let img_file = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("an image file is required");
            return
        }
    };

    // make the palette from the image
    let palette = match palette::Palette::from_file(&Path::new(&img_file)) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("couldn't make palette from image: {}", e);
            return
        }
    };

    palettes.insert(img_file, palette.clone());

    write_sequences(&seq_path, &palette);

    write_palettes(&palettes_path, palettes);

    println!("done!");
}

fn read_palettes(path: &Path) -> config::Palettes {
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

fn write_sequences(path: &Path, palette: &palette::Palette) {
    // write out the colors to a sequences file
    println!("writing sequences");
    if let Err(e) = io::write_sequences(palette, path) {
        eprintln!("couldn't write sequences: {}", e);
        process::exit(1);
    }
}

fn write_palettes(path: &Path, palettes: config::Palettes) {
    let serialized = match toml::to_string(&palettes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("couldn't serialize palettes: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(path, serialized) {
        eprintln!("couldn't write palettes: {}", e);
        process::exit(1);
    }
}
