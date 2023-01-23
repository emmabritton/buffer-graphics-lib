pub mod collection;
pub mod rendering;

use crate::drawable::{DrawType, Drawable};
use graphics_shapes::circle::Circle;
use graphics_shapes::coord::Coord;
use graphics_shapes::ellipse::Ellipse;
use graphics_shapes::line::Line;
use graphics_shapes::polygon::Polygon;
use graphics_shapes::rect::Rect;
use graphics_shapes::triangle::Triangle;
use graphics_shapes::Shape;

pub trait CreateDrawable<T: Clone> {
    fn from_obj(obj: T, draw_type: DrawType) -> Drawable<T>;
}

impl CreateDrawable<Line> for Drawable<Line> {
    fn from_obj(line: Line, draw_type: DrawType) -> Drawable<Line> {
        let drawing_points = vec![
            Coord::new(line.left(), line.top()),
            Coord::new(line.right(), line.bottom()),
        ];
        Drawable::new(line, draw_type, drawing_points)
    }
}

impl CreateDrawable<Rect> for Drawable<Rect> {
    fn from_obj(rect: Rect, draw_type: DrawType) -> Drawable<Rect> {
        let drawing_points = vec![
            Coord::new(rect.left(), rect.top()),
            Coord::new(rect.right(), rect.bottom()),
        ];
        Drawable::new(rect, draw_type, drawing_points)
    }
}

impl CreateDrawable<Triangle> for Drawable<Triangle> {
    fn from_obj(triangle: Triangle, draw_type: DrawType) -> Drawable<Triangle> {
        let points = triangle.points();
        let mut sorted_points: Vec<Coord> = vec![points[0], points[1], points[2]];
        sorted_points.sort_by_key(|c| c.y);
        Drawable::new(triangle, draw_type, sorted_points)
    }
}

macro_rules! create_drawable_from_points {
    ($shape: ty) => {
        impl CreateDrawable<$shape> for Drawable<$shape> {
            fn from_obj(shape: $shape, draw_type: DrawType) -> Drawable<$shape> {
                let points = shape.points();
                Drawable::new(shape, draw_type, points)
            }
        }
    };
}

create_drawable_from_points!(Polygon);
create_drawable_from_points!(Circle);
create_drawable_from_points!(Ellipse);
