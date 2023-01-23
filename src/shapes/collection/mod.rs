pub mod mutation;
pub mod shape_box;

use crate::drawing::Renderable;
use crate::shapes::collection::shape_box::ShapeBox;
use crate::Graphics;
use std::fmt::Debug;
use std::slice::Iter;

#[derive(Debug, Clone, Default)]
pub struct ShapeCollection {
    shapes: Vec<ShapeBox>,
}

impl ShapeCollection {
    pub fn new() -> Self {
        Self::default()
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

    pub fn remove(&mut self, idx: usize) -> ShapeBox {
        self.shapes.remove(idx)
    }

    pub fn insert(&mut self, idx: usize, element: ShapeBox) {
        self.shapes.insert(idx, element);
    }

    pub fn insert_under(&mut self, element: ShapeBox) {
        self.insert(0, element)
    }

    pub fn insert_above(&mut self, element: ShapeBox) {
        self.shapes.push(element)
    }
}

impl Renderable<ShapeCollection> for ShapeCollection {
    fn render(&self, graphics: &mut Graphics) {
        for shape in &self.shapes {
            shape.render(graphics);
        }
    }
}
