pub mod collection;
pub mod polyline;

use crate::drawable::{DrawType, Drawable};
use crate::drawing::Renderable;
use crate::Graphics;
use graphics_shapes::prelude::*;
use graphics_shapes::shape_box::ShapeBox;

impl<S: Shape + Clone> Renderable<S> for Drawable<S> {
    fn render(&self, graphics: &mut Graphics) {
        let color = self.draw_type().color();

        for px in self.drawing_points() {
            graphics.set_pixel(px.x, px.y, color);
        }
    }
}

pub trait CreateDrawable<T: Clone> {
    fn from_obj(obj: T, draw_type: DrawType) -> Drawable<T>;
}

impl CreateDrawable<Line> for Drawable<Line> {
    fn from_obj(line: Line, draw_type: DrawType) -> Drawable<Line> {
        let drawing_points = line.outline_pixels();
        Drawable::new(line, draw_type, drawing_points)
    }
}

macro_rules! create_drawable_from_points {
    ($shape: ty) => {
        impl CreateDrawable<$shape> for Drawable<$shape> {
            fn from_obj(shape: $shape, draw_type: DrawType) -> Drawable<$shape> {
                let drawing_points = if draw_type.is_stroke() {
                    shape.outline_pixels()
                } else {
                    shape.filled_pixels()
                };
                Drawable::new(shape, draw_type, drawing_points)
            }
        }
    };
}

create_drawable_from_points!(Triangle);
create_drawable_from_points!(Rect);
create_drawable_from_points!(Polygon);
create_drawable_from_points!(Circle);
create_drawable_from_points!(Ellipse);
create_drawable_from_points!(ShapeBox);
