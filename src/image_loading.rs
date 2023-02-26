use crate::drawing;
use crate::image_loading::ImageWrapperError::GraphicsLibError;
use crate::prelude::*;
use image::{DynamicImage, ImageFormat};
use std::io::{BufRead, Seek};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageWrapperError {
    #[error("Loading image")]
    GraphicsLibError(#[from] GraphicsError),
}

/// Load image bytes (from png, bmp, etc) as [Image]
/// Useful with include_bytes!()
pub fn load_image<R: BufRead + Seek>(
    r: R,
    format: ImageFormat,
) -> Result<Image, ImageWrapperError> {
    convert_image(drawing::load(r, format)?)
}

/// Load an image (png, bmp, etc) as [Image]
pub fn open_image<P: AsRef<Path>>(path: P) -> Result<Image, ImageWrapperError> {
    convert_image(drawing::open(path)?)
}

///Convert a [DynamicImage] from the `image` crate into an [Image]
pub fn convert_image(image: DynamicImage) -> Result<Image, ImageWrapperError> {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let pixels = image
        .into_rgba8()
        .chunks_exact(4)
        .map(|px| Color::rgba(px[0], px[1], px[2], px[3]))
        .collect();

    Image::new(pixels, width, height).map_err(GraphicsLibError)
}
