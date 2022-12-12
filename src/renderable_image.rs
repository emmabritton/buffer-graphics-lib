use crate::drawing::Renderable;
use crate::image::Image;
use crate::Graphics;
use graphics_shapes::coord::Coord;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DrawOffset {
    TopLeft,
    Center,
    Custom(Coord)
}

#[derive(Debug, Clone)]
pub struct RenderableImage {
    image: Image,
    xy: Coord,
    offset: DrawOffset
}

impl RenderableImage {
    pub fn new(image: Image, xy: Coord, offset: DrawOffset) -> Self {
        Self { image, xy, offset }
    }
}

impl RenderableImage {
    pub fn set_position<P: Into<Coord>>(&mut self, new_position: P) {
        self.xy = new_position.into();
    }

    pub fn update_position<P: Into<Coord>>(&mut self, delta: P) {
        self.xy = self.xy + delta.into();
    }

    pub fn set_offset(&mut self, offset: DrawOffset) {
        self.offset = offset;
    }
}

impl Renderable for RenderableImage {
    fn render(&self, graphics: &mut Graphics) {
        let offset = match self.offset {
            DrawOffset::TopLeft => (0,0).into(),
            DrawOffset::Center => (-(self.image.width()/2), -(self.image.height()/2)).into(),
            DrawOffset::Custom(coord) => coord
        };

        graphics.draw_image(self.xy + offset, &self.image);
    }
}
