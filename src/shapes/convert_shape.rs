//! Shape conversion
//!
//! Circle -> Rect
//!
//! Line -> Rect
//!
//! Triangle -> Rect
//!             Circle
//!

use crate::Coord;
use crate::shapes::circle::Circle;
use crate::shapes::line::Line;
use crate::shapes::rect::Rect;
use crate::shapes::Shape;
use crate::shapes::triangle::Triangle;

impl From<Circle> for Rect {
    fn from(circle: Circle) -> Self {
        let center = circle.center();
        let r = circle.radius() as isize;
        Rect::new(
            (center.x - r, center.y - r),
            (center.x + r, center.y + r),
            circle.draw_type(),
        )
    }
}

impl From<Line> for Rect {
    fn from(line: Line) -> Self {
        Rect::new(line.points()[0], line.points()[1], line.draw_type())
    }
}


impl From<Triangle> for Rect {
    fn from(triangle: Triangle) -> Self {
        let x1 = triangle.leftmost_point().x;
        let y1 = triangle.topmost_point().y;
        let x2 = triangle.rightmost_point().x;
        let y2 = triangle.bottommost_point().y;
        Rect::new((x1, y1), (x2, y2), triangle.draw_type())
    }
}

impl From<Triangle> for Circle {
    fn from(triangle: Triangle) -> Self {
        let x1 = triangle.leftmost_point().x;
        let y1 = triangle.topmost_point().y;
        let x2 = triangle.rightmost_point().x;
        let y2 = triangle.bottommost_point().y;
        let top_left = Coord::new(x1, y1);
        let bottom_right = Coord::new(x2, y2);
        let center = top_left.mid_point(bottom_right);
        let radius = (center.x - x1).max(top_left.y - y1).abs() as usize;
        Circle::new(center, radius, triangle.draw_type())
    }
}