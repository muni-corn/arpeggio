mod hsv;
mod io;
mod palette;

use hsv::Hsv;
use image::GenericImageView;
use std::convert::TryInto;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let config_dir = dirs::config_dir().expect("couldn't get config directory");
    let dir_path = config_dir.join("arpeggio");
    if let Err(e) = fs::create_dir_all(&dir_path) {
        eprintln!("couldn't create arpeggio's config directory: {}", e);
        return;
    }

    let seq_path = dir_path.join("sequences");

    let img_file = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("an image file is required");
            return;
        }
    };

    // open the image
    println!("opening image");
    let mut img = match image::io::Reader::open(&PathBuf::from(img_file)) {
        Ok(i) => match i.decode() {
            Ok(j) => j,
            Err(e) => {
                println!("couldn't decode image: {}", e);
                return;
            }
        },
        Err(e) => {
            println!("couldn't open image: {}", e);
            return;
        }
    };

    println!("resizing image");
    img = img.resize_exact(800, 500, image::imageops::FilterType::Triangle);

    println!("getting pixels");
    let px: Vec<_> = img.pixels().map(|p| { 
        let hsv = Hsv::from(p.2);
        hsv
    }).collect();
    

    // make the palette from the image
    println!("making colors");
    let palette = palette::Palette::from(&px[..]);

    // write out the colors to a sequences file
    println!("writing sequences");
    if let Err(e) = io::write_sequences(&palette, &seq_path) {
        eprintln!("couldn't write sequences: {}", e);
        return;
    }

    println!("done!");
}
