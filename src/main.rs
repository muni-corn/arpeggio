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
    let palettes_path = dir_path.join("palettes.json");

    let mut config = if palettes_path.exists() {
        read_config(&palettes_path)
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

    config.set_palette(img_file, palette.clone());

    write_sequences(&seq_path, &palette);

    write_config(&palettes_path, config);

    println!("done!");
}

fn read_config(path: &Path) -> config::Config {
    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("couldn't read config: {}", e);
            process::exit(1);
        }
    };

    match serde_json::from_str::<config::Config>(&raw) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("couldn't parse config: {}", e);
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

fn write_config(path: &Path, config: config::Config) {
    let serialized_config = match serde_json::to_string(&config) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("couldn't serialize config: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(path, serialized_config) {
        eprintln!("couldn't write palette: {}", e);
        process::exit(1);
    }
}
