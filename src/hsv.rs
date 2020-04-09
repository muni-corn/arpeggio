use std::cmp::{Ord, Ordering};

#[derive(Clone, Debug, PartialEq)]
pub struct Hsv {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

impl Hsv {
    pub fn to_hex_string(&self) -> String {
        let rgb = self.to_rgb();
        let r = (rgb.0 * 255.0) as u8;
        let g = (rgb.1 * 255.0) as u8;
        let b = (rgb.2 * 255.0) as u8;

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    pub fn to_rgb(&self) -> (f32, f32, f32) {
        let f = |n| {
            let k = (n as f32 + self.hue / 60.0) % 6.0;
            let s = self.saturation;
            let v = self.value;

            v - (v * s * (0.0f32).max(k.min(4.0 - k).min(1.0)))
        };

        (f(5), f(3), f(1))
    }

    pub fn to_rgb_bytes(&self) -> (u8, u8, u8) {
        let rgb = self.to_rgb();

        let (r, g, b) = (rgb.0 * 255.0, rgb.1 * 255.0, rgb.2 * 255.0);

        (r.round() as u8, g.round() as u8, b.round() as u8)
    }

    pub fn from_rgb_bytes(r: u8, g: u8, b: u8) -> Self {
        // convert to i32 (prevents overflow issues)
        let r = r as i32;
        let g = g as i32;
        let b = b as i32;

        let max_component = r.max(g).max(b);
        let min_component = r.min(g).min(b);
        let chroma = max_component - min_component;

        let value = max_component as f32 / 255.0;

        let saturation = if max_component == 0 {
            0.0
        } else {
            (chroma as f32 / 255.0) / value as f32
        };

        let hue = if chroma == 0 {
            0.01
        } else if max_component == r {
            60.0 * (g - b) as f32 / chroma as f32
        } else if max_component == g {
            60.0 * (2.0 + ((b - r) as f32 / chroma as f32))
        } else if max_component == b {
            60.0 * (4.0 + ((r - g) as f32 / chroma as f32))
        } else {
            unreachable!()
        };

        if hue.is_nan() {
            panic!("hue was NaN when r={}, g={}, b={}", r, g, b);
        }

        Self {
            hue,
            saturation,
            value,
        }
    }
}

impl From<image::Rgba<u8>> for Hsv {
    fn from(rgba: image::Rgba<u8>) -> Self {
        Self::from_rgb_bytes(rgba[0], rgba[1], rgba[2])
    }
}

impl Ord for Hsv {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hue < other.hue {
            Ordering::Less
        } else if self.hue > other.hue {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hsv {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hsv {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_rgb_test() {
        let hsv = Hsv::from_rgb_bytes(100, 200, 250);
        assert_eq!(
            hsv.hue, 200f32,
            "conversion from rgb to hsv failed with hue: should've been 200, was {}",
            hsv.hue
        );
        assert_eq!(
            hsv.saturation, 0.60,
            "conversion from rgb to hsv failed with saturation: should've been 0.60, was {}",
            hsv.saturation
        );
        assert!(
            (hsv.value - 0.98) < 0.001,
            "conversion from rgb to hsv failed with value: should've been 0.98, was {}",
            hsv.value
        );

        let (r, g, b) = hsv.to_rgb_bytes();
        assert_eq!(
            r, 100,
            "conversion from hsv to rgb failed with r: should've been 100, was {}",
            r
        );
        assert_eq!(
            g, 200,
            "conversion from hsv to rgb failed with g: should've been 200, was {}",
            g
        );
        assert_eq!(
            b, 250,
            "conversion from hsv to rgb failed with b: should've been 250, was {}",
            b
        );
    }
}
