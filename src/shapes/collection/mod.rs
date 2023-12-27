use crate::drawing::Renderable;
use crate::prelude::*;
use crate::Graphics;
use graphics_shapes::coord::Coord;
use graphics_shapes::rect::Rect;
use graphics_shapes::shape_box::ShapeBox;
use graphics_shapes::Shape;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::slice::Iter;

pub mod mutation;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeCollection {
    shapes: Vec<Drawable<ShapeBox>>,
    bounds: Rect,
}

pub fn calc_bounds(list: &[Drawable<ShapeBox>]) -> Rect {
    if list.is_empty() {
        return Rect::new((0, 0), (0, 0));
    }
    let mut bounds = Rect::new(
        (list[0].left(), list[0].top()),
        (list[0].right(), list[0].bottom()),
    );
    for shape in list.iter().skip(1) {
        bounds = Rect::new(
            (
                bounds.left().min(shape.left()),
                bounds.top().min(shape.top()),
            ),
            (
                bounds.right().max(shape.right()),
                bounds.bottom().max(shape.bottom()),
            ),
        );
    }
    bounds
}

impl ShapeCollection {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ShapeCollection {
            shapes: vec![],
            bounds: Rect::new((0, 0), (0, 0)),
        }
    }
}

impl ShapeCollection {
    pub fn iter(&self) -> Iter<'_, Drawable<ShapeBox>> {
        self.shapes.iter()
    }

    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    fn update_bounds(&mut self) {
        self.bounds = calc_bounds(&self.shapes);
    }

    pub fn remove(&mut self, idx: usize) -> Drawable<ShapeBox> {
        self.shapes.remove(idx)
    }

    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }

    pub fn left(&self) -> isize {
        self.bounds.left()
    }

    pub fn top(&self) -> isize {
        self.bounds.top()
    }

    pub fn bottom(&self) -> isize {
        self.bounds.bottom()
    }

    pub fn right(&self) -> isize {
        self.bounds.right()
    }

    pub fn center(&self) -> Coord {
        self.bounds.center()
    }
}

impl Renderable<ShapeCollection> for ShapeCollection {
    fn render(&self, graphics: &mut Graphics) {
        for shape in &self.shapes {
            shape.render(graphics);
        }
    }
}

impl InsertDrawable<ShapeBox> for ShapeCollection {
    fn insert(&mut self, index: usize, drawable: Drawable<ShapeBox>) {
        self.shapes.insert(index, drawable);
        self.update_bounds();
    }

    fn insert_above(&mut self, drawable: Drawable<ShapeBox>) {
        self.shapes.push(drawable);
        self.update_bounds();
    }

    fn insert_under(&mut self, drawable: Drawable<ShapeBox>) {
        InsertDrawable::<ShapeBox>::insert(self, 0, drawable);
    }
}

impl<S: Shape> InsertShape<S> for ShapeCollection {
    fn insert(&mut self, index: usize, shape: S, draw_type: DrawType) {
        InsertShapeBox::insert(self, index, shape.to_shape_box(), draw_type);
    }

    fn insert_above(&mut self, shape: S, draw_type: DrawType) {
        InsertShapeBox::insert_above(self, shape.to_shape_box(), draw_type);
    }

    fn insert_under(&mut self, shape: S, draw_type: DrawType) {
        InsertShapeBox::insert_under(self, shape.to_shape_box(), draw_type);
    }
}

impl InsertShapeBox for ShapeCollection {
    fn insert(&mut self, index: usize, shape_box: ShapeBox, draw_type: DrawType) {
        InsertDrawable::insert(self, index, Drawable::from_obj(shape_box, draw_type))
    }

    fn insert_above(&mut self, shape_box: ShapeBox, draw_type: DrawType) {
        InsertDrawable::insert_above(self, Drawable::from_obj(shape_box, draw_type))
    }

    fn insert_under(&mut self, shape_box: ShapeBox, draw_type: DrawType) {
        InsertDrawable::insert_under(self, Drawable::from_obj(shape_box, draw_type))
    }
}

pub trait InsertDrawable<S: Clone>: InsertShapeBox {
    fn insert(&mut self, index: usize, drawable: Drawable<S>);
    fn insert_above(&mut self, drawable: Drawable<S>);
    fn insert_under(&mut self, drawable: Drawable<S>);
}

pub trait InsertShape<S> {
    fn insert(&mut self, index: usize, shape: S, draw_type: DrawType);
    fn insert_above(&mut self, shape: S, draw_type: DrawType);
    fn insert_under(&mut self, shape: S, draw_type: DrawType);
}

pub trait InsertShapeBox {
    fn insert(&mut self, index: usize, shape_box: ShapeBox, draw_type: DrawType);
    fn insert_above(&mut self, shape_box: ShapeBox, draw_type: DrawType);
    fn insert_under(&mut self, shape_box: ShapeBox, draw_type: DrawType);
}
