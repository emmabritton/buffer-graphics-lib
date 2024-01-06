use crate::drawing::Renderable;
use crate::image::Image;
use crate::{sized_renderable, Graphics};
use graphics_shapes::coord::Coord;

sized_renderable!(
    RenderableImage,
    Image,
    |img: &Image| (img.width(), img.height()),
    |g: &mut Graphics, img: &Image| g.draw_image((0, 0), img)
);
