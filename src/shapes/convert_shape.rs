//! Shape conversion
//!
//! Circle -> Rect
//!
//! Line -> Rect
//!
//! Rect -> Line
//!

use crate::shapes::circle::Circle;
use crate::shapes::line::Line;
use crate::shapes::rect::Rect;
use crate::shapes::{DrawType, Shape};

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

impl From<Rect> for Line {
    fn from(rect: Rect) -> Self {
        let color = match rect.draw_type() {
            DrawType::Stroke(c) => c,
            DrawType::Fill(c) => c,
        };
        Line::new(rect.points()[0], rect.points()[1], color)
    }
}
