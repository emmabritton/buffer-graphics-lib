use crate::clipping::Clip;
use crate::drawable::{DrawType, Drawable};
use crate::image::Image;
use crate::prelude::PixelFont;
use crate::shapes::CreateDrawable;
use crate::text::format::TextFormat;
use crate::text::pos::TextPos;
use crate::text::{chr_to_code, Text};
use crate::GraphicsBuffer::RgbaU8;
use crate::{Graphics, GraphicsBuffer, GraphicsError};
use graphics_shapes::circle::Circle;
use graphics_shapes::coord::Coord;
use graphics_shapes::polygon::Polygon;
use graphics_shapes::prelude::Ellipse;
use graphics_shapes::rect::Rect;
use graphics_shapes::triangle::Triangle;
use ici_files::palette::simplify_palette_to_fit;
use ici_files::prelude::*;
use std::collections::HashSet;
use std::mem::swap;

/// Represents anything that [Graphics] can render
pub trait Renderable<T> {
    fn render(&self, graphics: &mut Graphics);
}

#[inline]
pub(crate) fn index_u8(width: usize, x: usize, y: usize) -> usize {
    (x + y * width) * 4
}

#[inline]
pub(crate) fn index_u32(width: usize, x: usize, y: usize) -> usize {
    x + y * width
}

pub(crate) fn clear_u8(buffer: &mut GraphicsBuffer, color: Color) {
    if let RgbaU8(buffer) = buffer {
        buffer.chunks_exact_mut(4).for_each(|px| {
            px[0] = color.r;
            px[1] = color.g;
            px[2] = color.b;
            px[3] = color.a;
        });
    } else {
        panic!(
            "clear_u8 called on non u8 buffer, please create GitHub issue for buffer-graphics-lib"
        )
    }
}

pub(crate) fn clear_u32(buffer: &mut GraphicsBuffer, color: Color) {
    #[allow(clippy::type_complexity)] //it's internal to this method
    let result: Option<(&mut &mut [u32], fn(Color) -> u32)> = match buffer {
        RgbaU8(_) => None,
        GraphicsBuffer::RgbaU32(buf) => Some((buf, Color::to_rgba)),
        GraphicsBuffer::ArgbU32(buf) => Some((buf, Color::to_argb)),
    };
    if let Some((buffer, method)) = result {
        let color = method(color);
        buffer.iter_mut().for_each(|p| *p = color);
    } else {
        panic!("clear_u32 called on non u32 buffer, please create GitHub issue for buffer-graphics-lib")
    }
}

impl Graphics<'_> {
    /// Convert an x,y coord to idx for use with `self.pixels`
    #[inline(always)]
    pub fn index(&self, x: usize, y: usize) -> usize {
        (self.index_method)(self.width, x, y)
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
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
    #[inline(always)]
    pub fn get_translate(&self) -> Coord {
        self.translate
    }

    /// Set the canvas offset in pixels
    ///
    /// All drawing commands will be offset by this value
    ///
    /// # Returns
    /// The previous translation value
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
        let pixels = self.buffer.to_pixels();
        Image::new(pixels, self.width, self.height)
            .expect("Copy to image failed, please create GitHub issue for buffer-graphics-lib")
    }

    /// Copy entire pixels array to an indexed image
    /// `simplify_palette` if true and there's more than 255 colours, this will simplify/merge the palette until there are under 255 colours
    ///
    /// # Errors
    ///
    /// * `GraphicsError::TooManyColors` Over 255 colors have been used and `simplify_palette` was false
    /// * `GraphicsError::TooBig` Image is bigger than 255x255
    /// * `GraphicsError::ImageError` Something went wrong creating the IndexedImage
    pub fn copy_to_indexed_image(
        &self,
        simplify_palette: bool,
    ) -> Result<IndexedImage, GraphicsError> {
        if self.width > 255 || self.height > 255 {
            return Err(GraphicsError::TooBig(self.width, self.height));
        }
        let width = self.width as u8;
        let height = self.height as u8;
        let pixels = self.buffer.to_pixels();
        let colors: HashSet<Color> = HashSet::from_iter(pixels.iter().copied());
        let colors = if colors.len() > 255 {
            if simplify_palette {
                simplify_palette_to_fit(&colors.into_iter().collect::<Vec<Color>>(), 255)
            } else {
                return Err(GraphicsError::TooManyColors);
            }
        } else {
            colors.into_iter().collect()
        };

        let pixels = pixels
            .iter()
            .map(|c| {
                colors
                    .iter()
                    .position(|o| o == c)
                    .unwrap_or_else(|| panic!()) as u8
            })
            .collect();

        IndexedImage::new(width, height, colors, pixels).map_err(GraphicsError::ImageError)
    }

    /// Get top left pixel coord for letter col row
    pub fn get_px_for_char(col: usize, row: usize, font: &PixelFont) -> (usize, usize) {
        (col * font.char_width(), row * font.line_height())
    }

    /// Draw an image at `x`, `y`
    /// If the image definitely will draw inside the window you can use [draw_image_unchecked] instead
    pub fn draw_image<P: Into<Coord>>(&mut self, xy: P, image: &Image) {
        let xy = xy.into();
        let mut x = 0;
        let mut y = 0;
        for pixel in image.pixels() {
            update_pixel(
                &mut self.buffer,
                &self.translate,
                &self.clip,
                (self.width, self.height),
                xy.x + x as isize,
                xy.y + y,
                *pixel,
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
                    &mut self.buffer,
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
                    &mut self.buffer,
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
                &mut self.buffer,
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
                    &mut self.buffer,
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
                    &mut self.buffer,
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
                        &mut self.buffer,
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
                        &mut self.buffer,
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
    #[inline(always)]
    pub fn draw_offset<T, P: Into<Coord>>(&mut self, xy: P, renderable: &dyn Renderable<T>) {
        self.with_translate(xy.into(), |g| renderable.render(g));
    }

    /// Draw renderable
    #[inline(always)]
    pub fn draw<T>(&mut self, renderable: &dyn Renderable<T>) {
        renderable.render(self);
    }

    /// Get the RGB values for a pixel
    /// Alpha will always be 255
    ///
    /// If `use_translate` is true then the x,y will be updated with `self.translate`
    ///
    /// # Returns
    ///
    /// `Some(Color)` if x,y is within bounds
    #[inline]
    pub fn get_pixel(&self, x: isize, y: isize, use_translate: bool) -> Option<Color> {
        let (x, y) = if use_translate {
            (x + self.translate.x, y + self.translate.y)
        } else {
            (x, y)
        };

        let len = self.width * self.height;
        if x >= 0 && y >= 0 && x < self.width as isize {
            let idx = self.index(x as usize, y as usize);
            if idx < len {
                return Some(self.buffer.get_color(idx));
            }
        }

        None
    }

    /// Set every pixel to `color`, this ignores translate and clip
    #[inline(always)]
    pub fn clear(&mut self, color: Color) {
        (self.clear_method)(&mut self.buffer, color);
    }

    /// Set/blend every pixel with `color`, same as [clear] but this follows translate and clip
    pub fn clear_aware(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x as isize, y as isize, color);
            }
        }
    }

    /// Draw `chr` at `pos`
    ///
    /// # Usage
    /// ```
    ///# use buffer_graphics_lib::prelude::*;
    ///# fn doc(graphics: &mut Graphics) {
    ///     graphics.draw_letter((20,20), 'A', PixelFont::Limited3x5, BLUE);
    ///# }
    /// ```
    #[inline(always)]
    pub fn draw_letter(&mut self, pos: (isize, isize), chr: char, font: PixelFont, color: Color) {
        self.draw_ascii_letter(pos, chr_to_code(chr), font, color);
    }

    /// Shouldn't be called in normal usage
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
                PixelFont::Limited3x5 => &custom._3x5,
            }
        } else {
            font.pixels(code)
        };

        for x in 0..width {
            for y in 0..height {
                let i = x + y * width;
                if px[i] {
                    update_pixel(
                        &mut self.buffer,
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

    /// Draw `text` on screen at `pos` using `format`
    ///
    /// # Params
    /// * `format` See [TextFormat], can be
    ///     * `TextFormat`
    ///     * `Color`
    ///     * `(Color, PixelFont)`
    ///     * `(Color, PixelFont, Positioning)`
    ///     * `(Color, PixelFont, WrappingStrategy, Positioning)`
    ///     * `(Color, PixelFont, WrappingStrategy)`
    ///     * `(Color, PixelFont, WrappingStrategy, f32)` (f32 = line height)
    ///     * `(Color, PixelFont, WrappingStrategy, f32, f32)` (1st f32 = line height, 2nd f32 = char width)
    ///     * `(Color, PixelFont, WrappingStrategy, f32, f32, Positioning)` (1st f32 = line height, 2nd f32 = char width)
    ///
    /// # Usage
    ///
    /// ```
    ///# use buffer_graphics_lib::prelude::*;
    ///# fn doc(graphics: &mut Graphics) {
    ///  //simple example
    ///  graphics.draw_text("Test", TextPos::ColRow(1,1), RED);
    ///# }
    ///
    /// //full example
    /// fn draw_message(graphics: &mut Graphics, msg: String) {
    ///     let width_in_columns = PixelFont::Standard6x7.get_max_characters(graphics.width(), graphics.height()).0;
    ///     graphics.draw_text(&msg, TextPos::px(coord!(8,8)), (BLACK, PixelFont::Standard6x7, WrappingStrategy::AtCol(width_in_columns - 1)));
    /// }
    /// ```
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

    /// Update a pixel color, replacing or blending depending on whether `color`s alpha is 255 or not
    ///
    /// If the alpha is 0 the call does nothing
    #[inline]
    pub fn set_pixel(&mut self, x: isize, y: isize, color: Color) {
        update_pixel(
            &mut self.buffer,
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
    buffer: &mut GraphicsBuffer,
    translate: &Coord,
    clip: &Clip,
    (width, height): (usize, usize),
    x: isize,
    y: isize,
    color: Color,
) {
    let x = x + translate.x;
    let y = y + translate.y;
    match buffer {
        RgbaU8(buffer) => {
            let idx = ((x + y * width as isize) * 4) as usize;
            if x >= 0
                && y >= 0
                && x < width as isize
                && y < height as isize
                && clip.is_valid((x, y))
            {
                match color.a {
                    255 => set_pixel_u8_rgba(buffer, idx, color),
                    0 => {}
                    _ => blend_pixel_u8_rgba(buffer, idx, color),
                }
            }
        }
        GraphicsBuffer::RgbaU32(buffer) => {
            let idx = (x + y * width as isize) as usize;
            if x >= 0
                && y >= 0
                && x < width as isize
                && y < height as isize
                && clip.is_valid((x, y))
            {
                match color.a {
                    255 => set_pixel_32(buffer, idx, color, Color::to_rgba),
                    0 => {}
                    _ => blend_pixel_32(buffer, idx, color, Color::to_rgba),
                }
            }
        }
        GraphicsBuffer::ArgbU32(buffer) => {
            let idx = (x + y * width as isize) as usize;
            if x >= 0
                && y >= 0
                && x < width as isize
                && y < height as isize
                && clip.is_valid((x, y))
            {
                match color.a {
                    255 => set_pixel_32(buffer, idx, color, Color::to_argb),
                    0 => {}
                    _ => blend_pixel_32(buffer, idx, color, Color::to_argb),
                }
            }
        }
    }
}

/// Set the RGB values for a pixel
///
/// Generally you should use [update_pixel] instead
///
/// This ignores alpha, so 255,0,0,0 will draw a red pixel
fn set_pixel_32(buffer: &mut [u32], idx: usize, color: Color, conv: fn(Color) -> u32) {
    if idx < buffer.len() {
        buffer[idx] = conv(color);
    }
}

/// Set the RGB values for a pixel by blending it with the provided color
/// This method uses alpha blending
/// Generally you should use [update_pixel] instead
fn blend_pixel_32(buffer: &mut [u32], idx: usize, color: Color, conv: fn(Color) -> u32) {
    let existing_color = Color::from_rgba(buffer[idx]);
    let new_color = existing_color.blend(color);
    buffer[idx] = conv(new_color);
}

/// Set the RGB values for a pixel
///
/// Generally you should use [update_pixel] instead
///
/// This ignores alpha, so 255,0,0,0 will draw a red pixel
fn set_pixel_u8_rgba(buffer: &mut [u8], idx: usize, color: Color) {
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
fn blend_pixel_u8_rgba(buffer: &mut [u8], idx: usize, color: Color) {
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
        let mut graphics = Graphics::new_u8_rgba(&mut buf, 10, 10).unwrap();
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
        let mut graphics = Graphics::new_u8_rgba(&mut buf, 10, 10).unwrap();

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
