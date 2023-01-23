pub mod shape_box;
pub mod mutation;

use std::fmt::Debug;
use std::slice::Iter;
use crate::drawing::Renderable;
use crate::Graphics;
use crate::shapes::collection::shape_box::ShapeBox;


#[derive(Debug, Clone)]
pub struct ShapeCollection {
    shapes: Vec<ShapeBox>
}

impl ShapeCollection {
    pub fn new() -> Self {
        Self { shapes: vec![] }
    }
}

impl ShapeCollection {
    pub fn iter(&self) -> Iter<'_, ShapeBox> {
        self.shapes.iter()
    }

    pub fn len(&self) -> usize {
        self.shapes.len()
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