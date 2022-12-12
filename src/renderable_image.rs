use crate::drawing::Renderable;
use crate::image::Image;
use crate::Graphics;
use graphics_shapes::coord::Coord;

pub struct RenderableImage {
    image: Image,
    xy: Coord,
}

impl RenderableImage {
    pub fn new(image: Image, xy: Coord) -> Self {
        Self { image, xy }
    }
}

impl RenderableImage {
    pub fn set_position<P: Into<Coord>>(&mut self, new_position: P) {
        self.xy = new_position.into();
    }

    pub fn update_position<P: Into<Coord>>(&mut self, delta: P) {
        self.xy = self.xy + delta.into();
    }
}

impl Renderable for RenderableImage {
    fn render(&self, graphics: &mut Graphics) {
        graphics.draw_image(self.xy, &self.image);
    }
}
