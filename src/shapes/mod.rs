use crate::color::Color;
use crate::coord::Coord;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

pub mod line;
pub mod rect;
pub mod circle;
pub mod convert_shape;
pub mod math;
//add arc, triangle, polygon, segment

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DrawType {
    Stroke(Color),
    Fill(Color),
}

pub fn fill(color: Color) -> DrawType {
    DrawType::Fill(color)
}

pub fn stroke(color: Color) -> DrawType {
    DrawType::Stroke(color)
}

pub trait Shape {
    fn draw_type(&self) -> DrawType;
    fn with_draw_type(&self, draw_type: DrawType) -> Self;
    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self;
    fn move_to<P: Into<Coord>>(&self, xy: P) -> Self;
    // fn scale_by(&self, factor: f32) -> Self;
    // fn topleft_point(&self) -> Coord;
    // fn bottomright_point(&self) -> Coord;
    fn points(&self) -> Vec<Coord>;
    // fn is_totally_off_screen(&self, graphics: &mut Graphics) -> Coord;
    // fn is_partially_off_screen(&self, graphics: &mut Graphics) -> Coord;
}