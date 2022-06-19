use std::{collections::HashMap, io::Write};

use clap::Parser;
use image::DynamicImage;
use log::{debug, info};
use palette::{
    FromColor, Lab, Srgb, ColorDifference, white_point::D65,
};
use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Deserialize, Serialize)]
enum ColorName {
    Black0,
    Black1,
    Black2,
    Black3,
    White0,
    White1,
    White2,
    White3,
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Pink,
    DarkRed,
    DarkOrange,
    DarkYellow,
    DarkGreen,
    DarkCyan,
    DarkBlue,
    DarkPurple,
    DarkPink,
}

#[derive(Clone, Deserialize, Serialize)]
struct Palette {
    colors: HashMap<ColorName, Lab<D65, f64>>,
}

impl Default for Palette {
    fn default() -> Self {
        todo!()
    }
}

/// arpeggio generates base16 and terminal color schemes from images
#[derive(Parser)]
struct ArpeggioOpts {
    /// The path to the reference image
    #[clap(short, long)]
    input: String,

    /// The path to save palette output to
    #[clap(short, long, default_value = "arpeggio_palette.toml")]
    output: String,
}

fn main() {
    use image::io::Reader as ImageReader;

    simplelog::SimpleLogger::init(log::LevelFilter::Off, simplelog::Config::default()).unwrap();

    debug!("parsing options");
    let opts = ArpeggioOpts::parse();

    // open output file early so we can fail early if we need to
    let mut out_file =
        std::fs::File::open(&opts.output).expect("couldn't open output file for writing");

    info!("opening image '{}'", opts.input);
    let img = ImageReader::open(&opts.input).unwrap().decode().unwrap();

    info!("generating palette");
    let palette = make_palette(img);

    info!("writing palette to '{}'", opts.output);
    let toml_str = toml::to_string_pretty(&palette).expect("couldn't format palette as toml");
    write!(out_file, "{toml_str}").expect("couldn't write output to file");
}

fn make_palette(src_img: DynamicImage) -> Palette {
    todo!()
}
