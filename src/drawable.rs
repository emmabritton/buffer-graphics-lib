use crate::color::Color;
use graphics_shapes::coord::Coord;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DrawType {
    Stroke(Color),
    Fill(Color),
}

impl DrawType {
    #[inline]
    pub fn color(&self) -> Color {
        *match self {
            DrawType::Stroke(c) => c,
            DrawType::Fill(c) => c,
        }
    }
}

#[inline]
pub fn fill(color: Color) -> DrawType {
    DrawType::Fill(color)
}

#[inline]
pub fn stroke(color: Color) -> DrawType {
    DrawType::Stroke(color)
}

#[derive(Clone, Debug)]
pub struct Drawable<T: Clone> {
    obj: T,
    draw_type: DrawType,
    drawing_points: Vec<Coord>,
}

impl<T: Clone> Drawable<T> {
    #[inline]
    pub fn obj(&self) -> &T {
        &self.obj
    }
    #[inline]
    pub fn draw_type(&self) -> DrawType {
        self.draw_type
    }
    #[inline]
    pub fn drawing_points(&self) -> &Vec<Coord> {
        &self.drawing_points
    }
}

impl<T: Clone> Drawable<T> {
    #[inline]
    pub fn new(obj: T, draw_type: DrawType, drawing_points: Vec<Coord>) -> Drawable<T> {
        Self {
            obj,
            draw_type,
            drawing_points,
        }
    }
}

impl<T: Clone> Drawable<T> {
    #[inline]
    pub fn with_draw_type(&self, draw_type: DrawType) -> Drawable<T> {
        Drawable::new(self.obj.clone(), draw_type, self.drawing_points.clone())
    }
}
