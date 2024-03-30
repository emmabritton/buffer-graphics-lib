//! Buffer Graphics Lib
//!
//! This is a simple graphics library for drawing to a buffer, mainly designed to be used with [Rust Graphics Library](https://github.com/raybritton/rust-graphics-lib) or [Pixels](https://github.com/parasyte/pixels)
//!
//! It has basic shape drawing, bitmap text and image rendering.
//!
//! Using the library is as simple as:
//!```
//! # use graphics_shapes::rect::Rect;
//! # use buffer_graphics_lib::color::{BLUE, LIGHT_GRAY};
//! # use buffer_graphics_lib::drawable::{Drawable, stroke};
//! # use buffer_graphics_lib::Graphics;
//! # use buffer_graphics_lib::shapes::CreateDrawable;
//! # use buffer_graphics_lib::text::pos::TextPos;
//! # use buffer_graphics_lib::text::pos::NewTextPos;
//! # use buffer_graphics_lib::text::Text;
//! # use buffer_graphics_lib::text::TextSize::Large;
//! let mut buffer = [0_u8; 800 * 600 * 4]; //800 x 600 RGBA
//! let mut graphics = Graphics::new(&mut buffer, 800, 600).unwrap();
//! let text = Text::new("Some text", TextPos::cr((1,1)), (LIGHT_GRAY, Large));
//! graphics.draw(&text);
//! let drawable = Drawable::from_obj(Rect::new((1,1),(15,16)), stroke(BLUE));
//! graphics.draw(&drawable);
//! ```

extern crate core;

pub mod clipping;
pub mod color;
pub mod color_conversion;
pub mod drawable;
pub mod drawing;
pub mod image;
#[cfg(feature = "image_loading")]
pub mod image_loading;
#[cfg(feature = "ici")]
pub mod indexed;
pub mod indexed_color;
pub mod renderable_image;
pub mod renderable_macros;
pub mod scaling;
pub mod shapes;
pub mod text;
pub mod integration;

use crate::clipping::Clip;
use crate::GraphicsError::InvalidBufferLength;
use fnv::FnvHashMap;
use graphics_shapes::coord::Coord;
use thiserror::Error;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::color_conversion::*;
    pub use crate::drawable::*;
    pub use crate::drawing::*;
    pub use crate::image::*;
    #[cfg(feature = "image_loading")]
    pub use crate::image_loading::*;
    #[cfg(feature = "ici")]
    pub use crate::indexed::*;
    #[cfg(feature = "ici")]
    pub use crate::indexed_color::*;
    pub use crate::shapes::collection::*;
    pub use crate::shapes::polyline::*;
    pub use crate::shapes::*;
    pub use crate::text::format::*;
    pub use crate::text::pos::*;
    pub use crate::text::wrapping::*;
    pub use crate::text::*;
    pub use crate::Graphics;
    pub use crate::GraphicsError;
    pub use graphics_shapes::prelude::*;
    #[cfg(feature = "ici")]
    pub use ici_files::animated::*;
    #[cfg(feature = "ici")]
    pub use ici_files::errors::*;
    #[cfg(feature = "ici")]
    pub use ici_files::image::*;
    #[cfg(feature = "ici")]
    pub use ici_files::jasc_palette::*;
    #[cfg(feature = "ici")]
    pub use ici_files::palette::*;
    #[cfg(feature = "ici")]
    pub use ici_files::wrapper::*;
    #[cfg(feature = "ici")]
    pub use ici_files::*;
    #[cfg(feature = "image_loading")]
    pub use image_lib::ImageError;
    #[cfg(feature = "image_loading")]
    pub use image_lib::ImageFormat;
}

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("Invalid buffer length, expected: {0}, found: {1}")]
    InvalidBufferLength(usize, usize),
    #[error("Invalid pixel array length, expected: {0}, found: {1}")]
    ImageInitSize(usize, usize),
    #[error("Both images must be the same size, expected: {0}x{1}, found: {2}x{3}")]
    ImageBlendSize(usize, usize, usize, usize),
}

pub struct Graphics<'buffer> {
    buffer: &'buffer mut [u8],
    width: usize,
    height: usize,
    translate: Coord,
    clip: Clip,
    /// Allows you to replace any ASCII with a custom glyph
    /// To replace 'a' with '█' write
    ///
    /// `custom_font.insert(chr_to_code('a'), CustomLetter { small: [true; 20], ..CustomLetter::default });`
    ///
    /// Note: case is ignored
    ///
    /// Note: `a-z 0-9 !@$%^&*(),./;'\\[]<>?:\"{}_+~#…¤£¥¢✓` are valid for [text::chr_to_code]
    pub custom_font: FnvHashMap<u8, CustomLetter>,
}

/// Only the letter sizes you'll use need to be set
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CustomLetter {
    pub small: [bool; text::small::LETTER_PX_COUNT],
    pub normal: [bool; text::normal::LETTER_PX_COUNT],
    pub large: [bool; text::large::LETTER_PX_COUNT],
}

impl Default for CustomLetter {
    fn default() -> Self {
        Self {
            small: [false; text::small::LETTER_PX_COUNT],
            normal: [false; text::normal::LETTER_PX_COUNT],
            large: [false; text::large::LETTER_PX_COUNT],
        }
    }
}

impl<'buffer> Graphics<'_> {
    pub fn new(
        buffer: &'buffer mut [u8],
        width: usize,
        height: usize,
    ) -> Result<Graphics<'buffer>, GraphicsError> {
        let count = width * height * 4;
        if count != buffer.len() {
            return Err(InvalidBufferLength(count, buffer.len()));
        }
        Ok(Graphics {
            buffer,
            width,
            height,
            translate: Coord::default(),
            clip: Clip::new(width, height),
            custom_font: FnvHashMap::default(),
        })
    }

    pub fn new_unchecked(
        buffer: &'buffer mut [u8],
        width: usize,
        height: usize,
    ) -> Graphics<'buffer> {
        if cfg!(debug) {
            let count = width * height * 4;
            debug_assert_eq!(count, buffer.len());
        }
        Graphics {
            buffer,
            width,
            height,
            translate: Coord::default(),
            clip: Clip::new(width, height),
            custom_font: FnvHashMap::default(),
        }
    }
}

impl Graphics<'_> {
    pub fn set_clip(&mut self, clip: Clip) {
        self.clip = clip;
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }

    pub fn clip_mut(&mut self) -> &mut Clip {
        &mut self.clip
    }
}

pub trait Tint {
    /// Add to the RGBA channels by the amounts specified
    ///
    /// Channels are clamped to 0..=255
    fn tint_add(&mut self, r_diff: isize, g_diff: isize, b_diff: isize, a_diff: isize);
    /// Multiple the RGBA channels by the amounts specified
    ///
    /// Channels are clamped to 0..=255
    fn tint_mul(&mut self, r_diff: f32, g_diff: f32, b_diff: f32, a_diff: f32);
}
