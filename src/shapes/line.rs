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
        let color = match draw_type {
            DrawType::Stroke(c) => c,
            DrawType::Fill(c) => c,
        };
        Line::new(self.start, self.end, color)
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

    fn points(&self) -> Vec<Coord> {
        vec![self.start, self.end]
    }
}

impl Renderable for Line {
    fn render(&self, graphics: &mut Graphics) {
        if self.start.x == self.end.x {
            for y in self.start.y..self.end.y {
                graphics.update_pixel(self.start.x, y, self.color);
            }
        } else if self.start.y == self.end.y {
            for x in self.start.x..self.end.x {
                graphics.update_pixel(x, self.start.y, self.color);
            }
        } else {
            let mut delta = 0;
            let x1 = self.start.x as isize;
            let y1 = self.start.y as isize;
            let x2 = self.end.x as isize;
            let y2 = self.end.y as isize;
            let dx = isize::abs(x2 - x1);
            let dy = isize::abs(y2 - y1);
            let dx2 = dx * 2;
            let dy2 = dy * 2;
            let ix: isize = if x1 < x2 { 1 } else { -1 };
            let iy: isize = if y1 < y2 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;
            if dx >= dy {
                loop {
                    graphics.update_pixel(x, y, self.color);
                    if x == x2 {
                        break;
                    }
                    x += ix;
                    delta += dy2;
                    if delta > dx {
                        y += iy;
                        delta -= dx2;
                    }
                }
            } else {
                loop {
                    graphics.update_pixel(x, y, self.color);
                    if y == y2 {
                        break;
                    }
                    y += iy;
                    delta += dx2;
                    if delta > dy {
                        x += ix;
                        delta -= dy2;
                    }
                }
            }
        }
    }
}
