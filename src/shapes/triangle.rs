use crate::Graphics;
use crate::shapes::{DrawType, Shape};

#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Triangle {
    points: [Coord; 3],
    leftmost_point: usize,
    topmost_point: usize,
    rightmost_point: usize,
    bottommost_point: usize,
}

impl Triangle {
    pub fn new(point1_x: isize, point1_y: isize, point2_x: isize, point2_y: isize, point3_x: isize, point3_y: isize) -> Self {
        let leftmost_point = [point1_x, point2_x, point3_x].iter().enumerate().min().unwrap().0;
        let rightmost_point = [point1_x, point2_x, point3_x].iter().enumerate().max().unwrap().0;
        let topmost_point = [point1_y, point2_y, point3_y].iter().enumerate().min().unwrap().0;
        let bottommost_point = [point1_y, point2_y, point3_y].iter().enumerate().max().unwrap().0;
        let points = [
            Point2 { x: point1_x, y: point1_y },
            Point2 { x: point2_x, y: point2_y },
            Point2 { x: point3_x, y: point3_y },
        ];

        Self {
            points,
            leftmost_point,
            topmost_point,
            rightmost_point,
            bottommost_point,
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
    fn render(&self, graphics: &mut Graphics, draw_type: &DrawType) {

    }

    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self {
        let delta = delta.into();
        let points = vec![];
        for point in self.points {
            points.add(Point2 {
                x: point.x + delta.x,
                y: point.y + delta.y,
            });
        }
        Triangle::new(points.0.x, points.0.y, points.1.x, points.1.y, points.2.x, points.2.y)
    }

    fn move_to<P: Into<Coord>>(&self, xy: P) -> Self {
        let xy = xy.into();
        let diff1_x = self.points[1].x - self.points[0].x;
        let diff1_y = self.points[1].y - self.points[0].y;
        let diff2_x = self.points[2].x - self.points[0].x;
        let diff2_y = self.points[2].y - self.points[0].y;
        Triangle::new(xy.x, xy.y, xy.x + diff1_x, xy.y + diff1_y, xy.x + diff2_x, xy.x + diff2_y)
    }

    fn points(&self) -> Vec<Coord> {
        self.points.to_vec()
    }
}