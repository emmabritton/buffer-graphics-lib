use crate::drawing::Renderable;
use crate::{sized_renderable, Graphics};
use graphics_shapes::coord::Coord;
use ici_files::prelude::*;

sized_renderable!(
    RenderableIndexedImage,
    IndexedImage,
    |img: &IndexedImage| (img.width() as usize, img.height() as usize),
    |g: &mut Graphics, img: &IndexedImage| g.draw_indexed_image((0, 0), img)
);
sized_renderable!(
    RenderableAnimatedImage,
    AnimatedIndexedImage,
    |img: &AnimatedIndexedImage| (img.width() as usize, img.height() as usize),
    |g: &mut Graphics, img: &AnimatedIndexedImage| g.draw_animated_image((0, 0), img)
);
sized_renderable!(
    RenderableWrappedImage,
    IndexedWrapper,
    |img: &IndexedWrapper| (img.width() as usize, img.height() as usize),
    |g: &mut Graphics, img: &IndexedWrapper| g.draw_wrapped_image((0, 0), img)
);

impl RenderableAnimatedImage {
    pub fn update(&mut self, delta: f64) {
        self.item.update(delta);
    }
}

impl RenderableWrappedImage {
    pub fn update(&mut self, delta: f64) {
        self.item.update(delta);
    }
}
