// pub const LUMINANCE_THRESHOLD: f64 = 0.03928;
pub const LUMINANCE_THRESHOLD: f64 = 0.04045;
pub const MINIMUM_CONTRAST_THRESHOLD: f64 = 4.5;
pub const MINIMUM_CONTRAST_THRESHOLD_LARGE_TEXT: f64 = 3.0;

// Y = 0.2126R + 0.7152G + 0.0722B
// Y: relative luminance
pub const RED: f64 = 0.2126;
pub const GREEN: f64 = 0.7152;
pub const BLUE: f64 = 0.0722;
pub const GAMMA: f64 = 2.4;

macro_rules! hex_u8 {
    ($hex:ident) => {{
        let c1 = $hex.next().unwrap_or('f');
        let c2 = $hex.next().unwrap_or('f');

        let mut c = String::new();
        c.push(c1);
        c.push(c2);

        u8::from_str_radix(&c, 16).unwrap()
    }};
}

/// An RGB color representation.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color(u8, u8, u8);

impl Color {
    /// Get a color from a hex string. (hashtag sign included)
    pub fn from_hex(hex: &str) -> Self {
        let mut hex = hex.chars();
        hex.next().unwrap(); // remove hashtag
        Self(hex_u8!(hex), hex_u8!(hex), hex_u8!(hex))
    }

    /// Get the luminance of a single color value.
    pub fn channel_luminance(x: u8) -> f64 {
        let x: f64 = (x / 255) as f64;

        if x <= LUMINANCE_THRESHOLD {
            x / 12.92
        } else {
            ((x as f64 + 0.055) / 1.055).powf(GAMMA)
        }
    }

    /// Get the luminance of the whole color.
    pub fn luminance(&self) -> f64 {
        Self::channel_luminance(self.0) * RED
            + Self::channel_luminance(self.1) * GREEN
            + Self::channel_luminance(self.2) * BLUE
    }

    /// Get the contrast ratio between this color and another color.
    pub fn contrast(&self, other: &Self) -> f64 {
        let s_lum = self.luminance();
        let o_lum = other.luminance();

        let bright = s_lum.max(o_lum);
        let dark = s_lum.min(o_lum);

        (bright + 0.05) / (dark + 0.05)
    }
}

#[cfg(test)]
mod test {
    use crate::Color;

    #[test]
    pub fn black_on_white() {
        let c1 = Color(255, 255, 255);
        let c2 = Color(0, 0, 0);

        assert_eq!(c1.contrast(&c2), 21.0);

        let c3 = Color::from_hex("#ffffff");
        let c4 = Color::from_hex("#000000");

        assert_eq!(c1, c3);
        assert_eq!(c2, c4);

        assert_eq!(c3.contrast(&c4), 21.0);
    }

    #[test]
    pub fn yellow_on_white() {
        let c1 = Color(255, 255, 255);
        let c2 = Color(255, 255, 0);

        assert_eq!(c1.contrast(&c2), 1.0738392309265699);
    }
}
