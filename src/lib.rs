//! Buffer Graphics Lib
//!
//! This is a simple graphics library for drawing to a buffer, mainly designed to be used with [Rust Graphics Library](https://github.com/raybritton/rust-graphics-lib) or [Pixels](https://github.com/parasyte/pixels)
//!
//! It has basic shape drawing, bitmap text and image rendering.
//!
//! Using the library is as simple as:
//!```
//! # use buffer_graphics_lib::color::{BLACK, LIGHT_GRAY};
//! # use buffer_graphics_lib::drawing::DrawingMethods;
//! # use buffer_graphics_lib::Graphics;
//! # use buffer_graphics_lib::image_loading::load_image;
//! # use buffer_graphics_lib::text::{TextPos, TextSize};
//!let mut buffer = [0_u8; 800 * 600 * 4]; //800 x 600 RGBA
//!let mut graphics = Graphics::new(&mut buffer, 800, 600).unwrap();
//!graphics.draw_text("Some text", None, TextPos::Px(1, 1), TextSize::Normal, BLACK);
//!graphics.draw_rect(1, 1, 100, 100, LIGHT_GRAY);
//! ```

#![deny(clippy::all)]

pub mod color;
pub mod drawing;
pub mod image;
#[cfg(feature = "image_loading")]
pub mod image_loading;
pub mod math;
pub mod scaling;
pub mod text;

use crate::GraphicsError::InvalidBufferLength;
use mint::Point2;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("Invalid buffer length, expected: {0}, found: {1}")]
    InvalidBufferLength(usize, usize),
    #[error("Invalid pixel array length, expected: {0}, found: {1}")]
    ImageInitSize(usize, usize),
    #[error("Both images must be the same size, expected: {0}x{1}, found: {2}x{3}")]
    ImageBlendSize(usize, usize, usize, usize),
}

pub struct Graphics<'buffer, const N: usize> {
    buffer: &'buffer mut [u8; N],
    width: usize,
    height: usize,
    translate: Point2<isize>,
}

impl<'buffer, const N: usize> Graphics<'_, N> {
    pub fn new(
        buffer: &'buffer mut [u8; N],
        width: usize,
        height: usize,
    ) -> Result<Graphics<'buffer, N>, GraphicsError> {
        let count = width * height * 4;
        if count != buffer.len() {
            return Err(InvalidBufferLength(count, buffer.len()));
        }
        Ok(Graphics {
            buffer,
            width,
            height,
            translate: Point2 { x: 0, y: 0 },
        })
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
