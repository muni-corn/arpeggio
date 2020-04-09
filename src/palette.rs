use crate::hsv::Hsv;
use std::fmt::{Display, Formatter};
use std::fmt;

/// Palette is a collection of colors. Each field is a tuple of two; a dark and light variant of
/// the color named (in that order).
#[derive(Debug)]
pub struct Palette {
    red: (Hsv, Hsv),
    yellow: (Hsv, Hsv),
    lime: (Hsv, Hsv),
    aqua: (Hsv, Hsv),
    blue: (Hsv, Hsv),
    magenta: (Hsv, Hsv),

    shades: Shades,

    accent: (Hsv, Hsv),
}

impl From<&[Hsv]> for Palette {
    fn from(pixels: &[Hsv]) -> Self {
        let mut px = pixels.to_vec();
        px.sort_unstable();

        let px_len = px.len();

        let red = get_average_color(&px[0..px_len / 6]);
        let yellow = get_average_color(&px[px_len / 6..px_len * 2 / 6]);
        let lime = get_average_color(&px[px_len * 2 / 6..px_len * 3 / 6]);
        let aqua = get_average_color(&px[px_len * 3 / 6..px_len * 4 / 6]);
        let blue = get_average_color(&px[px_len * 4 / 6..px_len * 5 / 6]);
        let magenta = get_average_color(&px[px_len * 5 / 6..px_len]);

        let median_hsv = &px[px_len / 2];

        let shades = Shades::from(median_hsv);
        let accent = get_accent(median_hsv);

        Self {
            red,
            yellow,
            lime,
            aqua,
            blue,
            magenta,
            shades,
            accent
        }
    }
}

impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "black:       {}", self.shades.black.to_hex_string())?;
        writeln!(f, "maroon:      {}", self.red.0.to_hex_string())?;
        writeln!(f, "olive:       {}", self.yellow.0.to_hex_string())?;
        writeln!(f, "green:       {}", self.lime.0.to_hex_string())?;
        writeln!(f, "teal:        {}", self.aqua.0.to_hex_string())?;
        writeln!(f, "navy:        {}", self.blue.0.to_hex_string())?;
        writeln!(f, "purple:      {}", self.magenta.0.to_hex_string())?;
        writeln!(f, "silver:      {}", self.shades.silver.to_hex_string())?;

        writeln!(f)?;

        writeln!(f, "gray:        {}", self.shades.gray.to_hex_string())?;
        writeln!(f, "red:         {}", self.red.1.to_hex_string())?;
        writeln!(f, "yellow:      {}", self.yellow.1.to_hex_string())?;
        writeln!(f, "lime:        {}", self.lime.1.to_hex_string())?;
        writeln!(f, "aqua:        {}", self.aqua.1.to_hex_string())?;
        writeln!(f, "blue:        {}", self.blue.1.to_hex_string())?;
        writeln!(f, "magenta:     {}", self.magenta.1.to_hex_string())?;
        writeln!(f, "white:       {}", self.shades.white.to_hex_string())?;

        writeln!(f)?;

        writeln!(f, "darkaccent:  {}", self.accent.0.to_hex_string())?;
        writeln!(f, "accent:      {}", self.accent.1.to_hex_string())?;
        Ok(())
    }
}

#[derive(Debug)]
struct Shades {
    black: Hsv,
    silver: Hsv,
    gray: Hsv,
    white: Hsv,
}

impl From<&Hsv> for Shades {
    fn from(median_hsv: &Hsv) -> Self {
        let black = Hsv {
            hue: median_hsv.hue,
            saturation: 0.3,
            value: 0.1,
        };
        let gray = Hsv {
            value: 0.4,
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
        Self {
            black,
            silver,
            gray,
            white,
        }
    }
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
    let dark = Hsv { hue: (median_hsv.hue + 60.0) % 360.0, saturation: 0.6, value: 0.7 };
    let light = Hsv { saturation: 0.7, value: 1.0, ..dark };

    (dark, light)
}
