mod hsv;
mod palette;

use hsv::Hsv;
use image::GenericImageView;
use std::env;
use std::path::PathBuf;

fn main() {
    let file = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("an image file is required");
            return;
        }
    };

    println!("opening image");
    let mut img = match image::io::Reader::open(&PathBuf::from(file)) {
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
    

    println!("making colors");
    let palette = palette::Palette::from(&px[..]);

    println!("{}", palette);
}
