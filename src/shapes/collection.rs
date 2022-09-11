use crate::drawable::Drawable;
use crate::Graphics;
use graphics_shapes::circle::Circle;
use graphics_shapes::line::Line;
use graphics_shapes::polygon::Polygon;
use graphics_shapes::rect::Rect;
use graphics_shapes::triangle::Triangle;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct AutoShapeCollection {
    rects: HashMap<usize, Drawable<Rect>>,
    lines: HashMap<usize, Drawable<Line>>,
    circles: HashMap<usize, Drawable<Circle>>,
    triangles: HashMap<usize, Drawable<Triangle>>,
    polygons: HashMap<usize, Drawable<Polygon>>,
}

impl AutoShapeCollection {
    pub fn new() -> Self {
        AutoShapeCollection::default()
    }
}

impl AutoShapeCollection {
    pub fn rects(&self) -> &HashMap<usize, Drawable<Rect>> {
        &self.rects
    }
    pub fn lines(&self) -> &HashMap<usize, Drawable<Line>> {
        &self.lines
    }
    pub fn circles(&self) -> &HashMap<usize, Drawable<Circle>> {
        &self.circles
    }
    pub fn triangles(&self) -> &HashMap<usize, Drawable<Triangle>> {
        &self.triangles
    }
    pub fn polygons(&self) -> &HashMap<usize, Drawable<Polygon>> {
        &self.polygons
    }

    pub fn render(&self, graphics: &mut Graphics) {
        for rect in self.rects.values() {
            graphics.draw(rect);
        }
        for circle in self.circles.values() {
            graphics.draw(circle);
        }
        for triangle in self.triangles.values() {
            graphics.draw(triangle);
        }
        for polygon in self.polygons.values() {
            graphics.draw(polygon);
        }
        for line in self.lines.values() {
            graphics.draw(line);
        }
    }

    pub fn remove_by_id(&mut self, key: usize) {
        self.rects.remove(&key);
        self.triangles.remove(&key);
        self.lines.remove(&key);
        self.polygons.remove(&key);
        self.circles.remove(&key);
    }
}

#[derive(Debug, Default)]
pub struct ShapeCollection<K: Eq + PartialEq + Hash + Clone> {
    rects: HashMap<K, Drawable<Rect>>,
    lines: HashMap<K, Drawable<Line>>,
    circles: HashMap<K, Drawable<Circle>>,
    triangles: HashMap<K, Drawable<Triangle>>,
    polygons: HashMap<K, Drawable<Polygon>>,
}

impl<K: Eq + PartialEq + Hash + Clone + Default> ShapeCollection<K> {
    pub fn new() -> Self {
        ShapeCollection::default()
    }
}

impl<K: Eq + PartialEq + Hash + Clone> ShapeCollection<K> {
    pub fn rects(&self) -> &HashMap<K, Drawable<Rect>> {
        &self.rects
    }
    pub fn lines(&self) -> &HashMap<K, Drawable<Line>> {
        &self.lines
    }
    pub fn circles(&self) -> &HashMap<K, Drawable<Circle>> {
        &self.circles
    }
    pub fn triangles(&self) -> &HashMap<K, Drawable<Triangle>> {
        &self.triangles
    }
    pub fn polygons(&self) -> &HashMap<K, Drawable<Polygon>> {
        &self.polygons
    }

    pub fn render(&self, graphics: &mut Graphics) {
        for rect in self.rects.values() {
            graphics.draw(rect);
        }
        for circle in self.circles.values() {
            graphics.draw(circle);
        }
        for triangle in self.triangles.values() {
            graphics.draw(triangle);
        }
        for polygon in self.polygons.values() {
            graphics.draw(polygon);
        }
        for line in self.lines.values() {
            graphics.draw(line);
        }
    }

    pub fn remove_by_id(&mut self, key: &K) {
        self.rects.remove(key);
        self.triangles.remove(key);
        self.lines.remove(key);
        self.polygons.remove(key);
        self.circles.remove(key);
    }
}

trait AddShape<K: Eq + PartialEq + Hash + Clone, T: Clone> {
    fn add(&mut self, key: K, shape: Drawable<T>);
    fn remove_shape(&mut self, shape: &T);
    fn remove(&mut self, shape: &Drawable<T>);
}

trait AutoAddShape<K: Eq + PartialEq + Hash + Clone, T: Clone> {
    fn add(&mut self, shape: Drawable<T>) -> K;
    fn remove_shape(&mut self, shape: &T);
    fn remove(&mut self, shape: &Drawable<T>);
}

macro_rules! impl_add_shape {
    ($shape_type: ty, $var: ident) => {
        impl<K: Eq + PartialEq + Hash + Clone> AddShape<K, $shape_type> for ShapeCollection<K> {
            fn add(&mut self, key: K, shape: Drawable<$shape_type>) {
                self.$var.insert(key.clone(), shape);
            }

            fn remove(&mut self, shape: &Drawable<$shape_type>) {
                let mut keys = vec![];
                for (key, item) in &self.$var {
                    if shape == item {
                        keys.push(key.clone());
                    }
                }
                for key in keys {
                    self.$var.remove(&key);
                }
            }

            fn remove_shape(&mut self, shape: &$shape_type) {
                let mut keys = vec![];
                for (key, item) in self.$var.iter_mut() {
                    if shape == item.obj() {
                        keys.push(key.clone());
                    }
                }
                for key in keys {
                    self.$var.remove(&key);
                }
            }
        }
    };
}

macro_rules! impl_add_auto_shape {
    ($shape_type: ty, $var: ident) => {
        impl AutoAddShape<usize, $shape_type> for AutoShapeCollection {
            fn add(&mut self, shape: Drawable<$shape_type>) -> usize {
                let key = self.$var.iter().last().map(|(k, _)| k).unwrap_or(&0) + 1;
                self.$var.insert(key, shape);
                key
            }

            fn remove(&mut self, shape: &Drawable<$shape_type>) {
                let mut keys = vec![];
                for (key, item) in &self.$var {
                    if shape == item {
                        keys.push(key.clone());
                    }
                }
                for key in keys {
                    self.$var.remove(&key);
                }
            }

            fn remove_shape(&mut self, shape: &$shape_type) {
                let mut keys = vec![];
                for (key, item) in self.$var.iter_mut() {
                    if shape == item.obj() {
                        keys.push(key.clone());
                    }
                }
                for key in keys {
                    self.$var.remove(&key);
                }
            }
        }
    };
}

impl_add_shape!(Line, lines);
impl_add_shape!(Circle, circles);
impl_add_shape!(Triangle, triangles);
impl_add_shape!(Polygon, polygons);
impl_add_shape!(Rect, rects);

impl_add_auto_shape!(Line, lines);
impl_add_auto_shape!(Circle, circles);
impl_add_auto_shape!(Triangle, triangles);
impl_add_auto_shape!(Polygon, polygons);
impl_add_auto_shape!(Rect, rects);

#[cfg(test)]
mod test {
    use crate::color::BLUE;
    use crate::drawable::{stroke, Drawable};
    use crate::shapes::collection::{AddShape, AutoAddShape, AutoShapeCollection, ShapeCollection};
    use crate::shapes::CreateDrawable;
    use graphics_shapes::line::Line;

    #[test]
    fn shape_collection() {
        let mut collection: ShapeCollection<String> = ShapeCollection::new();
        let line = Line::new((10, 10), (20, 20));
        let drawable = Drawable::from_obj(line.clone(), stroke(BLUE));
        collection.add(
            String::from("test"),
            Drawable::from_obj(line.clone(), stroke(BLUE)),
        );
        assert_eq!(collection.lines()["test"], drawable);
        collection.remove_by_id(&"test".to_string());
        assert!(collection.lines().is_empty());
        collection.add(
            String::from("test2"),
            Drawable::from_obj(line.clone(), stroke(BLUE)),
        );
        assert_eq!(collection.lines()["test2"], drawable);
        collection.remove_shape(&line);
        assert!(collection.lines().is_empty());
    }

    #[test]
    fn auto_shape_collection() {
        let mut collection = AutoShapeCollection::new();
        let line = Line::new((10, 10), (20, 20));
        let drawable = Drawable::from_obj(line.clone(), stroke(BLUE));
        let key = collection.add(Drawable::from_obj(line.clone(), stroke(BLUE)));
        assert_eq!(collection.lines()[&key], drawable);
        collection.remove_by_id(key);
        assert!(collection.lines().is_empty());
        let key = collection.add(Drawable::from_obj(line.clone(), stroke(BLUE)));
        assert_eq!(collection.lines()[&key], drawable);
        collection.remove_shape(&line);
        assert!(collection.lines().is_empty());
    }
}
