mod hsv;

use image::GenericImageView;
use std::env;
use std::path::PathBuf;
use hsv::Hsv;

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
    let mut px: Vec<_> = img.pixels().map(|p| Hsv::from(p.2)).collect();
    px.sort();

    let px_len = px.len();

    println!("making colors");
    let fg_red = get_average_color(&px[0..px_len/6]);
    let fg_yellow = get_average_color(&px[px_len/6..px_len*2/6]);
    let fg_lime = get_average_color(&px[px_len*2/6..px_len*3/6]);
    let fg_aqua = get_average_color(&px[px_len*3/6..px_len*4/6]);
    let fg_blue = get_average_color(&px[px_len*4/6..px_len*5/6]);
    let fg_magenta = get_average_color(&px[px_len*5/6..px_len]);

    println!("red:\t\t{}", fg_red.to_hex_string());
    println!("yellow:\t\t{}", fg_yellow.to_hex_string());
    println!("lime:\t\t{}", fg_lime.to_hex_string());
    println!("aqua:\t\t{}", fg_aqua.to_hex_string());
    println!("blue:\t\t{}", fg_blue.to_hex_string());
    println!("magenta:\t{}", fg_magenta.to_hex_string());
}

fn get_average_color(vec: &[Hsv]) -> Hsv {
    let mut avg_h = 0f32;
    let mut avg_s = 0f32;

    for (i, hsv) in vec.iter().enumerate() {
        let i = i as f32; // re-assign to an f32 version of itself
        avg_h = (avg_h * i / (i + 1.0)) + (hsv.hue / (i + 1.0));
        if avg_h.is_nan() {
            println!("hsv: {:?}", hsv);
        }
        avg_s = (avg_s * i / (i + 1.0)) + (hsv.saturation / (i + 1.0));
    }

    let result = Hsv {
        hue: avg_h,
        saturation: avg_s,
        value: 0.9,
    };

    println!("average: h: {}, s: {}, v:{}", result.hue, result.saturation, result.value);

    result
}
