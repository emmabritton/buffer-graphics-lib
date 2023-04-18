use crate::drawing::Renderable;
use crate::Graphics;
use graphics_shapes::coord::Coord;
use ici_files::animated::AnimatedIndexedImage;
use ici_files::prelude::IndexedImage;

pub struct RenderableIndexedImage {
    pub xy: Coord,
    pub image: IndexedImage,
}

impl RenderableIndexedImage {
    pub fn new(xy: Coord, image: IndexedImage) -> Self {
        Self { xy, image }
    }
}

impl Renderable<RenderableIndexedImage> for RenderableIndexedImage {
    fn render(&self, graphics: &mut Graphics) {
        graphics.draw_indexed_image(self.xy, &self.image);
    }
}

pub struct RenderableAnimatedImage {
    pub xy: Coord,
    pub image: AnimatedIndexedImage,
}

impl RenderableAnimatedImage {
    pub fn new(xy: Coord, image: AnimatedIndexedImage) -> Self {
        Self { xy, image }
    }
}

impl Renderable<RenderableAnimatedImage> for RenderableAnimatedImage {
    fn render(&self, graphics: &mut Graphics) {
        graphics.draw_animated_image(self.xy, &self.image);
    }
}

impl RenderableAnimatedImage {
    pub fn update(&mut self, delta: f64) {
        self.image.update(delta);
    }
}
