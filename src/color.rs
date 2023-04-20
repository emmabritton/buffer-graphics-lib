use crate::Tint;

use crate::color::ColorError::InvalidHexFormat;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("Hex string has invalid format: {0}")]
    InvalidHexFormat(String),
}

///This represents an RGBA color and is used to store a pixel by [`Image`](crate::image::Image) and [`PixelsWrapper`](crate::drawing::PixelsWrapper)
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    //red channel
    pub r: u8,
    //green channel
    pub g: u8,
    //blue channel
    pub b: u8,
    //alpha channel
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color::rgba(0, 0, 0, 255)
    }
}

impl Color {
    /// Converts 0.0..=1.0 to 0..=255
    /// Values outside 0.0..=1.0 are clamped
    #[inline]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        let to_u8 = |value: f32| (value * 255.).round().clamp(0., 255.) as u8;
        Color::rgba(to_u8(r), to_u8(g), to_u8(b), to_u8(a))
    }

    /// Create new color with `alpha` set to 255
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::rgba(r, g, b, 255)
    }

    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// Create a new color with `red`, `green` and `blue` set to `value` and `alpha` set to 255
    #[inline]
    pub const fn gray(value: u8) -> Self {
        Color::rgb(value, value, value)
    }

    /// Convert an i32 into a [`Color`] where bytes match the format [R,G,B,A]
    #[inline]
    pub const fn from_i32(value: i32) -> Self {
        let bytes = value.to_be_bytes();
        Color::rgba(bytes[0], bytes[1], bytes[2], bytes[3])
    }

    /// Convert f32 array in the format [R,G,B,A] to color where 0.0 = 0, and 1.0 = 255
    #[inline]
    pub fn from_f32_array(array: [f32; 4]) -> Self {
        Color::new(array[0], array[1], array[2], array[3])
    }

    pub fn from_hex(hex: &str) -> Result<Color, ColorError> {
        let mut hex = hex.to_string();
        if hex.starts_with('#') {
            hex.remove(0);
        }
        if hex.chars().count() != 6 && hex.chars().count() != 8 {
            return Err(InvalidHexFormat("wrong length".to_string()));
        }
        if hex.chars().any(|c| !c.is_ascii_hexdigit()) {
            return Err(InvalidHexFormat("non hex digits".to_string()));
        }
        let chars: Vec<char> = hex.chars().collect();
        let mut colours = vec![];
        for digits in chars.chunks_exact(2) {
            let num = u8::from_str_radix(&format!("{}{}", digits[0], digits[1]), 16)
                .map_err(|e| InvalidHexFormat(e.to_string()))?;
            colours.push(num);
        }
        if colours.len() == 3 {
            colours.push(255);
        }
        Ok(Color {
            r: colours[0],
            g: colours[1],
            b: colours[2],
            a: colours[3],
        })
    }
}

impl Color {
    /// Split color into array in the format [R,G,B,A]
    #[inline]
    pub fn as_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Convert color to i32 in the format [R,G,B,A]
    #[inline]
    pub fn as_i32(&self) -> i32 {
        i32::from_be_bytes(self.as_array())
    }

    /// Convert color to f32 array in the format [R,G,B,A] where 0.0 = 0, and 1.0 = 255
    #[inline]
    pub fn as_f32_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    pub fn blend(&self, other: Color) -> Color {
        let base = self.as_f32_array();
        let added = other.as_f32_array();
        let mut mix = [0.0, 0.0, 0.0, 0.0];
        mix[3] = 1.0 - (1.0 - added[3]) * (1.0 - base[3]);
        mix[0] = (added[0] * added[3] / mix[3]) + (base[0] * base[3] * (1.0 - added[3]) / mix[3]);
        mix[1] = (added[1] * added[3] / mix[3]) + (base[1] * base[3] * (1.0 - added[3]) / mix[3]);
        mix[2] = (added[2] * added[3] / mix[3]) + (base[2] * base[3] * (1.0 - added[3]) / mix[3]);

        Color::from_f32_array(mix)
    }

    /// ignores alpha
    pub fn brightness(&self) -> f32 {
        let rgba = self.as_f32_array();
        0.2126 * rgba[0] + 0.7152 * rgba[1] + 0.0722 * rgba[2]
    }

    #[inline]
    pub fn is_dark(&self) -> bool {
        self.brightness() < 0.5
    }

    #[inline]
    pub fn is_transparent(&self) -> bool {
        self.a == 0
    }

    pub fn darken(&self) -> Color {
        self.with_brightness(0.9)
    }

    pub fn lighten(&self) -> Color {
        self.with_brightness(1.1)
    }

    /// Copy color with brightness
    pub fn with_brightness(&self, amount: f32) -> Color {
        let rgba = self.as_f32_array();
        Color::new(
            (rgba[0] * amount).min(1.0).max(0.0),
            (rgba[1] * amount).min(1.0).max(0.0),
            (rgba[2] * amount).min(1.0).max(0.0),
            rgba[3],
        )
    }

    /// De/saturate color by percentage
    /// Negative amount increases saturation
    pub fn with_saturate(&self, amount: f32) -> Color {
        let mut rgba = self.as_f32_array();
        let lum = 0.2989 * rgba[0] + 0.5870 * rgba[1] + 0.1140 * rgba[2];
        rgba[0] = rgba[0] + amount * (lum - rgba[0]);
        rgba[1] = rgba[1] + amount * (lum - rgba[1]);
        rgba[2] = rgba[2] + amount * (lum - rgba[2]);
        Color::from_f32_array(rgba)
    }

    /// Decrease saturation by 10%
    #[inline]
    pub fn desaturate(&self) -> Color {
        self.with_saturate(0.1)
    }

    /// Increase saturation by 10%
    #[inline]
    pub fn saturate(&self) -> Color {
        self.with_saturate(-0.1)
    }

    #[inline]
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

impl Tint for Color {
    #[inline]
    fn tint_add(&mut self, r_diff: isize, g_diff: isize, b_diff: isize, a_diff: isize) {
        let add = |current: u8, diff: isize| {
            let value = current as isize + diff;
            value.clamp(0, 255) as u8
        };
        self.r = add(self.r, r_diff);
        self.g = add(self.g, g_diff);
        self.b = add(self.b, b_diff);
        self.a = add(self.a, a_diff);
    }

    #[inline]
    fn tint_mul(&mut self, r_diff: f32, g_diff: f32, b_diff: f32, a_diff: f32) {
        let mul = |current: u8, diff: f32| {
            let value = current as f32 * diff;
            value.round().clamp(0., 255.) as u8
        };
        self.r = mul(self.r, r_diff);
        self.g = mul(self.g, g_diff);
        self.b = mul(self.b, b_diff);
        self.a = mul(self.a, a_diff);
    }
}

pub const WHITE: Color = Color::gray(255);
pub const BLACK: Color = Color::gray(0);
pub const DARK_GRAY: Color = Color::gray(75);
pub const LIGHT_GRAY: Color = Color::gray(180);
pub const RED: Color = Color::rgb(255, 0, 0);
pub const GREEN: Color = Color::rgb(0, 255, 0);
pub const BLUE: Color = Color::rgb(0, 0, 255);
pub const MAGENTA: Color = Color::rgb(255, 0, 255);
pub const YELLOW: Color = Color::rgb(255, 255, 0);
pub const ORANGE: Color = Color::rgb(255, 165, 0);
pub const BROWN: Color = Color::rgb(139, 69, 19);
pub const PURPLE: Color = Color::rgb(75, 0, 130);
pub const CYAN: Color = Color::rgb(0, 255, 255);
pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);

#[cfg(test)]
mod test {
    use super::*;

    fn clone_and_add(initial: Color, r: isize, g: isize, b: isize, a: isize) -> Color {
        let mut color = initial;
        color.tint_add(r, g, b, a);
        color
    }

    fn clone_and_mul(initial: Color, r: f32, g: f32, b: f32, a: f32) -> Color {
        let mut color = initial;
        color.tint_mul(r, g, b, a);
        color
    }

    #[test]
    fn constructors() {
        assert_eq!(
            Color::new(0., 0., 0., 1.),
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        );
        assert_eq!(
            Color::new(1., 0.5, 0., 1.),
            Color {
                r: 255,
                g: 128,
                b: 0,
                a: 255,
            }
        );
        assert_eq!(
            Color::new(1., 1., 1., 1.),
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            }
        );
        assert_eq!(
            Color::rgb(0, 0, 0),
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        );
        assert_eq!(
            Color::rgb(0, 15, 0),
            Color {
                r: 0,
                g: 15,
                b: 0,
                a: 255,
            }
        );
        assert_eq!(
            Color::rgb(0, 0, 255),
            Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255,
            }
        );
        assert_eq!(
            Color::rgba(0, 0, 0, 255),
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        );
        assert_eq!(
            Color::rgba(10, 22, 35, 255),
            Color {
                r: 10,
                g: 22,
                b: 35,
                a: 255,
            }
        );
        assert_eq!(
            Color::rgba(255, 255, 255, 255),
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            }
        );
        assert_eq!(
            Color::gray(255),
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            }
        );
        assert_eq!(
            Color::gray(55),
            Color {
                r: 55,
                g: 55,
                b: 55,
                a: 255,
            }
        );
    }

    #[test]
    fn tint_add() {
        let initial = Color {
            r: 100,
            g: 150,
            b: 200,
            a: 255,
        };
        assert_eq!(
            clone_and_add(initial, 50, 50, 50, 0),
            Color::rgba(150, 200, 250, 255)
        );
        assert_eq!(
            clone_and_add(initial, 100, 100, 100, 0),
            Color::rgba(200, 250, 255, 255)
        );
        assert_eq!(
            clone_and_add(initial, -100, -100, -100, 0),
            Color::rgba(0, 50, 100, 255)
        );
        assert_eq!(
            clone_and_add(initial, 0, 0, 0, 0),
            Color::rgba(100, 150, 200, 255)
        );
        assert_eq!(
            clone_and_add(initial, 10, 0, 0, 0),
            Color::rgba(110, 150, 200, 255)
        );
        assert_eq!(
            clone_and_add(initial, 0, 10, 0, 0),
            Color::rgba(100, 160, 200, 255)
        );
        assert_eq!(
            clone_and_add(initial, 0, 0, 10, 0),
            Color::rgba(100, 150, 210, 255)
        );
        assert_eq!(
            clone_and_add(initial, 0, 0, 0, -10),
            Color::rgba(100, 150, 200, 245)
        );
        assert_eq!(
            clone_and_add(initial, 0, 0, 0, -500),
            Color::rgba(100, 150, 200, 0)
        );
    }

    #[test]
    fn tint_mul() {
        let initial = Color {
            r: 100,
            g: 150,
            b: 200,
            a: 255,
        };
        assert_eq!(
            clone_and_mul(initial, 1., 1., 1., 1.),
            Color::rgba(100, 150, 200, 255)
        );
        assert_eq!(
            clone_and_mul(initial, 0., 0., 0., 0.),
            Color::rgba(0, 0, 0, 0)
        );
        assert_eq!(
            clone_and_mul(initial, 2., 2., 2., 2.),
            Color::rgba(200, 255, 255, 255)
        );
        assert_eq!(
            clone_and_mul(initial, 0.5, 0.5, 0.5, 0.5),
            Color::rgba(50, 75, 100, 128)
        );
    }

    #[test]
    fn blend() {
        assert_eq!(
            Color::rgb(255, 255, 255).blend(Color::rgba(0, 0, 0, 0)),
            Color::rgb(255, 255, 255)
        );
        assert_eq!(
            Color::rgb(255, 255, 255).blend(Color::rgb(255, 0, 0)),
            Color::rgb(255, 0, 0)
        );
        assert_eq!(
            Color::rgb(255, 255, 255).blend(Color::rgba(255, 0, 0, 128)),
            Color::rgb(255, 127, 127)
        );
        assert_eq!(
            Color::rgba(0, 0, 255, 128).blend(Color::rgba(255, 0, 0, 128)),
            Color::rgba(170, 0, 85, 192)
        );
    }

    #[test]
    fn from_hex() {
        assert_eq!(
            Color::from_hex("112233").unwrap(),
            Color {
                r: 17,
                g: 34,
                b: 51,
                a: 255,
            }
        );
        assert_eq!(
            Color::from_hex("#112233").unwrap(),
            Color {
                r: 17,
                g: 34,
                b: 51,
                a: 255,
            }
        );
        assert_eq!(
            Color::from_hex("#11223344").unwrap(),
            Color {
                r: 17,
                g: 34,
                b: 51,
                a: 68,
            }
        );
        assert!(Color::from_hex("#aafgha").is_err())
    }

    #[test]
    fn to_hex() {
        assert_eq!(WHITE.to_hex(), "#FFFFFFFF".to_string());
        assert_eq!(RED.to_hex(), "#FF0000FF".to_string());
    }

    #[test]
    fn brightness() {
        assert_eq!(WHITE.brightness(), 1.0);
        assert_eq!(BLACK.brightness(), 0.0);
        assert_eq!(Color::rgb(255, 0, 0).brightness(), 0.2126);
        assert_eq!(Color::rgb(123, 0, 0).brightness(), 0.102548234);
    }

    #[test]
    fn dark() {
        assert!(!WHITE.is_dark());
        assert!(BLACK.is_dark());
        assert!(!GREEN.is_dark());
        assert!(!CYAN.is_dark());
        assert!(RED.is_dark());
        assert!(DARK_GRAY.is_dark());
        assert!(!LIGHT_GRAY.is_dark());
    }
}
