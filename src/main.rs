use std::{collections::HashMap, io::Write};

use clap::Parser;
use image::{DynamicImage, GenericImageView, Pixel as ImagePixel};
use log::{debug, info, trace, warn};
use palette::{white_point::D65, ColorDifference, FromColor, Lab, Srgb};
use rayon::prelude::*;
use serde::Serialize;

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

    fn as_str(&self) -> &'static str {
        match self {
            ColorName::Black0 => "Black0",
            ColorName::Black1 => "Black1",
            ColorName::Black2 => "Black2",
            ColorName::Black3 => "Black3",
            ColorName::White0 => "White0",
            ColorName::White1 => "White1",
            ColorName::White2 => "White2",
            ColorName::White3 => "White3",
            ColorName::Red => "Red",
            ColorName::Orange => "Orange",
            ColorName::Yellow => "Yellow",
            ColorName::Green => "Green",
            ColorName::Cyan => "Cyan",
            ColorName::Blue => "Blue",
            ColorName::Purple => "Purple",
            ColorName::Pink => "Pink",
            ColorName::DarkRed => "DarkRed",
            ColorName::DarkOrange => "DarkOrange",
            ColorName::DarkYellow => "DarkYellow",
            ColorName::DarkGreen => "DarkGreen",
            ColorName::DarkCyan => "DarkCyan",
            ColorName::DarkBlue => "DarkBlue",
            ColorName::DarkPurple => "DarkPurple",
            ColorName::DarkPink => "DarkPink",
        }
    }

    fn as_default_lab(&self) -> Lab<D65, f64> {
        let rgb = match self {
            // shades
            ColorName::Black0 => Srgb::new(0.0, 0.0, 0.0),
            ColorName::Black1 => Srgb::new(1.0 / 7.0, 1.0 / 7.0, 1.0 / 7.0),
            ColorName::Black2 => Srgb::new(2.0 / 7.0, 2.0 / 7.0, 2.0 / 7.0),
            ColorName::Black3 => Srgb::new(3.0 / 7.0, 3.0 / 7.0, 3.0 / 7.0),
            ColorName::White0 => Srgb::new(4.0 / 7.0, 4.0 / 7.0, 4.0 / 7.0),
            ColorName::White1 => Srgb::new(5.0 / 7.0, 5.0 / 7.0, 5.0 / 7.0),
            ColorName::White2 => Srgb::new(6.0 / 7.0, 6.0 / 7.0, 6.0 / 7.0),
            ColorName::White3 => Srgb::new(1.0, 1.0, 1.0),

            // normal colors
            ColorName::Red => Srgb::new(1.0, 0.0, 0.0),
            ColorName::Orange => Srgb::new(1.0, 1.0 / 2.0, 0.0),
            ColorName::Yellow => Srgb::new(1.0, 1.0, 0.0),
            ColorName::Green => Srgb::new(0.0, 1.0, 0.0),
            ColorName::Cyan => Srgb::new(0.0, 1.0, 1.0),
            ColorName::Blue => Srgb::new(0.0, 0.0, 1.0),
            ColorName::Purple => Srgb::new(0.5, 0.0, 1.0),
            ColorName::Pink => Srgb::new(1.0, 0.0, 1.0),

            // dark colors
            ColorName::DarkRed => Srgb::new(0.5, 0.0, 0.0),
            ColorName::DarkOrange => Srgb::new(0.5, 0.25, 0.0),
            ColorName::DarkYellow => Srgb::new(0.5, 0.5, 0.0),
            ColorName::DarkGreen => Srgb::new(0.0, 0.5, 0.0),
            ColorName::DarkCyan => Srgb::new(0.0, 0.5, 0.5),
            ColorName::DarkBlue => Srgb::new(0.0, 0.0, 0.5),
            ColorName::DarkPurple => Srgb::new(0.25, 0.0, 0.5),
            ColorName::DarkPink => Srgb::new(0.5, 0.0, 0.5),
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
        let colors = buckets.into_iter().map(|(color_name, bucket)| {
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
        }).collect();

        Self { colors }
    }

    /// Checks each available `ColorName`. If one is missing from this `Palette`'s `colors`, gets
    /// an existing color that is closest to the default (missing) color and copy it over.
    fn copy_missing_colors(&mut self) {
        ColorName::all().iter().for_each(|color_name| {
            if !self.colors.contains_key(color_name) {
                let new = if let Some((closest_color_name, closest_color)) = get_closest_palette_color(self, &color_name.as_default_lab()) {
                    warn!("copying {} to {}", closest_color_name.as_str(), color_name.as_str());
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
}

impl Default for Palette {
    fn default() -> Self {
        let colors = ColorName::all().iter().map(|color_name| (*color_name, color_name.as_default_lab())).collect();

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

/// Returns the name of the color that `color` is closest to in the `palette`
fn get_closest_palette_color(palette: &Palette, color: &Lab<D65, f64>) -> Option<(ColorName, Lab<D65, f64>)> {
    palette
        .colors
        .iter()
        .min_by(|(_, lab_1), (_, lab_2)| {
            lab_1
                .get_color_difference(color)
                .partial_cmp(&lab_2.get_color_difference(color))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(color_name, color)| (*color_name, *color))
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
            if let Some((closest_color_name, _)) = get_closest_palette_color(&centroids, &img_lab) {
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
