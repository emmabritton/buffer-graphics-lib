use crate::shapes::CreateDrawable;
use graphics_shapes::coord::Coord;
use graphics_shapes::{IntersectsContains, Shape};
use ici_files::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DrawType {
    Stroke(Color),
    Fill(Color),
}

impl DrawType {
    pub fn is_stroke(&self) -> bool {
        matches!(self, DrawType::Stroke(_))
    }

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

/// Represents a shape that is made of points that can be drawn either as a outline or solid
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Clone)]
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
    pub fn with_draw_type(&self, draw_type: DrawType) -> Drawable<T> {
        Drawable::new(self.obj.clone(), draw_type, self.drawing_points.clone())
    }
}

impl<T> Drawable<T>
where
    Self: CreateDrawable<T>,
    T: Shape + Clone,
{
    pub fn with_translation<P: Into<Coord>>(&self, delta: P) -> Drawable<T> {
        let moved = self.obj.translate_by(delta.into());
        Drawable::from_obj(moved, self.draw_type)
    }

    pub fn with_move<P: Into<Coord>>(&self, xy: P) -> Drawable<T> {
        let moved = self.obj.move_to(xy.into());
        Drawable::from_obj(moved, self.draw_type)
    }

    pub fn with_move_center<P: Into<Coord>>(&self, xy: P) -> Drawable<T> {
        let moved = self.obj.move_center_to(xy.into());
        Drawable::from_obj(moved, self.draw_type)
    }

    pub fn with_scale(&self, scale: f32) -> Drawable<T> {
        let moved = self.obj.scale(scale);
        Drawable::from_obj(moved, self.draw_type)
    }

    pub fn with_scale_around<P: Into<Coord>>(&self, scale: f32, point: P) -> Drawable<T> {
        let moved = self.obj.scale_around(scale, point.into());
        Drawable::from_obj(moved, self.draw_type)
    }

    pub fn with_rotation(&self, degrees: isize) -> Drawable<T> {
        let rotated = self.obj.rotate(degrees);
        Drawable::from_obj(rotated, self.draw_type)
    }

    pub fn with_rotation_around<P: Into<Coord>>(&self, degrees: isize, point: P) -> Drawable<T> {
        let rotated = self.obj.rotate_around(degrees, point.into());
        Drawable::from_obj(rotated, self.draw_type)
    }

    pub fn left(&self) -> isize {
        self.obj.left()
    }

    pub fn right(&self) -> isize {
        self.obj.right()
    }

    pub fn top(&self) -> isize {
        self.obj.top()
    }

    pub fn bottom(&self) -> isize {
        self.obj.bottom()
    }
}

impl<T> Drawable<T>
where
    Self: CreateDrawable<T>,
    T: IntersectsContains + Clone,
{
    pub fn intersects_shape(&self, shape: &dyn Shape) -> Option<bool> {
        self.obj.intersects_shape(shape)
    }

    pub fn contains_shape(&self, shape: &dyn Shape) -> Option<bool> {
        self.obj.contains_shape(shape)
    }

    pub fn intersects_drawable<S: Shape + Clone>(&self, drawable: &Drawable<S>) -> Option<bool> {
        self.obj.intersects_shape(&drawable.obj)
    }

    pub fn contains_drawable<S: Shape + Clone>(&self, drawable: &Drawable<S>) -> Option<bool> {
        self.obj.contains_shape(&drawable.obj)
    }
}
