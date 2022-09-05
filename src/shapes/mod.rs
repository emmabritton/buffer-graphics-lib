use crate::color::Color;
use crate::coord::Coord;
use crate::drawing::Renderable;
use crate::shapes::circle::*;
use crate::shapes::line::*;
use crate::shapes::rect::*;
use crate::shapes::triangle::*;
use crate::Graphics;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

mod circle;
mod line;
mod rect;
mod triangle;
//add arc, polygon, segment

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Shape {
    Line {
        start: Coord,
        end: Coord,
        color: Color,
    },
    Rect {
        topleft: Coord,
        bottomright: Coord,
        draw_type: DrawType,
    },
    Circle {
        center: Coord,
        radius: usize,
        draw_type: DrawType,
    },
    Triangle {
        points: [Coord; 3],
        draw_type: DrawType,
    },
}

impl Shape {
    pub fn line<P1: Into<Coord>, P2: Into<Coord>>(start: P1, end: P2, color: Color) -> Shape {
        let start = start.into();
        let end = end.into();
        let points = if start.y < end.y {
            (start, end)
        } else {
            (end, start)
        };
        Shape::Line {
            start: points.0,
            end: points.1,
            color,
        }
    }

    pub fn rect<P1: Into<Coord>, P2: Into<Coord>>(
        topleft: P1,
        bottomright: P2,
        draw_type: DrawType,
    ) -> Shape {
        let topleft = topleft.into();
        let bottomright = bottomright.into();
        let x1 = topleft.x.min(bottomright.x);
        let y1 = topleft.y.min(bottomright.y);
        let x2 = topleft.x.max(bottomright.x);
        let y2 = topleft.y.max(bottomright.y);
        Shape::Rect {
            topleft: Coord::new(x1, y1),
            bottomright: Coord::new(x2, y2),
            draw_type,
        }
    }

    pub fn circle<P: Into<Coord>>(center: P, radius: usize, draw_type: DrawType) -> Shape {
        Shape::Circle {
            center: center.into(),
            radius,
            draw_type,
        }
    }

    pub fn triangle<P1: Into<Coord>, P2: Into<Coord>, P3: Into<Coord>>(
        point1: P1,
        point2: P2,
        point3: P3,
        draw_type: DrawType,
    ) -> Shape {
        let mut sorted_points: Vec<Coord> = vec![point1.into(), point2.into(), point3.into()];
        sorted_points.sort_by_key(|c| c.y);
        Shape::Triangle {
            points: [sorted_points[0], sorted_points[1], sorted_points[2]],
            draw_type,
        }
    }
}

impl Shape {
    pub fn draw_type(&self) -> DrawType {
        match self {
            Shape::Line { color, .. } => stroke(*color),
            Shape::Rect { draw_type, .. } => *draw_type,
            Shape::Circle { draw_type, .. } => *draw_type,
            Shape::Triangle { draw_type, .. } => *draw_type,
        }
    }

    pub fn with_draw_type(&self, draw_type: DrawType) -> Self {
        match self {
            Shape::Line { start, end, .. } => Shape::line(*start, *end, draw_type.color()),
            Shape::Rect {
                topleft,
                bottomright,
                ..
            } => Shape::rect(*topleft, *bottomright, draw_type),
            Shape::Circle { center, radius, .. } => Shape::circle(*center, *radius, draw_type),
            Shape::Triangle { points, .. } => {
                Shape::triangle(points[0], points[1], points[2], draw_type)
            }
        }
    }

    pub fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self {
        let delta = delta.into();
        match self {
            Shape::Line { start, end, color } => Shape::line(*start + delta, *end + delta, *color),
            Shape::Rect {
                topleft,
                bottomright,
                draw_type,
            } => Shape::rect(*topleft + delta, *bottomright + delta, *draw_type),
            Shape::Circle {
                center,
                radius,
                draw_type,
            } => Shape::circle(*center + delta, *radius, *draw_type),
            Shape::Triangle { points, draw_type } => {
                let points: Vec<Coord> = points.iter().map(|p| *p + delta).collect();
                Shape::triangle(points[0], points[1], points[2], *draw_type)
            }
        }
    }

    pub fn move_to<P: Into<Coord>>(&self, point: P) -> Self {
        let point = point.into();
        match self {
            Shape::Line { start, end, color } => {
                let diff = (*start).diff(*end);
                Shape::line(point, point + diff, *color)
            }
            Shape::Rect {
                topleft,
                bottomright,
                draw_type,
            } => {
                let diff = (*topleft).diff(*bottomright);
                Shape::rect(point, point + diff, *draw_type)
            }
            Shape::Circle {
                draw_type, radius, ..
            } => Shape::circle(point, *radius, *draw_type),
            Shape::Triangle { points, draw_type } => {
                let diff1 = points[1] - points[0];
                let diff2 = points[2] - points[0];
                Shape::triangle(point, point + diff1, point + diff2, *draw_type)
            }
        }
    }

    pub fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        match self {
            Shape::Line { start, end, .. } => line_contains(*start, *end, point.into()),
            Shape::Rect {
                topleft,
                bottomright,
                ..
            } => rect_contains(*topleft, *bottomright, point.into()),
            Shape::Circle { center, radius, .. } => circle_contains(*center, *radius, point.into()),
            Shape::Triangle { points, .. } => triangle_contains(*points, point.into()),
        }
    }

    pub fn points(&self) -> Vec<Coord> {
        match self {
            Shape::Line { start, end, .. } => vec![*start, *end],
            Shape::Rect {
                topleft,
                bottomright,
                ..
            } => vec![*topleft, *bottomright],
            Shape::Circle { center, .. } => vec![*center],
            Shape::Triangle { points, .. } => points.to_vec(),
        }
    }

    // fn scale_by(&self, factor: f32) -> Self;
    // fn topleft_point(&self) -> Coord;
    // fn bottomright_point(&self) -> Coord;
    // fn is_totally_off_screen(&self, graphics: &mut Graphics) -> Coord;
    // fn is_partially_off_screen(&self, graphics: &mut Graphics) -> Coord;
}

impl Renderable for Shape {
    fn render(&self, graphics: &mut Graphics) {
        match self {
            Shape::Line { start, end, color } => graphics.draw_line(*start, *end, *color),
            Shape::Rect {
                topleft,
                bottomright,
                draw_type,
            } => rect_draw(graphics, *topleft, *bottomright, *draw_type),
            Shape::Circle {
                center,
                radius,
                draw_type,
            } => circle_draw(graphics, *center, *radius, *draw_type),
            Shape::Triangle { points, draw_type } => triangle_draw(graphics, *points, *draw_type),
        }
    }
}

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DrawType {
    Stroke(Color),
    Fill(Color),
}

impl DrawType {
    fn color(&self) -> Color {
        *match self {
            DrawType::Stroke(c) => c,
            DrawType::Fill(c) => c,
        }
    }
}

pub fn fill(color: Color) -> DrawType {
    DrawType::Fill(color)
}

pub fn stroke(color: Color) -> DrawType {
    DrawType::Stroke(color)
}
