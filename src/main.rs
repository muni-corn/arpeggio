use std::{collections::HashMap, io::Write};

use clap::Parser;
use image::{DynamicImage, GenericImageView, Pixel as ImagePixel};
use log::{debug, info, trace, warn};
use palette::{white_point::D65, ColorDifference, FromColor, Lab, Oklch, Srgb};
use rayon::prelude::*;
use serde::Serialize;

#[derive(Eq, PartialEq)]
enum ColorType {
    /// A black, gray, or white color.
    Shade,

    /// A bright color with saturation.
    BrightColor,

    /// A dark color with saturation.
    DarkColor,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize)]
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

impl ColorName {
    fn all() -> Vec<Self> {
        vec![
            Self::Black0,
            Self::Black1,
            Self::Black2,
            Self::Black3,
            Self::White0,
            Self::White1,
            Self::White2,
            Self::White3,
            Self::Red,
            Self::Orange,
            Self::Yellow,
            Self::Green,
            Self::Cyan,
            Self::Blue,
            Self::Purple,
            Self::Pink,
            Self::DarkRed,
            Self::DarkOrange,
            Self::DarkYellow,
            Self::DarkGreen,
            Self::DarkCyan,
            Self::DarkBlue,
            Self::DarkPurple,
            Self::DarkPink,
        ]
    }

    fn color_type(&self) -> ColorType {
        match self {
            Self::Black0
            | Self::Black1
            | Self::Black2
            | Self::Black3
            | Self::White0
            | Self::White1
            | Self::White2
            | Self::White3 => ColorType::Shade,

            Self::Red
            | Self::Orange
            | Self::Yellow
            | Self::Green
            | Self::Cyan
            | Self::Blue
            | Self::Purple
            | Self::Pink => ColorType::BrightColor,

            Self::DarkRed
            | Self::DarkOrange
            | Self::DarkYellow
            | Self::DarkGreen
            | Self::DarkCyan
            | Self::DarkBlue
            | Self::DarkPurple
            | Self::DarkPink => ColorType::DarkColor,
        }
    }

    fn default_hue(&self) -> Option<f64> {
        let stop = match self {
            Self::Red | Self::DarkRed => Some(0),
            Self::Orange | Self::DarkOrange => Some(1),
            Self::Yellow | Self::DarkYellow => Some(2),
            Self::Green | Self::DarkGreen => Some(3),
            Self::Cyan | Self::DarkCyan => Some(4),
            Self::Blue | Self::DarkBlue => Some(5),
            Self::Purple | Self::DarkPurple => Some(6),
            Self::Pink | Self::DarkPink => Some(7),
            _ => return None, // shades have no hue
        };

        stop.map(|s| s as f64 * 45.0) // assuming 8 different stops; this goes around the cylinder
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Black0 => "black_0",
            Self::Black1 => "black_1",
            Self::Black2 => "black_2",
            Self::Black3 => "black_3",
            Self::White0 => "white_0",
            Self::White1 => "white_1",
            Self::White2 => "white_2",
            Self::White3 => "white_3",
            Self::Red => "red",
            Self::Orange => "orange",
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Cyan => "cyan",
            Self::Blue => "blue",
            Self::Purple => "purple",
            Self::Pink => "pink",
            Self::DarkRed => "dark_red",
            Self::DarkOrange => "dark_orange",
            Self::DarkYellow => "dark_yellow",
            Self::DarkGreen => "dark_green",
            Self::DarkCyan => "dark_cyan",
            Self::DarkBlue => "dark_blue",
            Self::DarkPurple => "dark_purple",
            Self::DarkPink => "dark_pink",
        }
    }

    fn as_default_lab(&self) -> Lab<D65, f64> {
        let l = |n| Oklch::<f64>::max_l() * n as f64 / 8.0;
        let h = |stop| stop as f64 * 45.0; // assuming 8 different stops

        let bright_l = Oklch::<f64>::max_l() * 0.9;
        let dark_l = Oklch::<f64>::max_l() * 0.7;
        let chroma = Oklch::<f64>::max_chroma() * 0.8;

        let rgb = match self {
            // shades
            ColorName::Black0 => Oklch::new(l(1), 0.0, 0.0),
            ColorName::Black1 => Oklch::new(l(2), 0.0, 0.0),
            ColorName::Black2 => Oklch::new(l(3), 0.0, 0.0),
            ColorName::Black3 => Oklch::new(l(4), 0.0, 0.0),
            ColorName::White0 => Oklch::new(l(5), 0.0, 0.0),
            ColorName::White1 => Oklch::new(l(6), 0.0, 0.0),
            ColorName::White2 => Oklch::new(l(7), 0.0, 0.0),
            ColorName::White3 => Oklch::new(l(8), 0.0, 0.0),

            // normal colors
            ColorName::Red => Oklch::new(bright_l, chroma, h(0)),
            ColorName::Orange => Oklch::new(bright_l, chroma, h(1)),
            ColorName::Yellow => Oklch::new(bright_l, chroma, h(2)),
            ColorName::Green => Oklch::new(bright_l, chroma, h(3)),
            ColorName::Cyan => Oklch::new(bright_l, chroma, h(4)),
            ColorName::Blue => Oklch::new(bright_l, chroma, h(5)),
            ColorName::Purple => Oklch::new(bright_l, chroma, h(6)),
            ColorName::Pink => Oklch::new(bright_l, chroma, h(7)),

            // dark colors
            ColorName::DarkRed => Oklch::new(dark_l, chroma, h(0)),
            ColorName::DarkOrange => Oklch::new(dark_l, chroma, h(1)),
            ColorName::DarkYellow => Oklch::new(dark_l, chroma, h(2)),
            ColorName::DarkGreen => Oklch::new(dark_l, chroma, h(3)),
            ColorName::DarkCyan => Oklch::new(dark_l, chroma, h(4)),
            ColorName::DarkBlue => Oklch::new(dark_l, chroma, h(5)),
            ColorName::DarkPurple => Oklch::new(dark_l, chroma, h(6)),
            ColorName::DarkPink => Oklch::new(dark_l, chroma, h(7)),
        };

        Lab::from_color(rgb)
    }
}

#[derive(Clone, Serialize)]
struct Palette {
    colors: HashMap<ColorName, Lab<D65, f64>>,
}

impl Palette {
    fn from_buckets(buckets: Buckets) -> Self {
        let colors = buckets
            .into_iter()
            .map(|(color_name, bucket)| {
                // compute average color by first adding up all components...
                let sums = bucket.iter().fold((0.0, 0.0, 0.0), |mut acc, color| {
                    acc.0 += color.l;
                    acc.1 += color.a;
                    acc.2 += color.b;
                    acc
                });

                // ...and then dividing by the amount of colors
                let (l, a, b) = (
                    sums.0 / bucket.len() as f64,
                    sums.1 / bucket.len() as f64,
                    sums.2 / bucket.len() as f64,
                );

                // now return the new averaged Lab color
                (color_name, Lab::<D65, f64>::from_components((l, a, b)))
            })
            .collect();

        Self { colors }
    }

    /// Checks each available `ColorName`. If one is missing from this `Palette`'s `colors`, gets
    /// an existing color that is closest to the default (missing) color and copy it over.
    fn copy_missing_colors(&mut self) {
        ColorName::all().iter().for_each(|color_name| {
            if !self.colors.contains_key(color_name) {
                let new = if let Some((closest_color_name, closest_color)) = self
                    .get_closest_palette_color_with_type(
                        color_name.color_type(),
                        &color_name.as_default_lab(),
                    ) {
                    warn!(
                        "copying {} to {}",
                        closest_color_name.as_str(),
                        color_name.as_str()
                    );
                    closest_color
                } else {
                    // if for some reason there just isn't a closest color, we'll default to the
                    // default color

                    color_name.as_default_lab()
                };
                self.colors.insert(*color_name, new);
            }
        })
    }

    fn as_strings(&self) -> HashMap<&str, String> {
        self.colors
            .iter()
            .map(|(k, v)| {
                let rgb = Srgb::from_color(*v);
                let red_byte = (rgb.red * 255.0) as u8;
                let green_byte = (rgb.green * 255.0) as u8;
                let blue_byte = (rgb.blue * 255.0) as u8;
                (
                    k.as_str(),
                    format!("#{red_byte:02x}{green_byte:02x}{blue_byte:02x}"),
                )
            })
            .collect()
    }

    /// Returns the name of the color that `color` is closest to in the palette.
    fn get_closest_palette_color(
        &self,
        color: &Lab<D65, f64>,
    ) -> Option<(ColorName, Lab<D65, f64>)> {
        self.colors
            .iter()
            .min_by(|(_, lab_1), (_, lab_2)| compare_distances_to_color(color, lab_1, lab_2))
            .map(|(color_name, color)| (*color_name, *color))
    }

    /// Returns the name of the color that `color` is closest to in the palette, except the set of
    /// possible colors is restricted to those colors that match the `color_type`.
    fn get_closest_palette_color_with_type(
        &self,
        color_type: ColorType,
        color: &Lab<D65, f64>,
    ) -> Option<(ColorName, Lab<D65, f64>)> {
        self.colors
            .iter()
            .filter(|(color_name, _)| color_name.color_type() == color_type)
            .min_by(|(_, lab_1), (_, lab_2)| compare_distances_to_color(color, lab_1, lab_2))
            .map(|(color_name, color)| (*color_name, *color))
    }
}

impl Default for Palette {
    fn default() -> Self {
        let colors = ColorName::all()
            .iter()
            .map(|color_name| (*color_name, color_name.as_default_lab()))
            .collect();

        Self { colors }
    }
}

/// arpeggio generates base16 and terminal color schemes from images
#[derive(Parser)]
struct ArpeggioOpts {
    /// The path to the reference image
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
        std::fs::File::create(&opts.output).expect("couldn't create output file for writing");

    info!("opening image '{}'", opts.input);
    let img = ImageReader::open(&opts.input).unwrap().decode().unwrap();

    info!("generating palette");
    let mut palette = make_palette(img, Palette::default());
    palette.copy_missing_colors();

    info!("writing palette to '{}'", opts.output);
    let toml_str =
        toml::to_string_pretty(&palette.as_strings()).expect("couldn't format palette as toml");
    write!(out_file, "{toml_str}").expect("couldn't write output to file");
}

/// Returns a `Palette` with colors from the image that most closely match the colors of the
/// default `Palette`.
fn get_initial_centroids(src_img: DynamicImage) -> Palette {
    let original_palette = Palette::default();

    src_img
        .pixels()
        // convert each image pixel into a Lab color (from the palette lib)
        .map(|(x, y, val)| {
            info!("mapping px {x}, {y}");

            let rgb = val.to_rgb();
            let rgb_slice = rgb.channels();
            let palette_srgb = Srgb::from_components((
                rgb_slice[0] as f64 / u8::MAX as f64,
                rgb_slice[1] as f64 / u8::MAX as f64,
                rgb_slice[2] as f64 / u8::MAX as f64,
            ));
            Lab::from_color(palette_srgb)
        })
        // iterate over each pixel to create a new palette that most closely matches the original
        // (default) palette using only colors from the image
        .fold(original_palette.clone(), |mut acc, img_pixel| {
            // for each color in the current palette...
            acc.colors.iter_mut().for_each(|(color_name, acc_color)| {
                if let Some(original_palette_color) = original_palette.colors.get(color_name) {
                    // get the source image pixel's color distance to the original palette
                    let img_pixel_diff = img_pixel.get_color_difference(original_palette_color);

                    // and the palette-so-far's color distance to the original palette
                    let acc_color_diff = acc_color.get_color_difference(original_palette_color);

                    // if the image pixel is closer to the original palette than the current
                    // centroid (acc) color...
                    if img_pixel_diff.abs() < acc_color_diff.abs() {
                        // ...update the centroid
                        *acc_color = img_pixel;
                    }
                }
            });

            acc
        })
}

type Bucket = Vec<Lab<D65, f64>>;
type Buckets = HashMap<ColorName, Bucket>;

/// Returns a palette made from the colors of the source image according to the centroids provided.
fn make_palette(src_img: DynamicImage, centroids: Palette) -> Palette {
    let buckets = src_img
        // get the pixels
        .pixels()
        // iterate in parallel
        .par_bridge()
        // map each image pixel into a `palette::Lab`
        .map(|(x, y, val)| {
            trace!("mapping px {x}, {y}");

            // convert the image rgb to the palette lib `Lab`
            let rgb = val.to_rgb();
            let rgb_slice = rgb.channels();
            let palette_srgb = Srgb::from_components((
                rgb_slice[0] as f64 / u8::MAX as f64,
                rgb_slice[1] as f64 / u8::MAX as f64,
                rgb_slice[2] as f64 / u8::MAX as f64,
            ));
            Lab::from_color(palette_srgb)
        })
        // fold into sets of buckets
        //
        // parallel `fold` is weird because it isn't guaranteed to yield exactly one result, so we
        // have to use `reduce` below to combine the series of `Buckets` into one `Buckets`
        .fold(Buckets::new, |mut buckets, img_lab| {
            // if we can get the name of the color this image pixel is closest to...
            if let Some((closest_color_name, _)) = centroids.get_closest_palette_color(&img_lab) {
                // ...update the bucket or insert a new one
                buckets
                    .entry(closest_color_name)
                    .or_insert_with(Vec::new)
                    .push(img_lab);
            }
            // we won't do anything if for some reason we can't get the closest color name

            // return the HashMap for the next iteration of `fold`
            buckets
        })
        // reduce the series of `Buckets` into one `Buckets` object
        .reduce(Buckets::new, |a, b| {
            a.into_iter().chain(b.into_iter()).fold(
                Buckets::new(),
                |mut acc, (color_name, mut bucket)| {
                    // if the color name entry already exists in `acc`, append this bucket.
                    // otherwise, initialize it with this bucket
                    acc.entry(color_name)
                        .or_insert_with(Bucket::new)
                        .append(&mut bucket);

                    acc
                },
            )
        });

    Palette::from_buckets(buckets)
}

fn compare_distances_to_color(
    target: &Lab<D65, f64>,
    lab_1: &Lab<D65, f64>,
    lab_2: &Lab<D65, f64>,
) -> std::cmp::Ordering {
    lab_1
        .get_color_difference(target)
        .partial_cmp(&lab_2.get_color_difference(target))
        .unwrap_or(std::cmp::Ordering::Equal)
}
