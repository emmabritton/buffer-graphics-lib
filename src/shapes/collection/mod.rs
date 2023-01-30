pub mod mutation;
pub mod shape_box;

use crate::drawing::Renderable;
use crate::prelude::collection::shape_box::FromDrawable;
use crate::prelude::*;
use crate::shapes::collection::shape_box::{FromShape, ShapeBox};
use crate::Graphics;
use graphics_shapes::coord::Coord;
use graphics_shapes::rect::Rect;
use graphics_shapes::Shape;
use std::fmt::Debug;
use std::slice::Iter;

pub mod prelude {
    pub use crate::shapes::collection::shape_box::*;
    pub use crate::shapes::collection::*;
}

#[derive(Debug, Clone)]
pub struct ShapeCollection {
    shapes: Vec<ShapeBox>,
    bounds: Rect,
}

pub fn calc_bounds(list: &[ShapeBox]) -> Rect {
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
    pub fn new() -> Self {
        ShapeCollection {
            shapes: vec![],
            bounds: Rect::new((0, 0), (0, 0)),
        }
    }
}

impl ShapeCollection {
    pub fn iter(&self) -> Iter<'_, ShapeBox> {
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

    pub fn remove(&mut self, idx: usize) -> ShapeBox {
        self.shapes.remove(idx)
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

impl<S: Clone> InsertDrawable<S> for ShapeCollection
where
    ShapeBox: FromDrawable<S>,
{
    fn insert(&mut self, index: usize, drawable: Drawable<S>) {
        InsertShapeBox::insert(self, index, ShapeBox::from_drawable(drawable));
    }

    fn insert_above(&mut self, drawable: Drawable<S>) {
        InsertShapeBox::insert_above(self, ShapeBox::from_drawable(drawable));
    }

    fn insert_under(&mut self, drawable: Drawable<S>) {
        InsertShapeBox::insert_under(self, ShapeBox::from_drawable(drawable));
    }
}

impl<S> InsertShape<S> for ShapeCollection
where
    ShapeBox: FromShape<S>,
{
    fn insert(&mut self, index: usize, shape: S, draw_type: DrawType) {
        InsertShapeBox::insert(self, index, ShapeBox::from_shape(shape, draw_type));
    }

    fn insert_above(&mut self, shape: S, draw_type: DrawType) {
        InsertShapeBox::insert_above(self, ShapeBox::from_shape(shape, draw_type));
    }

    fn insert_under(&mut self, shape: S, draw_type: DrawType) {
        InsertShapeBox::insert_under(self, ShapeBox::from_shape(shape, draw_type));
    }
}

impl InsertShapeBox for ShapeCollection {
    fn insert(&mut self, index: usize, shape_box: ShapeBox) {
        self.shapes.insert(index, shape_box);
        self.update_bounds();
    }

    fn insert_above(&mut self, shape_box: ShapeBox) {
        InsertShapeBox::insert(self, 0, shape_box);
        self.update_bounds();
    }

    fn insert_under(&mut self, shape_box: ShapeBox) {
        self.shapes.push(shape_box);
        self.update_bounds();
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
    fn insert(&mut self, index: usize, shape_box: ShapeBox);
    fn insert_above(&mut self, shape_box: ShapeBox);
    fn insert_under(&mut self, shape_box: ShapeBox);
}
