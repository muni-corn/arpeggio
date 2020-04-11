use crate::errors::ArpeggioError;
use crate::hsv::Hsv;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

/// Palette is a collection of colors. Each field is a tuple of two; a dark and light variant of
/// the color named (in that order).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Palette {
    pub maroon: Hsv,
    pub olive: Hsv,
    pub green: Hsv,
    pub teal: Hsv,
    pub navy: Hsv,
    pub purple: Hsv,

    pub red: Hsv,
    pub yellow: Hsv,
    pub lime: Hsv,
    pub aqua: Hsv,
    pub blue: Hsv,
    pub magenta: Hsv,

    pub black: Hsv,
    pub silver: Hsv,
    pub gray: Hsv,
    pub white: Hsv,

    pub dark_accent: Hsv,
    pub accent: Hsv,
}

impl Palette {
    pub fn from_file(file_path: &Path) -> Result<Self, ArpeggioError> {
        let raw_pixels = get_pixels_from_file(file_path)?;

        let mut px = raw_pixels;
        px.sort_unstable();

        println!("making colors for {}...", file_path.display());

        let px_len = px.len();
        let (maroon, red) = get_average_color(&px[0..px_len / 6]);
        let (olive, yellow) = get_average_color(&px[px_len / 6..px_len * 2 / 6]);
        let (green, lime) = get_average_color(&px[px_len * 2 / 6..px_len * 3 / 6]);
        let (teal, aqua) = get_average_color(&px[px_len * 3 / 6..px_len * 4 / 6]);
        let (navy, blue) = get_average_color(&px[px_len * 4 / 6..px_len * 5 / 6]);
        let (purple, magenta) = get_average_color(&px[px_len * 5 / 6..px_len]);

        let median_hsv = &px[px_len / 2];

        let (black, silver, gray, white) = get_shades(median_hsv);
        let (dark_accent, accent) = get_accent(median_hsv);

        println!("palette generated for {}", file_path.display());

        Ok(Self {
            maroon,
            olive,
            green,
            teal,
            navy,
            purple,

            red,
            yellow,
            lime,
            aqua,
            blue,
            magenta,

            black,
            silver,
            gray,
            white,

            dark_accent,
            accent,
        })
    }
}

impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "black:       {}", self.black.to_hex_string())?;
        writeln!(f, "maroon:      {}", self.red.to_hex_string())?;
        writeln!(f, "olive:       {}", self.yellow.to_hex_string())?;
        writeln!(f, "green:       {}", self.lime.to_hex_string())?;
        writeln!(f, "teal:        {}", self.aqua.to_hex_string())?;
        writeln!(f, "navy:        {}", self.blue.to_hex_string())?;
        writeln!(f, "purple:      {}", self.magenta.to_hex_string())?;
        writeln!(f, "silver:      {}", self.silver.to_hex_string())?;

        writeln!(f)?;

        writeln!(f, "gray:        {}", self.gray.to_hex_string())?;
        writeln!(f, "red:         {}", self.red.to_hex_string())?;
        writeln!(f, "yellow:      {}", self.yellow.to_hex_string())?;
        writeln!(f, "lime:        {}", self.lime.to_hex_string())?;
        writeln!(f, "aqua:        {}", self.aqua.to_hex_string())?;
        writeln!(f, "blue:        {}", self.blue.to_hex_string())?;
        writeln!(f, "magenta:     {}", self.magenta.to_hex_string())?;
        writeln!(f, "white:       {}", self.white.to_hex_string())?;

        writeln!(f)?;

        writeln!(f, "darkaccent:  {}", self.accent.to_hex_string())?;
        writeln!(f, "accent:      {}", self.accent.to_hex_string())?;
        Ok(())
    }
}

/// Returns (black, silver, gray, white)
fn get_shades(median_hsv: &Hsv) -> (Hsv, Hsv, Hsv, Hsv) {
    let black = Hsv {
        hue: median_hsv.hue,
        saturation: 0.3,
        value: 0.05,
    };
    let gray = Hsv {
        value: 0.2,
        ..black
    };
    let silver = Hsv {
        hue: median_hsv.hue,
        saturation: 0.1,
        value: 0.7,
    };
    let white = Hsv {
        value: 1.0,

        ..silver
    };

    (black, silver, gray, white)
}

/// Returns the average color of the colors in the Vec, returning a dark and light variant (in
/// that order).
fn get_average_color(vec: &[Hsv]) -> (Hsv, Hsv) {
    let mut avg_h = 0f32;
    let mut avg_s = 0f32;

    for (i, hsv) in vec.iter().enumerate() {
        let i = i as f32; // re-assign to an f32 version of itself
        avg_h = (avg_h * i / (i + 1.0)) + (hsv.hue / (i + 1.0));
        avg_s = (avg_s * i / (i + 1.0)) + (hsv.saturation / (i + 1.0));
    }

    let light = Hsv {
        hue: avg_h,
        saturation: avg_s,
        value: 1.0,
    };
    let dark = Hsv {
        value: 0.7,
        ..light
    };

    (dark, light)
}

fn get_accent(median_hsv: &Hsv) -> (Hsv, Hsv) {
    let dark = Hsv {
        hue: (median_hsv.hue + 60.0) % 360.0,
        saturation: 0.6,
        value: 0.7,
    };
    let light = Hsv {
        saturation: 0.7,
        value: 1.0,
        ..dark
    };

    (dark, light)
}

fn get_pixels_from_file(file: &Path) -> Result<Vec<Hsv>, ArpeggioError> {
    // open the image
    println!("opening {}...", file.display());
    let img = match image::io::Reader::open(&PathBuf::from(file)) {
        Ok(i) => match i.decode() {
            Ok(j) => j,
            Err(e) => {
                println!("couldn't decode image: {}", e);
                return Err(ArpeggioError::from(e));
            }
        },
        Err(e) => {
            println!("couldn't open image: {}", e);
            return Err(ArpeggioError::from(e));
        }
    };

    // collect the pixels, converting each to hsv.
    // if the image is bigger than 800 pixels by 500, we step over extra pixels so that we only
    // read a total of 400,000
    println!("getting pixels for {}...", file.display());

    let all_px = img.pixels();
    let img_px_count = {
        let (w, h) = img.dimensions();
        w * h
    };

    let result = if img_px_count <= 800 * 500 {
        all_px.map(|p| Hsv::from(p.2)).collect()
    } else {
        all_px
            .step_by((img_px_count / (800 * 500)).try_into().unwrap())
            .map(|p| Hsv::from(p.2))
            .collect()
    };

    Ok(result)
}
