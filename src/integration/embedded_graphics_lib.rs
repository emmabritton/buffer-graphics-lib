use std::convert::Infallible;
use embedded_graphics_core::Pixel;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::{Dimensions, DrawTarget, Point, RgbColor, Size};
use embedded_graphics_core::primitives::Rectangle;
use graphics_shapes::coord;
use graphics_shapes::rect::Rect;
use ici_files::prelude::Color;
use crate::Graphics;
use crate::prelude::fill;

impl Dimensions for Graphics<'_> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0,0), Size::new(self.width as u32, self.height as u32))
    }
}

impl DrawTarget for Graphics<'_> {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> where I: IntoIterator<Item=Pixel<Self::Color>> {
        for pixel in pixels {
            self.set_pixel(pixel.0.x as isize, pixel.0.y as isize, convert(pixel.1));
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let rect = Rect::new_with_size(coord!(area.top_left.x, area.top_left.y), area.size.width as usize, area.size.height as usize);
        self.draw_rect(rect, fill(convert(color)));
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear(convert(color));
        Ok(())
    }
}

fn convert(color: Rgb888) -> Color {
    Color::new(color.r(), color.g(), color.b(), 255)
}