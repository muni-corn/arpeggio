use crate::errors::{ArpeggioError, ArpeggioFileError};
use crate::hsv::Hsv;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// Palette is a collection of colors. Each field is a tuple of two; a dark and light variant of
/// the color named (in that order).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Palette {
    #[serde(skip)]
    pub file_path: String,

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
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, ArpeggioFileError<ArpeggioError>> {
        println!("starting palette generation for {}", file_path.as_ref().display());
        let mut raw_pixels = match get_pixels_from_file(&file_path) {
            Ok(p) => p,
            Err(e) => {
                return Err(ArpeggioFileError {
                    file_path: file_path.as_ref().display().to_string(),
                    error: e,
                })
            }
        };

        // remove pixels without enough color, pixels too dark, or pixels too bright
        println!("\tfiltering colors");
        raw_pixels.retain(|c| c.saturation > 0.0);


        let mut px = raw_pixels;
        println!("\tsorting colors");
        px.sort_unstable();

        println!("\tmaking palette");

        let px_len = px.len();

        let reddishes = &px[0..px_len / 6];
        let yellowishes = &px[px_len / 6..px_len * 2 / 6];
        let greenishes=  &px[px_len * 2 / 6..px_len * 3 / 6];
        let tealishes = &px[px_len * 3 / 6..px_len * 4 / 6];
        let bluishes = &px[px_len * 4 / 6..px_len * 5 / 6];
        let purplishes = &px[px_len * 5 / 6..px_len];

        let (maroon, red) = get_average_color(reddishes);
        let (olive, yellow) = get_average_color(yellowishes);
        let (green, lime) = get_average_color(greenishes);
        let (teal, aqua) = get_average_color(tealishes);
        let (navy, blue) = get_average_color(bluishes);
        let (purple, magenta) = get_average_color(purplishes);

        let median_hsv = &px[px_len / 2];

        let (black, silver, gray, white) = get_shades(median_hsv);
        let (dark_accent, accent) = get_accent(median_hsv);

        println!("\tpalette generated");

        Ok(Self {
            file_path: String::from(file_path.as_ref().to_str().unwrap()),

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
    // blacks
    let black = Hsv {
        hue: median_hsv.hue,
        saturation: 0.3,
        value: 0.05,
    };
    let gray = Hsv {
        value: 0.15,
        ..black
    };

    // whites
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
    let mut avg_hx = 0f32;
    let mut avg_hy = 0f32;
    let mut avg_s = 0f32;
    let mut avg_v = 0f32;

    for (i, hsv) in vec.iter().enumerate() {
        let i = i as f32; // re-assign to an f32 version of itself
        let (x, y) = (hsv.hue.to_radians().cos(), hsv.hue.to_radians().sin());
        avg_hx = (avg_hx * i / (i + 1.0)) + (x / (i + 1.0));
        avg_hy = (avg_hy * i / (i + 1.0)) + (y / (i + 1.0));
        avg_s = (avg_s * i / (i + 1.0)) + (hsv.saturation / (i + 1.0));
        avg_v = (avg_v * i / (i + 1.0)) + (hsv.value / (i + 1.0));
    }

    let avg_h = (avg_hy / avg_hx).atan().to_degrees();

    let light = Hsv {
        hue: avg_h,
        saturation: avg_s,
        value: normalize_light_value(avg_v),
    };
    let dark = Hsv {
        value: normalize_dark_value(avg_v),
        ..light
    };

    (dark, light)
}

/// Constrains the original value so that it is greater than 0.6 but no greater than 1.
fn normalize_light_value(value: f32) -> f32 {
    (value * 0.3) + 0.7
}

/// Constrains the original value so that it is greater than 0.3 but no greater than 0.6.
fn normalize_dark_value(value: f32) -> f32 {
    (value * 0.3) + 0.4
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

fn get_pixels_from_file<P: AsRef<Path>>(file: P) -> Result<Vec<Hsv>, ArpeggioError> {
    // open the image
    println!("\topening image");
    let img = match image::io::Reader::open(file) {
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
    println!("\tgetting pixels");

    let all_px = img.pixels();
    let img_px_count = {
        let (w, h) = img.dimensions();
        w * h
    };

    // if the image is bigger than 800 pixels by 500, we step over extra pixels so that we only
    // read a total of 400,000
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
