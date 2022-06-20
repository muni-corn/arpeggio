use std::{collections::HashMap, io::Write};

use clap::Parser;
use image::{DynamicImage, GenericImageView, Pixel as ImagePixel};
use log::{debug, info};
use palette::{white_point::D65, ColorDifference, FromColor, Lab, Srgb};
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
        let colors = {
            let mut hm = HashMap::new();

            // insert shades
            hm.insert(ColorName::Black0, Srgb::new(0.0, 0.0, 0.0));
            hm.insert(
                ColorName::Black1,
                Srgb::new(1.0 / 7.0, 1.0 / 7.0, 1.0 / 7.0),
            );
            hm.insert(
                ColorName::Black2,
                Srgb::new(2.0 / 7.0, 2.0 / 7.0, 2.0 / 7.0),
            );
            hm.insert(
                ColorName::Black3,
                Srgb::new(3.0 / 7.0, 3.0 / 7.0, 3.0 / 7.0),
            );
            hm.insert(
                ColorName::White0,
                Srgb::new(4.0 / 7.0, 4.0 / 7.0, 4.0 / 7.0),
            );
            hm.insert(
                ColorName::White1,
                Srgb::new(5.0 / 7.0, 5.0 / 7.0, 5.0 / 7.0),
            );
            hm.insert(
                ColorName::White2,
                Srgb::new(6.0 / 7.0, 6.0 / 7.0, 6.0 / 7.0),
            );
            hm.insert(ColorName::White3, Srgb::new(1.0, 1.0, 1.0));

            // insert normal colors
            hm.insert(ColorName::Red, Srgb::new(1.0, 0.0, 0.0));
            hm.insert(ColorName::Orange, Srgb::new(1.0, 1.0 / 2.0, 0.0));
            hm.insert(ColorName::Yellow, Srgb::new(1.0, 1.0, 0.0));
            hm.insert(ColorName::Green, Srgb::new(0.0, 1.0, 0.0));
            hm.insert(ColorName::Cyan, Srgb::new(0.0, 1.0, 1.0));
            hm.insert(ColorName::Blue, Srgb::new(0.0, 0.0, 1.0));
            hm.insert(ColorName::Purple, Srgb::new(0.5, 0.0, 1.0));
            hm.insert(ColorName::Pink, Srgb::new(1.0, 0.0, 1.0));

            // insert dark colors
            hm.insert(ColorName::DarkRed, Srgb::new(0.5, 0.0, 0.0));
            hm.insert(ColorName::DarkOrange, Srgb::new(0.5, 0.25, 0.0));
            hm.insert(ColorName::DarkYellow, Srgb::new(0.5, 0.5, 0.0));
            hm.insert(ColorName::DarkGreen, Srgb::new(0.0, 0.5, 0.0));
            hm.insert(ColorName::DarkCyan, Srgb::new(0.0, 0.5, 0.5));
            hm.insert(ColorName::DarkBlue, Srgb::new(0.0, 0.0, 0.5));
            hm.insert(ColorName::DarkPurple, Srgb::new(0.25, 0.0, 0.5));
            hm.insert(ColorName::DarkPink, Srgb::new(0.5, 0.0, 0.5));

            hm.iter().map(|(k, v)| (*k, Lab::from_color(*v))).collect::<HashMap<ColorName, Lab<D65, f64>>>()
        };

        Self { colors }
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
