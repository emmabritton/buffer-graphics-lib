use mint::Point2;
use crate::{Coord, Graphics};
use crate::shapes::{DrawType, Shape};

#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use crate::color::Color;
use crate::drawing::Renderable;

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Triangle {
    points: [Coord; 3],
    leftmost_point: usize,
    topmost_point: usize,
    rightmost_point: usize,
    bottommost_point: usize,
    draw_type: DrawType,
}

impl Triangle {
    /// Points will be resorted so the first is the highest
    pub fn new<P1: Into<Coord>, P2: Into<Coord>, P3: Into<Coord>>(corner1: P1, corner2: P2, corner3: P3, draw_type: DrawType) -> Self {
        let c1 = corner1.into();
        let c2 = corner2.into();
        let c3 = corner3.into();

        let mut sorted_points: Vec<Coord> = vec![c1, c2, c3];
        sorted_points.sort_by_key(|c| c.y);

        let leftmost_point = [c1.x, c2.x, c3.x].iter().enumerate().min().unwrap().0;
        let rightmost_point = [c1.x, c2.x, c3.x].iter().enumerate().max().unwrap().0;
        let topmost_point = [c1.y, c2.y, c3.y].iter().enumerate().min().unwrap().0;
        let bottommost_point = [c1.y, c2.y, c3.y].iter().enumerate().max().unwrap().0;
        let points = [sorted_points[0], sorted_points[1], sorted_points[2]];

        Self {
            points,
            leftmost_point,
            topmost_point,
            rightmost_point,
            bottommost_point,
            draw_type,
        }
    }

    pub fn leftmost_point(&self) -> Coord {
        self.points[self.leftmost_point]
    }

    pub fn leftmost_point_idx(&self) -> usize {
        self.leftmost_point
    }

    pub fn rightmost_point(&self) -> Coord {
        self.points[self.rightmost_point]
    }

    pub fn rightmost_point_idx(&self) -> usize {
        self.rightmost_point
    }

    pub fn topmost_point(&self) -> Coord {
        self.points[self.topmost_point]
    }

    pub fn topmost_point_idx(&self) -> usize {
        self.topmost_point
    }

    pub fn bottommost_point(&self) -> Coord {
        self.points[self.bottommost_point]
    }

    pub fn bottommost_point_idx(&self) -> usize {
        self.bottommost_point
    }
}

impl Shape for Triangle {
    fn draw_type(&self) -> DrawType {
        self.draw_type
    }

    fn with_draw_type(&self, draw_type: DrawType) -> Self {
        Triangle::new(self.points[0], self.points[1], self.points[2], draw_type)
    }

    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self {
        let delta = delta.into();
        let mut points = vec![];
        for point in self.points {
            points.push(Coord {
                x: point.x + delta.x,
                y: point.y + delta.y,
            });
        }
        Triangle::new(points[0], points[1], points[2], self.draw_type)
    }

    fn move_to<P: Into<Coord>>(&self, xy: P) -> Self {
        let xy = xy.into();
        let diff1 = self.points[1] - self.points[0];
        let diff2 = self.points[2] - self.points[0];
        Triangle::new(xy, xy + diff1, xy + diff2, self.draw_type)
    }

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let p = point.into();
        let p1 = Coord::new(self.points[1].x - self.points[0].x, self.points[1].y - self.points[0].y);
        let p2 = Coord::new(self.points[2].x - self.points[0].x, self.points[2].y - self.points[0].y);
        let q = Coord::new(p.x - self.points[0].x, p.y - self.points[0].y);

        let s = q.cross_product(p2) as f32 / p1.cross_product(p2) as f32;
        let t = p1.cross_product(q) as f32 / p1.cross_product(p2) as f32;

        s >= 0.0 && t >= 0.0 && (s+t) <= 1.0
    }

    fn points(&self) -> Vec<Coord> {
        self.points.to_vec()
    }
}

impl Renderable for Triangle {
    fn render(&self, graphics: &mut Graphics) {
        let color = self.draw_type.color();
        graphics.draw_line(self.points[0], self.points[1], color);
        graphics.draw_line(self.points[1], self.points[2], color);
        graphics.draw_line(self.points[0], self.points[2], color);
        if let DrawType::Fill(_) = self.draw_type {
            let points = [
                Point2 { x: self.points[0].x as f32, y: self.points[0].y as f32 },
                Point2 { x: self.points[1].x as f32, y: self.points[1].y as f32 },
                Point2 { x: self.points[2].x as f32, y: self.points[2].y as f32 },
            ];
            if self.points[1].y == self.points[2].y {
                draw_flat_bottom(graphics, points, color);
            } else if self.points[0].y == self.points[1].y {
                draw_flat_top(graphics, points, color);
            } else {
                let p = Point2 {
                    x: points[0].x + ((points[1].y - points[0].y) / (points[2].y - points[0].y)) * (points[2].x - points[0].x),
                    y: points[1].y,
                };
                draw_flat_bottom(graphics, [points[0], points[1], p], color);
                draw_flat_top(graphics, [points[1], p, points[2]], color);
            }
        }
    }
}

fn draw_flat_bottom(graphics: &mut Graphics, points: [Point2<f32>; 3], color: Color) {
    let slope1 = (points[1].x - points[0].x) / (points[1].y - points[0].y);
    let slope2 = (points[2].x - points[0].x) / (points[2].y - points[0].y);
    let mut x1 = points[0].x;
    let mut x2 = points[0].x;
    for y in (points[0].y as usize)..(points[1].y as usize) {
        graphics.draw_line((x1 as usize, y), (x2 as usize + 1, y), color);
        x1 += slope1;
        x2 += slope2;
    }
}

fn draw_flat_top(graphics: &mut Graphics, points: [Point2<f32>; 3], color: Color) {
    let slope1 = (points[2].x - points[0].x) / (points[2].y - points[0].y);
    let slope2 = (points[2].x - points[1].x) / (points[2].y - points[1].y);
    let mut x1 = points[2].x;
    let mut x2 = points[2].x;
    for y in ((points[0].y as usize)..(points[2].y as usize)).rev() {
        graphics.draw_line((x1 as usize, y), (x2 as usize + 1, y), color);
        x1 -= slope1;
        x2 -= slope2;
    }
}