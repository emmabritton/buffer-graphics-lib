use crate::clipping::Clip;
use crate::drawable::{DrawType, Drawable};
use crate::image::Image;
use crate::prelude::PixelFont;
use crate::shapes::CreateDrawable;
use crate::text::format::TextFormat;
use crate::text::pos::TextPos;
use crate::text::{chr_to_code, Text};
use crate::Graphics;
use graphics_shapes::circle::Circle;
use graphics_shapes::coord::Coord;
use graphics_shapes::polygon::Polygon;
use graphics_shapes::prelude::Ellipse;
use graphics_shapes::rect::Rect;
use graphics_shapes::triangle::Triangle;
use ici_files::prelude::*;
use std::mem::swap;

/// Represents anything that [Graphics] can render
pub trait Renderable<T> {
    fn render(&self, graphics: &mut Graphics);
}

impl Graphics<'_> {
    /// Convert an x,y coord to idx for use with `self.pixels`
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        (x + y * self.width) * 4
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_on_screen(&self, point: Coord) -> bool {
        let x = point.x - self.translate.x;
        let y = point.y - self.translate.y;
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }
}

impl Graphics<'_> {
    /// Get the canvas offset in pixels
    #[inline]
    pub fn get_translate(&self) -> Coord {
        self.translate
    }

    /// Set the canvas offset in pixels
    ///
    /// All drawing commands will be offset by this value
    ///
    /// # Returns
    /// The previous translate value
    #[inline]
    pub fn set_translate(&mut self, new_value: Coord) -> Coord {
        let old = self.translate;
        self.translate = new_value;
        old
    }

    #[inline]
    pub fn with_translate<F: Fn(&mut Graphics)>(&mut self, set: Coord, method: F) {
        let old_trans = self.set_translate(set);
        method(self);
        self.set_translate(old_trans);
    }

    /// Adds `delta` to the current canvas offset
    #[inline]
    pub fn update_translate(&mut self, delta: Coord) {
        self.translate.x += delta.x;
        self.translate.y += delta.y;
    }

    /// Copy entire pixels array to an image
    pub fn copy_to_image(&self) -> Image {
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
            .expect("Copy to image failed, please create GitHub issue for buffer-graphics-lib")
    }

    /// Get top left pixel coord for letter col row
    pub fn get_px_for_char(col: usize, row: usize, font: &PixelFont) -> (usize, usize) {
        (col * font.char_width(), row * font.line_height())
    }

    /// Draw an image at `x`, `y` as fast as possible
    /// If the image may draw outside the window you must use [draw_image] instead
    /// Ignores clipping and translation for opaque images
    pub fn draw_image_unchecked<P: Into<Coord>>(&mut self, xy: P, image: &Image) {
        let xy = xy.into();
        if image.is_transparent() {
            self.draw_image(xy, image);
        } else {
            let byte_count = image.width() * 4;
            for (y, row) in image.bytes.chunks_exact(byte_count).enumerate() {
                let addr = (((xy.y + y as isize) * self.width() as isize + xy.x) * 4) as usize;
                unsafe {
                    let dst = self.buffer.as_mut_ptr().add(addr);
                    std::ptr::copy_nonoverlapping(row.as_ptr(), dst, byte_count);
                }
            }
        }
    }

    /// Draw an image at `x`, `y`
    /// If the image definitely will draw inside the window you can use [draw_image_unchecked] instead
    pub fn draw_image<P: Into<Coord>>(&mut self, xy: P, image: &Image) {
        let xy = xy.into();
        let mut x = 0;
        let mut y = 0;
        for pixel in image.bytes.chunks_exact(4) {
            update_pixel(
                self.buffer,
                &self.translate,
                &self.clip,
                (self.width, self.height),
                xy.x + x as isize,
                xy.y + y,
                Color {
                    r: pixel[0],
                    g: pixel[1],
                    b: pixel[2],
                    a: pixel[3],
                },
            );
            x += 1;
            if x >= image.width() {
                x = 0;
                y += 1;
            }
        }
    }

    /// Draw an indexed image at `x`, `y`

    pub fn draw_indexed_image<P: Into<Coord>>(&mut self, xy: P, image: &IndexedImage) {
        let xy = xy.into();
        let palette = image.get_palette();
        let (width, height) = image.size();
        for x in 0..width {
            for y in 0..height {
                let i = image.get_pixel_index(x, y).unwrap();
                let color_idx = image.get_pixel(i).unwrap() as usize;
                let color = palette[color_idx];
                update_pixel(
                    self.buffer,
                    &self.translate,
                    &self.clip,
                    (self.width, self.height),
                    x as isize + xy.x,
                    y as isize + xy.y,
                    color,
                );
            }
        }
    }

    pub fn draw_wrapped_image<P: Into<Coord>>(&mut self, xy: P, image: &IndexedWrapper) {
        match image {
            IndexedWrapper::Static(img) => self.draw_indexed_image(xy, img),
            IndexedWrapper::Animated(img) => self.draw_animated_image(xy, img),
        }
    }

    /// Draw an animated image at `x`, `y`

    pub fn draw_animated_image<P: Into<Coord>>(&mut self, xy: P, image: &AnimatedIndexedImage) {
        let xy = xy.into();
        let palette = image.get_palette();
        let (width, height) = image.size();
        let current_frame = image.get_current_frame_pixels();
        for x in 0..width {
            for y in 0..height {
                let i = image.get_pixel_index(x, y).unwrap();
                let color_idx = current_frame[i] as usize;
                let color = palette[color_idx];
                update_pixel(
                    self.buffer,
                    &self.translate,
                    &self.clip,
                    (self.width, self.height),
                    x as isize + xy.x,
                    y as isize + xy.y,
                    color,
                );
            }
        }
    }

    pub fn draw_arc(
        &mut self,
        center: Coord,
        angle_start: isize,
        angle_end: isize,
        radius: usize,
        close: bool,
        color: Color,
    ) {
        for r in angle_start..=angle_end {
            let px = Coord::from_angle(center, radius, r);
            update_pixel(
                self.buffer,
                &self.translate,
                &self.clip,
                (self.width, self.height),
                px.x,
                px.y,
                color,
            );
        }
        if close {
            self.draw_line(
                center,
                Coord::from_angle(center, radius, angle_start),
                color,
            );
            self.draw_line(center, Coord::from_angle(center, radius, angle_end), color);
        }
    }

    pub fn draw_line<P1: Into<Coord>, P2: Into<Coord>>(
        &mut self,
        start: P1,
        end: P2,
        color: Color,
    ) {
        let mut start = start.into();
        let mut end = end.into();
        if start.x > end.x || start.y > end.y {
            swap(&mut start, &mut end);
        }
        if start.x == end.x {
            for y in start.y..=end.y {
                update_pixel(
                    self.buffer,
                    &self.translate,
                    &self.clip,
                    (self.width, self.height),
                    start.x,
                    y,
                    color,
                );
            }
        } else if start.y == end.y {
            for x in start.x..=end.x {
                update_pixel(
                    self.buffer,
                    &self.translate,
                    &self.clip,
                    (self.width, self.height),
                    x,
                    start.y,
                    color,
                );
            }
        } else {
            let mut delta = 0;
            let x1 = start.x;
            let y1 = start.y;
            let x2 = end.x;
            let y2 = end.y;
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
                    update_pixel(
                        self.buffer,
                        &self.translate,
                        &self.clip,
                        (self.width, self.height),
                        x,
                        y,
                        color,
                    );
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
                    update_pixel(
                        self.buffer,
                        &self.translate,
                        &self.clip,
                        (self.width, self.height),
                        x,
                        y,
                        color,
                    );
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

    /// Draw renderable offset by [xy]
    #[inline]
    pub fn draw_offset<T, P: Into<Coord>>(&mut self, xy: P, renderable: &dyn Renderable<T>) {
        self.with_translate(xy.into(), |g| renderable.render(g));
    }

    /// Draw renderable
    #[inline]
    pub fn draw<T>(&mut self, renderable: &dyn Renderable<T>) {
        renderable.render(self);
    }

    /// Get the RGB values for a pixel
    /// Alpha will always be 255
    ///
    /// If `use_translate` is true than the x,y will be updated with `self.translate`
    ///
    /// Although the method takes `&mut self` it doesn't mutate anything
    #[inline]
    pub fn get_pixel(&mut self, x: isize, y: isize, use_translate: bool) -> Option<Color> {
        let (x, y) = if use_translate {
            (x + self.translate.x, y + self.translate.y)
        } else {
            (x, y)
        };

        if x >= 0 && y >= 0 && x < self.width as isize {
            let idx = self.index(x as usize, y as usize);
            if idx < self.buffer.len() {
                return Some(Color::new(
                    self.buffer[idx],
                    self.buffer[idx + 1],
                    self.buffer[idx + 2],
                    255,
                ));
            }
        }

        None
    }

    /// Sets every pixel to the same color, this ignores translate and clip
    pub fn clear(&mut self, color: Color) {
        self.buffer.chunks_exact_mut(4).for_each(|px| {
            px[0] = color.r;
            px[1] = color.g;
            px[2] = color.b;
            px[3] = color.a;
        });
    }

    /// Sets every pixel to the same color, same as [clear] but this follows translate and clip
    pub fn clear_aware(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x as isize, y as isize, color);
            }
        }
    }

    /// Draw a letter at pos
    #[inline]
    pub fn draw_letter(&mut self, pos: (isize, isize), chr: char, font: PixelFont, color: Color) {
        self.draw_ascii_letter(pos, chr_to_code(chr), font, color);
    }

    pub fn draw_ascii_letter(
        &mut self,
        pos: (isize, isize),
        code: u8,
        font: PixelFont,
        color: Color,
    ) {
        if code == 32 || code == 9 {
            return;
        }
        let (width, height) = font.size();

        let px: &[bool] = if let Some(custom) = self.custom_font.get(&code) {
            match font {
                PixelFont::Standard4x4 => &custom._4x4,
                PixelFont::Script8x8 => &custom._8x8,
                PixelFont::Outline7x9 => &custom._7x9,
                PixelFont::Standard4x5 => &custom._4x5,
                PixelFont::Standard6x7 => &custom._6x7,
                PixelFont::Standard8x10 => &custom._8x10,
            }
        } else {
            font.pixels(code)
        };

        for x in 0..width {
            for y in 0..height {
                let i = x + y * width;
                if px[i] {
                    update_pixel(
                        self.buffer,
                        &self.translate,
                        &self.clip,
                        (self.width, self.height),
                        x as isize + pos.0,
                        y as isize + pos.1,
                        color,
                    );
                }
            }
        }
    }

    /// Should only be used by Text::render
    /// `text` param must already be corrected wrapped
    pub fn draw_ascii<P: Into<TextPos>, F: Into<TextFormat>>(
        &mut self,
        text: &[Vec<u8>],
        pos: P,
        format: F,
    ) {
        let format = format.into();
        let font = format.font();
        let color = format.color();
        let per_x = format.char_width();
        let per_y = format.line_height();

        if per_y == 0 || per_x == 0 {
            return;
        }

        let (start_x, start_y) = format.positioning().calc(
            pos.into().to_coord(font),
            text.iter().map(|list| list.len()).max().unwrap() * per_x.unsigned_abs(),
            text.len() * per_y.unsigned_abs(),
        );

        for (y, line) in text.iter().enumerate() {
            let y = y as isize * per_y;
            for (x, char) in line.iter().enumerate() {
                let x = x as isize * per_x;
                self.draw_ascii_letter((start_x + x, start_y + y), *char, font, color);
            }
        }
    }

    #[inline]
    pub fn draw_text<P: Into<TextPos>, F: Into<TextFormat>>(
        &mut self,
        text: &str,
        pos: P,
        format: F,
    ) {
        let text = Text::new(text, pos.into(), format.into());
        text.render(self);
    }

    #[inline]
    pub fn draw_rect<R: Into<Rect>>(&mut self, rect: R, draw_type: DrawType) {
        Drawable::from_obj(rect.into(), draw_type).render(self)
    }

    #[inline]
    pub fn draw_circle<C: Into<Circle>>(&mut self, circle: C, draw_type: DrawType) {
        Drawable::from_obj(circle.into(), draw_type).render(self)
    }

    #[inline]
    pub fn draw_polygon<P: Into<Polygon>>(&mut self, polygon: P, draw_type: DrawType) {
        Drawable::from_obj(polygon.into(), draw_type).render(self)
    }

    #[inline]
    pub fn draw_triangle<T: Into<Triangle>>(&mut self, triangle: T, draw_type: DrawType) {
        Drawable::from_obj(triangle.into(), draw_type).render(self)
    }

    #[inline]
    pub fn draw_ellipse<E: Into<Ellipse>>(&mut self, ellipse: E, draw_type: DrawType) {
        Drawable::from_obj(ellipse.into(), draw_type).render(self)
    }

    /// Update a pixel color, using [set_pixel] or [blend_pixel] depending on whether `color`s alpha is 255 or not
    ///
    /// If the alpha is 0 the call is does nothing
    #[inline]
    pub fn set_pixel(&mut self, x: isize, y: isize, color: Color) {
        update_pixel(
            self.buffer,
            &self.translate,
            &self.clip,
            (self.width, self.height),
            x,
            y,
            color,
        );
    }
}

/// Update a pixel color, using [set_pixel] or [blend_pixel] depending on whether `color`s alpha is 255 or not
///
/// If the alpha is 0 the call is does nothing
fn update_pixel(
    buffer: &mut [u8],
    translate: &Coord,
    clip: &Clip,
    (width, height): (usize, usize),
    x: isize,
    y: isize,
    color: Color,
) {
    let x = x + translate.x;
    let y = y + translate.y;
    let idx = ((x + y * width as isize) * 4) as usize;
    if x >= 0 && y >= 0 && x < width as isize && y < height as isize && clip.is_valid((x, y)) {
        match color.a {
            255 => set_pixel(buffer, idx, color),
            0 => {}
            _ => blend_pixel(buffer, idx, color),
        }
    }
}

/// Set the RGB values for a pixel
///
/// Generally you should use [update_pixel] instead
///
/// This ignores alpha, so 255,0,0,0 will draw a red pixel
fn set_pixel(buffer: &mut [u8], idx: usize, color: Color) {
    if idx < buffer.len() {
        buffer[idx] = color.r;
        buffer[idx + 1] = color.g;
        buffer[idx + 2] = color.b;
        buffer[idx + 3] = color.a;
    }
}

/// Set the RGB values for a pixel by blending it with the provided color
/// This method uses alpha blending
/// Generally you should use [update_pixel] instead
fn blend_pixel(buffer: &mut [u8], idx: usize, color: Color) {
    let existing_color = Color {
        r: buffer[idx],
        g: buffer[idx + 1],
        b: buffer[idx + 2],
        a: buffer[idx + 3],
    };
    let new_color = existing_color.blend(color);
    buffer[idx] = new_color.r;
    buffer[idx + 1] = new_color.g;
    buffer[idx + 2] = new_color.b;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    use crate::shapes::polyline::Segment::*;
    use crate::text::pos::TextPos::Px;

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

    #[test]
    fn check_draw() {
        let mut buf = [0; 400];
        let mut graphics = Graphics::new(&mut buf, 10, 10).unwrap();

        let drawable = Drawable::from_obj(Line::new((10, 10), (20, 20)), stroke(RED));
        let text = Text::new("", Px(1, 1), WHITE);
        let polyline = Polyline::new(
            vec![Start(Coord::new(0, 0)), LineTo(Coord::new(0, 0))],
            WHITE,
        );

        graphics.draw(&drawable);
        graphics.draw(&text);
        graphics.draw(&polyline);
    }
}
