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

#![deny(clippy::all)]

pub mod color;
pub mod drawable;
pub mod drawing;
pub mod image;
#[cfg(feature = "image_loading")]
pub mod image_loading;
pub mod renderable_image;
pub mod scaling;
pub mod shapes;
pub mod text;

use crate::GraphicsError::InvalidBufferLength;
use graphics_shapes::coord::Coord;
use thiserror::Error;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::drawable::*;
    pub use crate::drawing::Renderable;
    pub use crate::Graphics;
    pub use crate::GraphicsError;
    pub use graphics_shapes::prelude::*;
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
        }
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
