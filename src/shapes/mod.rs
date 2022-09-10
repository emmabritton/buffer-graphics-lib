pub mod rendering;

use crate::drawable::{DrawType, Drawable};
use graphics_shapes::circle::Circle;
use graphics_shapes::coord::Coord;
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
        let (start, end) = if line.start().x < line.end().x || line.start().y < line.end().y {
            (line.start(), line.end())
        } else {
            (line.end(), line.start())
        };
        Drawable::new(line, draw_type, vec![start, end])
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

impl CreateDrawable<Circle> for Drawable<Circle> {
    fn from_obj(circle: Circle, draw_type: DrawType) -> Drawable<Circle> {
        let points = circle.points();
        Drawable::new(circle, draw_type, points)
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

impl CreateDrawable<Polygon> for Drawable<Polygon> {
    fn from_obj(polygon: Polygon, draw_type: DrawType) -> Drawable<Polygon> {
        let points = polygon.points();
        Drawable::new(polygon, draw_type, points)
    }
}
