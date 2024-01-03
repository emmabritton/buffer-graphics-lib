use crate::color::{Color, WHITE};
use crate::renderable_image::{DrawOffset, RenderableImage};
use crate::scaling::*;
use crate::{Graphics, GraphicsError, Tint};
use graphics_shapes::coord::Coord;
use ici_files::image::IndexedImage;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// Images are rectangles of pixels that can be manipulated and drawn on screen
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq)]
pub struct Image {
    pub(crate) bytes: Vec<u8>,
    width: usize,
    height: usize,
    is_transparent: bool,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image: {}x{}", self.width, self.height)
    }
}

impl Image {
    /// Create a image of width x height size using provided pixels
    pub fn new(pixels: Vec<Color>, width: usize, height: usize) -> Result<Self, GraphicsError> {
        let is_transparent = pixels.iter().any(|c| c.is_transparent());
        if width * height != pixels.len() {
            Err(GraphicsError::ImageInitSize(width * height, pixels.len()))
        } else {
            let bytes = pixels.into_iter().flat_map(|c| c.as_array()).collect();
            Ok(Image {
                bytes,
                width,
                height,
                is_transparent,
            })
        }
    }

    /// Create a white image of width x height size
    pub fn new_blank(width: usize, height: usize) -> Self {
        let pixels = vec![WHITE; width * height];
        Image::new(pixels, width, height).expect(
            "Failed to create blank image, please create GitHub issue for buffer-graphics-lib",
        )
    }

    pub fn from_indexed(indexed_image: IndexedImage) -> Image {
        let mut pixels = vec![0; indexed_image.width() as usize * indexed_image.height() as usize];
        let mut graphics = Graphics::new(&mut pixels, indexed_image.width() as usize, indexed_image.height() as usize)
            .expect("Creating buffer to make image from indexed image, please raise an issue on GitHub buffer-graphics-lib");
        graphics.draw_indexed_image((0, 0), &indexed_image);
        graphics.copy_to_image()
    }
}

impl Image {
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns true if any pixels are transparent
    #[inline]
    pub fn is_transparent(&self) -> bool {
        self.is_transparent
    }

    /// This is calculated every time
    pub fn pixels(&self) -> Vec<Color> {
        self.bytes
            .chunks_exact(4)
            .map(|chunk| Color {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            })
            .collect()
    }

    fn recalc_transparency(&mut self) {
        self.is_transparent = self.pixels().iter().any(|c| c.is_transparent());
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let addr = (y * self.width + x) * 4;
        Color {
            r: self.bytes[addr],
            g: self.bytes[addr + 1],
            b: self.bytes[addr + 2],
            a: self.bytes[addr + 3],
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, value: Color) {
        let addr = (y * self.width + x) * 4;
        self.bytes[addr] = value.r;
        self.bytes[addr + 1] = value.g;
        self.bytes[addr + 2] = value.b;
        self.bytes[addr + 3] = value.a;
        self.recalc_transparency();
    }

    #[inline]
    pub fn blend_pixel(&mut self, x: usize, y: usize, value: Color) {
        let new_color = self.get_pixel(x, y).blend(value);
        let addr = (y * self.width + x) * 4;
        self.bytes[addr] = new_color.r;
        self.bytes[addr + 1] = new_color.g;
        self.bytes[addr + 2] = new_color.b;
        self.bytes[addr + 3] = new_color.a;
        self.recalc_transparency();
    }

    /// Flip image horizontally
    pub fn flip_horizontal(&mut self) {
        let half_width = (self.width as f32 / 2.).floor() as usize;
        for y in 0..self.height {
            for x in 0..half_width {
                let y = y * self.width;
                unsafe {
                    std::ptr::swap_nonoverlapping(
                        &mut self.bytes[(y + x) * 4],
                        &mut self.bytes[(y + self.width - 1 - x) * 4],
                        4,
                    );
                }
            }
        }
    }

    /// Flip image vertically
    pub fn flip_vertical(&mut self) {
        let half_height = (self.height as f32 / 2.).floor() as usize;
        for y in 0..half_height {
            unsafe {
                std::ptr::swap_nonoverlapping(
                    &mut self.bytes[(y * self.width) * 4],
                    &mut self.bytes[((self.height - 1 - y) * self.width) * 4],
                    self.width * 4,
                );
            }
        }
    }

    /// Rotate 90° clockwise
    pub fn rotate_cw(&mut self) -> Image {
        let mut output = Image::new_blank(self.height, self.width);
        for y in 0..self.height {
            for x in 0..self.width {
                let new_y = x;
                let new_x = output.width - y - 1;
                output.set_pixel(new_x, new_y, self.get_pixel(x, y));
            }
        }
        output
    }

    /// Rotate 90° counterclockwise
    pub fn rotate_ccw(&mut self) -> Image {
        let mut output = Image::new_blank(self.height, self.width);
        for y in 0..self.height {
            for x in 0..self.width {
                let new_y = output.height - x - 1;
                let new_x = y;
                output.set_pixel(new_x, new_y, self.get_pixel(x, y));
            }
        }
        output
    }

    /// Blend with another image
    pub fn blend(&mut self, other: &Image) -> Result<(), GraphicsError> {
        if self.width != other.width || self.height != other.height {
            return Err(GraphicsError::ImageBlendSize(
                self.width,
                self.height,
                other.width,
                other.height,
            ));
        }

        for y in 0..self.height {
            for x in 0..self.width {
                self.blend_pixel(x, y, other.get_pixel(x, y));
            }
        }

        self.recalc_transparency();
        Ok(())
    }

    /// Return a new image after scaling
    pub fn scale(&self, algo: Scaling) -> Image {
        match algo {
            Scaling::NearestNeighbour { x_scale, y_scale } => {
                scale_nearest_neighbor(self, usize::from(x_scale), usize::from(y_scale))
            }
            Scaling::Epx2x => scale_epx(self),
            Scaling::Epx4x => scale_epx(&scale_epx(self)),
        }
    }

    pub fn to_renderable<P: Into<Coord>>(self, xy: P, draw_offset: DrawOffset) -> RenderableImage {
        RenderableImage::new(self, xy.into(), draw_offset)
    }
}

impl Tint for Image {
    fn tint_add(&mut self, r_diff: isize, g_diff: isize, b_diff: isize, a_diff: isize) {
        for pixel in self.bytes.chunks_exact_mut(4) {
            let mut color = Color {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            };
            color.tint_add(r_diff, g_diff, b_diff, a_diff);
            pixel[0] = color.r;
            pixel[1] = color.g;
            pixel[2] = color.b;
            pixel[3] = color.a;
        }
        self.recalc_transparency();
    }

    fn tint_mul(&mut self, r_diff: f32, g_diff: f32, b_diff: f32, a_diff: f32) {
        for pixel in self.bytes.chunks_exact_mut(4) {
            let mut color = Color {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            };
            color.tint_mul(r_diff, g_diff, b_diff, a_diff);
            pixel[0] = color.r;
            pixel[1] = color.g;
            pixel[2] = color.b;
            pixel[3] = color.a;
        }
        self.recalc_transparency();
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;
    use crate::image::Image;
    use crate::scaling::Scaling;
    use crate::Tint;

    fn make_image() -> Image {
        Image::new(
            vec![
                Color::gray(1),
                Color::gray(2),
                Color::gray(3),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
                Color::gray(7),
                Color::gray(8),
                Color::gray(9),
            ],
            3,
            3,
        )
            .unwrap()
    }

    #[test]
    fn constructor() {
        let image = make_image();

        assert_eq!(image.width, 3);
        assert_eq!(image.height, 3);
        assert_eq!(image.bytes.len(), 9 * 4);
        assert!(!image.is_transparent);
        assert_eq!(
            image.pixels(),
            vec![
                Color::gray(1),
                Color::gray(2),
                Color::gray(3),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
                Color::gray(7),
                Color::gray(8),
                Color::gray(9),
            ]
        );
    }

    #[test]
    fn test_flip() {
        let image = make_image();
        assert_eq!(
            image.pixels(),
            vec![
                Color::gray(1),
                Color::gray(2),
                Color::gray(3),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
                Color::gray(7),
                Color::gray(8),
                Color::gray(9),
            ]
        );
        let mut horz = make_image();
        horz.flip_horizontal();
        assert_eq!(
            horz.pixels(),
            vec![
                Color::gray(3),
                Color::gray(2),
                Color::gray(1),
                Color::gray(6),
                Color::gray(5),
                Color::gray(4),
                Color::gray(9),
                Color::gray(8),
                Color::gray(7),
            ]
        );
        let mut vert = make_image();
        vert.flip_vertical();
        assert_eq!(
            vert.pixels(),
            vec![
                Color::gray(7),
                Color::gray(8),
                Color::gray(9),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
                Color::gray(1),
                Color::gray(2),
                Color::gray(3),
            ]
        );
        let mut horz_vert = make_image();
        horz_vert.flip_horizontal();
        horz_vert.flip_vertical();
        assert_eq!(
            horz_vert.pixels(),
            vec![
                Color::gray(9),
                Color::gray(8),
                Color::gray(7),
                Color::gray(6),
                Color::gray(5),
                Color::gray(4),
                Color::gray(3),
                Color::gray(2),
                Color::gray(1),
            ]
        );
        let mut vert_horz = make_image();
        vert_horz.flip_horizontal();
        vert_horz.flip_vertical();
        assert_eq!(
            vert_horz.pixels(),
            vec![
                Color::gray(9),
                Color::gray(8),
                Color::gray(7),
                Color::gray(6),
                Color::gray(5),
                Color::gray(4),
                Color::gray(3),
                Color::gray(2),
                Color::gray(1),
            ]
        );
    }

    #[test]
    fn tint_add() {
        let mut image = make_image();
        image.tint_add(10, 20, 30, -50);
        assert_eq!(
            image.pixels(),
            vec![
                Color::rgba(11, 21, 31, 205),
                Color::rgba(12, 22, 32, 205),
                Color::rgba(13, 23, 33, 205),
                Color::rgba(14, 24, 34, 205),
                Color::rgba(15, 25, 35, 205),
                Color::rgba(16, 26, 36, 205),
                Color::rgba(17, 27, 37, 205),
                Color::rgba(18, 28, 38, 205),
                Color::rgba(19, 29, 39, 205),
            ]
        );
    }

    #[test]
    fn tint_mul() {
        let mut image = make_image();
        image.tint_mul(0.5, 1.0, 2.0, 1.0);
        assert_eq!(
            image.pixels(),
            vec![
                Color::rgba(1, 1, 2, 255),
                Color::rgba(1, 2, 4, 255),
                Color::rgba(2, 3, 6, 255),
                Color::rgba(2, 4, 8, 255),
                Color::rgba(3, 5, 10, 255),
                Color::rgba(3, 6, 12, 255),
                Color::rgba(4, 7, 14, 255),
                Color::rgba(4, 8, 16, 255),
                Color::rgba(5, 9, 18, 255),
            ]
        );
    }

    #[test]
    fn rect_scaling() {
        let image = Image::new(
            vec![
                Color::gray(1),
                Color::gray(2),
                Color::gray(3),
                Color::gray(4),
                Color::gray(5),
                Color::gray(6),
            ],
            3,
            2,
        )
            .unwrap();

        let epx2 = image.scale(Scaling::Epx2x);
        let epx4 = image.scale(Scaling::Epx4x);
        let nn2 = image.scale(Scaling::nearest_neighbour(2, 2));
        let nn3 = image.scale(Scaling::nearest_neighbour(3, 3));

        assert_eq!(image.width, 3);
        assert_eq!(image.height, 2);
        assert_eq!(epx2.width, 6);
        assert_eq!(epx2.height, 4);
        assert_eq!(epx4.width, 12);
        assert_eq!(epx4.height, 8);
        assert_eq!(nn2.width, 6);
        assert_eq!(nn2.height, 4);
        assert_eq!(nn3.width, 9);
        assert_eq!(nn3.height, 6);
    }

    #[test]
    fn square_scaling() {
        let image = make_image();
        let epx2 = image.scale(Scaling::Epx2x);
        let epx4 = image.scale(Scaling::Epx4x);
        let nn_double = image.scale(Scaling::nn_double());
        let nn2 = image.scale(Scaling::nearest_neighbour(2, 2));
        let nn3 = image.scale(Scaling::nearest_neighbour(3, 3));
        let nn1_1 = image.scale(Scaling::nearest_neighbour(1, 1));
        let nn1_2 = image.scale(Scaling::nearest_neighbour(1, 2));
        let nn2_1 = image.scale(Scaling::nearest_neighbour(2, 1));

        assert_eq!(image.width, 3);
        assert_eq!(image.height, 3);
        assert_eq!(epx2.width, 6);
        assert_eq!(epx2.height, 6);
        assert_eq!(nn_double.width, 6);
        assert_eq!(nn_double.height, 6);
        assert_eq!(nn2.width, 6);
        assert_eq!(nn2.height, 6);
        assert_eq!(nn3.width, 9);
        assert_eq!(nn3.height, 9);
        assert_eq!(epx4.width, 12);
        assert_eq!(epx4.height, 12);
        assert_eq!(nn1_1.width, 3);
        assert_eq!(nn1_1.height, 3);
        assert_eq!(nn1_2.width, 3);
        assert_eq!(nn1_2.height, 6);
        assert_eq!(nn2_1.width, 6);
        assert_eq!(nn2_1.height, 3);
    }
}
