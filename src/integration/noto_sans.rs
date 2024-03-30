use crate::prelude::Renderable;
use crate::Graphics;
use ici_files::color::Color;
use noto_sans_mono_bitmap::RasterizedChar;

impl Renderable<RasterizedChar> for RasterizedChar {
    fn render(&self, graphics: &mut Graphics) {
        for (y, row) in self.raster().iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                graphics.set_pixel(x as isize, y as isize, Color::gray(*pixel));
            }
        }
    }
}
