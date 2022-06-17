use std::{collections::HashMap, io::Write};

use clap::Parser;
use image::DynamicImage;
use log::{debug, info};
use palette::{rgb::Rgb, Lab};
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

#[derive(Deserialize, Serialize)]
struct Palette {
    colors: HashMap<ColorName, Rgb>,
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
    println!("my little function")
}
