use crate::color::Color;
use crate::coord::Coord;
use crate::drawing::Renderable;
use crate::shapes::{DrawType, Shape};
use crate::Graphics;

#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Line {
    start: Coord,
    end: Coord,
    len: isize,
    color: Color,
}

impl Line {
    pub fn new<P1: Into<Coord>, P2: Into<Coord>>(start: P1, end: P2, color: Color) -> Self {
        let start = start.into();
        let end = end.into();
        let diff = start.diff(end);
        let len = ((((diff.0 ^ 2) as f32) + (diff.1 ^ 2) as f32).sqrt()) as isize;
        Self {
            start,
            end,
            len,
            color,
        }
    }

    pub fn length(&self) -> isize {
        self.len
    }
}

impl Shape for Line {
    fn draw_type(&self) -> DrawType {
        DrawType::Stroke(self.color)
    }

    fn with_draw_type(&self, draw_type: DrawType) -> Self {
        Line::new(self.start, self.end, draw_type.color())
    }

    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self {
        let delta = delta.into();
        let start = self.start + delta;
        let end = self.end + delta;
        Line::new(start, end, self.color)
    }

    fn move_to<P: Into<Coord>>(&self, xy: P) -> Self {
        let start = xy.into();
        let end = start + Coord::from(self.start.diff(self.end));
        Line::new(start, end, self.color)
    }

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let p = point.into();
        let min = Coord::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y));
        let max = Coord::new(self.start.x.max(self.end.x), self.start.y.max(self.end.y));
        if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
            let normal = (self.end - self.start).perpendicular();
            let d = self.start - p;
            if d.dot_product(normal) == 0 {
                return true;
            }
        }
        false
    }

    fn points(&self) -> Vec<Coord> {
        vec![self.start, self.end]
    }
}

impl Renderable for Line {
    fn render(&self, graphics: &mut Graphics) {
        graphics.draw_line(self.start, self.end, self.color)
    }
}
