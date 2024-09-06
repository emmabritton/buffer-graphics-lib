//! Buffer Graphics Lib
//!
//! This is a simple graphics library for drawing to a buffer, mainly designed to be used with [Rust Graphics Library](https://github.com/raybritton/rust-graphics-lib) or [Pixels](https://github.com/parasyte/pixels)
//!
//! It has basic shape drawing, bitmap text and image rendering.
//!
//! Using the library is as simple as:
//!```
//! # use graphics_shapes::rect::Rect;
//! # use buffer_graphics_lib::prelude::*;
//! let mut buffer = Graphics::create_buffer(800, 600); //800 x 600 RGBA
//! let mut graphics = Graphics::new(&mut buffer, 800, 600).unwrap();
//! let text = Text::new("Some text", TextPos::cr((1,1)), (LIGHT_GRAY, PixelFont::Standard6x7));
//! graphics.draw(&text);
//! graphics.draw_rect(Rect::new((40, 50), (100, 100)), stroke(BLUE));
//! ```

extern crate core;

pub mod clipping;
pub mod drawable;
pub mod drawing;
pub mod image;
#[cfg(feature = "image_loading")]
pub mod image_loading;
pub mod indexed;
pub mod integration;
pub mod renderable_image;
pub mod renderable_macros;
pub mod scaling;
pub mod shapes;
pub mod text;

use crate::clipping::Clip;
use crate::prelude::Image;
use crate::GraphicsError::InvalidBufferLength;
use fnv::FnvHashMap;
use graphics_shapes::coord::Coord;
use ici_files::errors::IndexedImageError;
use thiserror::Error;

pub mod prelude {
    pub use crate::drawable::*;
    pub use crate::drawing::*;
    pub use crate::image::*;
    #[cfg(feature = "image_loading")]
    pub use crate::image_loading::*;
    pub use crate::indexed::*;
    #[allow(unused_imports)]
    pub use crate::integration::*;
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
    pub use ici_files::prelude::*;
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
    #[error("Over 255 colours have been drawn")]
    TooManyColors,
    #[error("Size is greater than 255x255: {0}x{1}")]
    TooBig(usize, usize),
    #[error("Creating image")]
    ImageError(IndexedImageError),
}

pub struct Graphics<'buffer> {
    buffer: &'buffer mut [u8],
    width: usize,
    height: usize,
    ///Offsets all drawing commands
    translate: Coord,
    ///Used to restrict drawing to inside this region, initially includes the whole screen
    clip: Clip,
    /// Allows you to replace any supported ASCII with a custom glyph
    /// To replace 'a' with '█' for 4x5 fonts (such as `Standard4x5`) write
    ///
    /// `graphics.custom_font.insert(chr_to_code('a'), CustomLetter { _4x5: [true; 20], ..CustomLetter::default });`
    ///
    /// Characters are replaced on a size basis, so if `_4x4` is provided then all _4x4 fonts will draw this custom character
    ///
    /// Whitespace isn't supported and is skipped when drawing
    ///
    /// Note: `A-Za-z0-9!@$%^&*(),./;'\\[]<>?:\"{}_+~#…¤£¥¢✓|€` are valid for [text::chr_to_code]
    pub custom_font: FnvHashMap<u8, CustomLetter>,
}

impl Graphics<'_> {
    #[inline]
    pub fn create_buffer(width: usize, height: usize) -> Vec<u8> {
        vec![0; width * height * 4]
    }
}

/// Only the letter sizes you'll use need to be set
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CustomLetter {
    pub _4x4: [bool; text::font::standard_4x4::LETTER_PX_COUNT],
    pub _4x5: [bool; text::font::standard_4x5::LETTER_PX_COUNT],
    pub _6x7: [bool; text::font::standard_6x7::LETTER_PX_COUNT],
    pub _7x9: [bool; text::font::outline_7x9::LETTER_PX_COUNT],
    pub _8x8: [bool; text::font::script_8x8::LETTER_PX_COUNT],
    pub _8x10: [bool; text::font::standard_8x10::LETTER_PX_COUNT],
    pub _3x5: [bool; text::font::limited_3x5::LETTER_PX_COUNT],
}

impl Default for CustomLetter {
    fn default() -> Self {
        Self {
            _4x4: [false; text::font::standard_4x4::LETTER_PX_COUNT],
            _4x5: [false; text::font::standard_4x5::LETTER_PX_COUNT],
            _6x7: [false; text::font::standard_6x7::LETTER_PX_COUNT],
            _7x9: [false; text::font::outline_7x9::LETTER_PX_COUNT],
            _8x8: [false; text::font::script_8x8::LETTER_PX_COUNT],
            _8x10: [false; text::font::standard_8x10::LETTER_PX_COUNT],
            _3x5: [false; text::font::limited_3x5::LETTER_PX_COUNT],
        }
    }
}

pub fn make_image(
    width: usize,
    height: usize,
    method: fn(&mut Graphics),
) -> Result<Image, GraphicsError> {
    let mut buffer = Graphics::create_buffer(width, height);
    let mut graphics = Graphics::new(&mut buffer, width, height)?;
    method(&mut graphics);
    Ok(graphics.copy_to_image())
}

impl<'buffer> Graphics<'_> {
    /// `buffer` needs to be `width * height * 4` long
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
        if cfg!(debug_assertions) {
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
