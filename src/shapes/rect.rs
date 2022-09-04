use crate::coord::Coord;
use crate::drawing::Renderable;
use crate::shapes::{DrawType, Shape};
use crate::Graphics;

#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rect {
    start: Coord,
    end: Coord,
    width: isize,
    height: isize,
    draw_type: DrawType,
}

impl Rect {
    /// Creates a new Rect
    ///
    /// If necessary the points will be swapped so start_x < end_x and start_y < end_y
    pub fn new<P1: Into<Coord>, P2: Into<Coord>>(start: P1, end: P2, draw_type: DrawType) -> Self {
        let start = start.into();
        let end = end.into();
        let start_x = start.x.min(end.x);
        let end_x = start.x.max(end.x);
        let start_y = start.y.min(end.y);
        let end_y = start.y.max(end.y);
        let width = end_x - start_x;
        let height = end_y - start_y;
        Self {
            start: Coord::new(start_x, start_y),
            end: Coord::new(end_x, end_y),
            width,
            height,
            draw_type,
        }
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.width
    }

    pub fn topleft(&self) -> Coord {
        self.points()[0]
    }

    pub fn bottomright(&self) -> Coord {
        self.points()[1]
    }

    /// Union this rect and another, the result will contain both rectangles
    /// Generally, this means the result will be bigger than self
    pub fn union(&self, other: &Rect) -> Rect {
        let x1 = self.start.x.min(other.start.x);
        let y1 = self.start.y.min(other.start.y);
        let x2 = self.end.x.max(other.end.x);
        let y2 = self.end.y.max(other.end.y);

        Rect::new((x1, y1), (x2, y2), self.draw_type)
    }

    /// Intersect this rect and another, the result will contain the area covered by both rectangles
    /// Generally, this means the result will be smaller than self
    ///
    /// # Returns
    ///
    /// self if rectangles do not intersect
    pub fn intersect(&self, other: &Rect) -> Rect {
        let x1 = self.start.x.max(other.start.x);
        let y1 = self.start.y.max(other.start.y);
        let x2 = self.end.x.min(other.end.x);
        let y2 = self.end.y.min(other.end.y);
        if x1 < x2 && y1 < y2 {
            Rect::new((x1, y1), (x2, y2), self.draw_type)
        } else {
            self.clone()
        }
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        let x1 = self.start.x.max(other.start.x);
        let y1 = self.start.y.max(other.start.y);
        let x2 = self.end.x.min(other.end.x);
        let y2 = self.end.y.min(other.end.y);

        x1 < x2 && y1 < y2
    }
}

impl Shape for Rect {
    fn draw_type(&self) -> DrawType {
        self.draw_type
    }

    fn with_draw_type(&self, draw_type: DrawType) -> Self {
        Rect::new(self.start, self.end, draw_type)
    }

    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self {
        let delta = delta.into();
        let start = self.start + delta;
        let end = self.end + delta;
        Rect::new(start, end, self.draw_type)
    }

    fn move_to<P: Into<Coord>>(&self, xy: P) -> Self {
        let start = xy.into();
        let end = start + Coord::from(self.start.diff(self.end));
        Rect::new(start, end, self.draw_type)
    }

    fn points(&self) -> Vec<Coord> {
        vec![self.start, self.end]
    }
}

impl Renderable for Rect {
    fn render(&self, graphics: &mut Graphics) {
        match self.draw_type {
            DrawType::Stroke(color) => {
                for x in self.start.x..self.end.x {
                    graphics.update_pixel(x, self.start.y, color);
                    graphics.update_pixel(x, self.end.y, color);
                }
                for y in self.start.y..=self.end.y {
                    graphics.update_pixel(self.start.x, y, color);
                    graphics.update_pixel(self.end.x, y, color);
                }
            }
            DrawType::Fill(color) => {
                for x in self.start.x..self.end.x {
                    for y in self.start.y..self.end.y {
                        graphics.update_pixel(x, y, color);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::BLACK;

    #[test]
    fn union() {
        let rect = Rect::new((10, 10), (20, 20), DrawType::Stroke(BLACK));
        let other = Rect::new((15, 15), (25, 25), DrawType::Stroke(BLACK));

        let union = rect.union(&other);

        assert_eq!(
            union,
            Rect::new((10, 10), (25, 25), DrawType::Stroke(BLACK))
        );

        let rect = Rect::new((50, 1), (50, 100), DrawType::Stroke(BLACK));
        let other = Rect::new((1, 50), (100, 50), DrawType::Stroke(BLACK));

        let union = rect.union(&other);

        assert_eq!(
            union,
            Rect::new((1, 1), (100, 100), DrawType::Stroke(BLACK))
        );
    }

    #[test]
    fn intersects() {
        let rect = Rect::new((10, 10), (20, 20), DrawType::Stroke(BLACK));
        let does_intersect = Rect::new((15, 15), (25, 25), DrawType::Stroke(BLACK));
        let doesnt_intersect = Rect::new((30, 30), (40, 40), DrawType::Stroke(BLACK));

        assert!(rect.intersects(&does_intersect));
        assert!(!rect.intersects(&doesnt_intersect));
    }

    #[test]
    fn intersect() {
        let rect = Rect::new((10, 10), (20, 20), DrawType::Stroke(BLACK));
        let other = Rect::new((15, 15), (25, 25), DrawType::Stroke(BLACK));

        let intersection = rect.intersect(&other);

        assert_eq!(
            intersection,
            Rect::new((15, 15), (20, 20), DrawType::Stroke(BLACK))
        );

        let rect = Rect::new((50, 1), (51, 100), DrawType::Stroke(BLACK));
        let other = Rect::new((1, 50), (100, 51), DrawType::Stroke(BLACK));

        let intersection = rect.intersect(&other);

        assert_eq!(
            intersection,
            Rect::new((50, 50), (51, 51), DrawType::Stroke(BLACK))
        );

        let rect = Rect::new((10, 10), (20, 20), DrawType::Stroke(BLACK));
        let doesnt_intersect = Rect::new((30, 30), (40, 40), DrawType::Stroke(BLACK));

        assert_eq!(rect.intersect(&doesnt_intersect), rect);
    }
}
