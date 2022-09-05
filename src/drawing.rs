use crate::color::{Color, BLACK};
use crate::coord::Coord;
use crate::image::Image;
use crate::shapes::rect::Rect;
use crate::shapes::{DrawType, Shape};
use crate::text::{normal_letters, small_letters, TextPos, TextSize};
use crate::Graphics;

pub trait Renderable {
    fn render(&self, graphics: &mut Graphics);
}

impl Graphics<'_> {
    /// Convert an x,y coord to idx for use with `self.pixels`
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        (x + y * self.width) * 4
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_on_screen(&self, point: Coord) -> bool {
        let mut rect = Rect::new((0, 0), (self.width, self.height), DrawType::Stroke(BLACK));
        rect = rect.translate_by((self.translate.x, self.translate.y));
        rect.contains((point.x, point.y))
    }
}

impl Graphics<'_> {
    /// Get the canvas offset in pixels
    pub fn get_translate(&self) -> Coord {
        self.translate
    }

    /// Set the canvas offset in pixels
    ///
    /// All drawing commands will be offset by this value
    pub fn set_translate(&mut self, new_value: Coord) {
        self.translate = new_value;
    }

    /// Adds `delta` to the current canvas offset
    pub fn update_translate(&mut self, delta: Coord) {
        self.translate.x += delta.x;
        self.translate.y += delta.y;
    }

    /// Copy entire pixels array to an image
    ///
    /// Although the method takes `&mut self` it doesn't mutate anything
    pub fn copy_to_image(&mut self) -> Image {
        let pixels = self
            .buffer
            .chunks_exact(4)
            .map(|px| Color {
                r: px[0],
                g: px[1],
                b: px[2],
                a: px[3],
            })
            .collect::<Vec<Color>>();
        Image::new(pixels, self.width, self.height)
            .expect("Copy to image failed, please create github issue for buffer-graphics-lib")
    }

    /// Get top left pixel coord for letter px coord
    pub fn get_px_for_char(x: usize, y: usize, size: TextSize) -> (usize, usize) {
        let (width, height) = size.get_size();
        let margin = size.get_margin();
        (x * (width + margin), y * (height + margin))
    }

    /// Get width and height for string
    ///
    /// # Arguments
    /// * `text` - The string to be measured
    /// * `width` - The line width in characters
    /// * `size` - The text size to use when measuring
    ///
    /// # Returns
    ///
    /// The width and height of the string in pixels
    pub fn get_text_size(text: &str, width: usize, size: TextSize) -> (usize, usize) {
        let len = text.chars().count();
        let x = if len < width { len } else { width };
        let y = (len as f64 / width as f64).ceil() as usize;
        let (width, height) = size.get_size();
        let margin = size.get_margin();
        ((width + margin) * x, (height + margin) * y)
    }

    /// Draw an image at `x`, `y`
    pub fn draw_image<P: Into<Coord>>(&mut self, xy: P, image: &Image) {
        let xy = xy.into();
        let mut x = 0;
        for (y, row) in image.pixels.chunks_exact(image.width()).enumerate() {
            for color in row {
                if color.a > 0 {
                    self.update_pixel(xy.x + x, xy.y + y as isize, *color);
                }
                x += 1;
            }
            x = 0;
        }
    }

    pub fn draw_line<P1: Into<Coord>,P2: Into<Coord>>(&mut self, start: P1, end: P2, color: Color) {
        let start = start.into();
        let end = end.into();
        if start.x == end.x {
            for y in start.y..end.y {
                self.update_pixel(start.x, y, color);
            }
        } else if start.y == end.y {
            for x in start.x..end.x {
                self.update_pixel(x, start.y, color);
            }
        } else {
            let mut delta = 0;
            let x1 = start.x as isize;
            let y1 = start.y as isize;
            let x2 = end.x as isize;
            let y2 = end.y as isize;
            let dx = isize::abs(x2 - x1);
            let dy = isize::abs(y2 - y1);
            let dx2 = dx * 2;
            let dy2 = dy * 2;
            let ix: isize = if x1 < x2 { 1 } else { -1 };
            let iy: isize = if y1 < y2 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;
            if dx >= dy {
                loop {
                    self.update_pixel(x, y, color);
                    if x == x2 {
                        break;
                    }
                    x += ix;
                    delta += dy2;
                    if delta > dx {
                        y += iy;
                        delta -= dx2;
                    }
                }
            } else {
                loop {
                    self.update_pixel(x, y, color);
                    if y == y2 {
                        break;
                    }
                    y += iy;
                    delta += dx2;
                    if delta > dy {
                        x += ix;
                        delta -= dy2;
                    }
                }
            }
        }
    }

    /// Draw renderable
    pub fn draw_at<P: Into<Coord>>(&mut self, xy: P, shape: &dyn Renderable) {
        let xy = xy.into();
        self.update_translate(xy);
        shape.render(self);
        self.update_translate(-xy);
    }

    /// Draw renderable
    pub fn draw(&mut self, shape: &dyn Renderable) {
        shape.render(self);
    }

    /// Get the RGB values for a pixel
    /// Alpha will always be 255
    ///
    /// If `use_translate` is true than the x,y will be updated with `self.translate`
    ///
    /// Although the method takes `&mut self` it doesn't mutate anything
    #[inline]
    fn get_pixel(&mut self, x: isize, y: isize, use_translate: bool) -> Option<Color> {
        let (x, y) = if use_translate {
            (x as isize + self.translate.x, y as isize + self.translate.y)
        } else {
            (x as isize, y as isize)
        };

        if x >= 0 && y >= 0 && x < self.width as isize {
            let idx = self.index(x as usize, y as usize);
            if idx < self.buffer.len() {
                return Some(Color::rgb(
                    self.buffer[idx],
                    self.buffer[idx + 1],
                    self.buffer[idx + 2],
                ));
            }
        }

        None
    }

    /// Update a pixel color, using [PixelWrapper::set_pixel] or [PixelWrapper::blend_pixel] depending
    /// on whether `color`s alpha is 255 or not
    #[inline]
    pub fn update_pixel(&mut self, x: isize, y: isize, color: Color) {
        if color.a == 255 {
            self.set_pixel(x, y, color);
        } else {
            self.blend_pixel(x, y, color);
        }
    }

    /// Sets every pixel to the same color, this ignores translate
    pub fn clear(&mut self, color: Color) {
        self.buffer.chunks_exact_mut(4).for_each(|px| {
            px[0] = color.r;
            px[1] = color.g;
            px[2] = color.b;
            px[3] = color.a;
        });
    }

    /// Draw a letter at pos
    pub fn draw_letter(&mut self, pos: TextPos, chr: char, size: TextSize, color: Color) {
        if chr == ' ' {
            return;
        }
        let (width, height) = size.get_size();
        let px: Vec<bool> = match size {
            TextSize::Small => small_letters::get_px(chr).to_vec(),
            TextSize::Normal => normal_letters::get_px(chr).to_vec(),
        };
        let start_x;
        let start_y;
        match pos {
            TextPos::Px(x, y) => {
                start_x = x;
                start_y = y;
            }
            TextPos::ColRow(x, y) => {
                start_x = (x * size.get_size().0) as isize;
                start_y = (y * size.get_size().1) as isize;
            }
        }

        for x in 0..width {
            for y in 0..height {
                let i = x + y * width;
                if px[i] {
                    self.update_pixel(x as isize + start_x, y as isize + start_y, color);
                }
            }
        }
    }

    /// Draws text in lines at most `width` chars long at pixel coord
    ///
    /// Width should be max chars - x
    /// See [TextSize::get_max_characters] for maximum chars
    pub fn draw_text(
        &mut self,
        text: &str,
        line_width: Option<usize>,
        start_pos: TextPos,
        size: TextSize,
        color: Color,
    ) {
        let start_x;
        let mut y;
        match start_pos {
            TextPos::Px(xpx, ypx) => {
                start_x = xpx;
                y = ypx;
            }
            TextPos::ColRow(c, r) => {
                start_x = (c * size.get_size().0) as isize;
                y = (r * size.get_size().1) as isize;
            }
        }
        let mut x = start_x;
        for char in text.chars() {
            self.draw_letter(TextPos::Px(x, y), char, size, color);
            x += (size.get_size().0 + size.get_margin()) as isize;
            if let Some(width) = line_width {
                if x >= (width * (size.get_size().0 + size.get_margin())) as isize + start_x {
                    y += (size.get_size().1 + size.get_margin()) as isize;
                    x = start_x;
                }
            }
        }
    }

    /// Set the RGB values for a pixel by blending it with the provided color
    /// This method uses alpha blending, note that the canvas pixels always have 255 alpha
    #[inline]
    pub fn blend_pixel(&mut self, x: isize, y: isize, color: Color) {
        let x = x + self.translate.x;
        let y = y + self.translate.y;
        if x >= 0 && y >= 0 && x < self.width as isize {
            if let Some(base) = self.get_pixel(x, y, false) {
                let new_color = base.blend(color);
                let idx = self.index(x as usize, y as usize);
                self.buffer[idx] = new_color.r;
                self.buffer[idx + 1] = new_color.g;
                self.buffer[idx + 2] = new_color.b;
            }
        }
    }

    /// Set the RGB values for a pixel
    /// This ignores alpha, so 255,0,0,0 will draw a red pixel
    #[inline]
    pub fn set_pixel(&mut self, x: isize, y: isize, color: Color) {
        let x = x + self.translate.x;
        let y = y + self.translate.y;
        if x >= 0 && y >= 0 && x < self.width as isize {
            let idx = self.index(x as usize, y as usize);

            if idx < self.buffer.len() {
                self.buffer[idx] = color.r;
                self.buffer[idx + 1] = color.g;
                self.buffer[idx + 2] = color.b;
                self.buffer[idx + 3] = color.a;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Coord, Graphics};

    #[test]
    fn is_inside() {
        let mut buf = [0; 400];
        let mut graphics = Graphics::new(&mut buf, 10, 10).unwrap();
        assert!(graphics.is_on_screen(Coord { x: 1, y: 1 }));
        assert!(graphics.is_on_screen(Coord { x: 9, y: 9 }));
        assert!(graphics.is_on_screen(Coord { x: 0, y: 0 }));
        assert!(!graphics.is_on_screen(Coord { x: 10, y: 10 }));
        assert!(!graphics.is_on_screen(Coord { x: 4, y: -1 }));
        assert!(!graphics.is_on_screen(Coord { x: -1, y: 4 }));

        graphics.set_translate(Coord { x: 2, y: -1 });
        assert!(graphics.is_on_screen(Coord { x: 4, y: 4 }));
        assert!(graphics.is_on_screen(Coord { x: 4, y: 0 }));
        assert!(!graphics.is_on_screen(Coord { x: 0, y: 0 }));
        assert!(!graphics.is_on_screen(Coord { x: 4, y: 9 }));
    }
}
