use crate::color::Color;
use crate::image::Image;
use crate::image_loading::indexed::IndexedImageError::{
    GraphicsLibError, InvalidLineLengths, TooManyColors, UnknownChar,
};
use crate::renderable_image::{DrawOffset, RenderableImage};
use crate::{color, Graphics, GraphicsError};
use graphics_shapes::coord::Coord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem::swap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexedImageError {
    #[error("Creating graphics")]
    GraphicsLibError(#[from] GraphicsError),
    #[error("Char {0} is not in the palette")]
    UnknownChar(char),
    #[error("Image has different line lengths")]
    InvalidLineLengths(),
    #[error("Image has more than 255 colors")]
    TooManyColors,
}

pub const TRANSPARENT: u8 = 255;
pub const MAX_COLOR_COUNT: usize = 255;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexedImage {
    colors: Vec<Color>,
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl IndexedImage {
    pub fn new(colors: Vec<Color>, width: usize, height: usize, pixels: Vec<u8>) -> Self {
        Self {
            colors,
            width,
            height,
            pixels,
        }
    }

    pub fn new_empty(colors: Vec<Color>, width: usize, height: usize) -> Self {
        Self {
            colors,
            width,
            height,
            pixels: vec![TRANSPARENT; width * height],
        }
    }
}

impl IndexedImage {
    pub fn color(&self, idx: u8) -> Color {
        if idx != TRANSPARENT {
            self.colors[idx as usize]
        } else {
            color::TRANSPARENT
        }
    }

    pub fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    pub fn colors_mut(&mut self) -> &mut Vec<Color> {
        &mut self.colors
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut Vec<u8> {
        &mut self.pixels
    }

    pub fn apply_new_colors(&mut self, mut colors: Vec<Color>) {
        let new_max = colors.len() as u8;
        if new_max < self.colors.len() as u8 {
            for idx in &mut self.pixels {
                let r_idx = *idx;
                if r_idx >= new_max - 1 && r_idx != TRANSPARENT {
                    *idx = 0;
                }
            }
        }
        swap(&mut self.colors, &mut colors);
    }
}

impl IndexedImage {
    pub fn to_image(&self) -> Result<Image, IndexedImageError> {
        let mut buffer = vec![0_u8; self.pixels.len() * 4];
        let mut graphics =
            Graphics::new(&mut buffer, self.width, self.height).map_err(GraphicsLibError)?;
        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + y * self.width;
                let color = self.colors[self.pixels[i] as usize];
                graphics.set_pixel(x as isize, y as isize, color);
            }
        }
        Ok(graphics.copy_to_image())
    }

    pub fn to_renderable<P: Into<Coord>>(
        &self,
        xy: P,
        offset: DrawOffset,
    ) -> Result<RenderableImage, IndexedImageError> {
        Ok(RenderableImage::new(self.to_image()?, xy.into(), offset))
    }
}

/// Creates an image using an index and a grid of chars
pub fn create_image_from_string(
    palette: &HashMap<char, Color>,
    input: &str,
) -> Result<IndexedImage, IndexedImageError> {
    let mut pixels = vec![];
    let mut width = 0;
    let mut line = vec![];
    let mut colors = vec![];
    let chars = input.chars().enumerate();
    let count = input.chars().count() - 2;
    for (i, chr) in chars {
        if chr == '\n' || i == count {
            if pixels.is_empty() {
                width = line.len();
            } else if line.len() != width {
                return Err(InvalidLineLengths());
            } else {
                pixels.push(line.clone());
                line.clear();
            }
        } else {
            match palette.get(&chr) {
                None => return Err(UnknownChar(chr)),
                Some(color) => {
                    let i = if let Some(i) = colors.iter().position(|clr| clr == color) {
                        i
                    } else {
                        colors.push(*color);
                        if colors.len() > MAX_COLOR_COUNT {
                            return Err(TooManyColors);
                        }
                        colors.len() - 1
                    };
                    line.push(i as u8);
                }
            }
        }
    }
    let height = pixels.len();
    let pixels = pixels.iter().flatten().cloned().collect();
    Ok(IndexedImage::new(colors, width, height, pixels))
}
