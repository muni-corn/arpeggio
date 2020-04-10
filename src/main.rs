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
    let img = match image::io::Reader::open(&PathBuf::from(img_file)) {
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

    // collect the pixels, converting each to hsv.
    // if the image is bigger than 800 pixels by 500, we step over extra pixels so that we only
    // read a total of 400,000
    println!("getting pixels");
    let px: Vec<_> = {
        let all_px = img.pixels();
        let img_px_count = {
            let (w, h) = img.dimensions();
            w * h
        };
        if img_px_count <= 800 * 500 {
            all_px
                .map(|p| Hsv::from(p.2))
                .collect()
        } else {
            all_px
                .step_by((img_px_count / (800 * 500)).try_into().unwrap())
                .map(|p| Hsv::from(p.2))
                .collect()
        }
    };

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
