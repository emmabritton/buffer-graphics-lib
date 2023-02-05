use crate::prelude::DrawType;
use crate::shapes::collection::shape_box::{FromDrawable, ShapeBox};
use crate::shapes::collection::{calc_bounds, ShapeCollection};
use graphics_shapes::coord::Coord;

impl ShapeCollection {
    pub fn with_draw_type(&self, draw_type: DrawType) -> ShapeCollection {
        let shapes: Vec<ShapeBox> = self
            .shapes
            .iter()
            .map(|shape| shape.with_draw_type(draw_type))
            .collect();
        ShapeCollection {
            shapes,
            bounds: self.bounds.clone(),
        }
    }
}

impl ShapeCollection {
    pub fn with_translation<P: Into<Coord>>(&self, delta: P) -> ShapeCollection {
        let delta = delta.into();
        let shapes: Vec<ShapeBox> = self
            .shapes
            .iter()
            .map(|shape| shape.with_translation(delta))
            .collect();
        let bounds = calc_bounds(&shapes);
        ShapeCollection { shapes, bounds }
    }

    pub fn with_move<P: Into<Coord>>(&self, xy: P) -> ShapeCollection {
        let xy = xy.into();
        let collection_start = Coord::new(self.left(), self.top());
        let diff = xy - collection_start;
        let shapes: Vec<ShapeBox> = self
            .shapes
            .iter()
            .map(|shape| shape.with_translation(diff))
            .collect();
        let bounds = calc_bounds(&shapes);
        ShapeCollection { shapes, bounds }
    }
}

macro_rules! shapebox_mutate_coord {
    ($name: ident, $param: ident) => {
        impl ShapeBox {
            pub fn $name<P: Into<Coord>>(&self, $param: P) -> ShapeBox {
                match self {
                    ShapeBox::Line(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Rect(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Triangle(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Circle(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Ellipse(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Polygon(s) => ShapeBox::from_drawable(s.$name($param)),
                }
            }
        }
    };
}

macro_rules! shapebox_mutate_one {
    ($name: ident, $drawable_name: ident, $param: ident, $param_type: ty) => {
        impl ShapeBox {
            pub fn $name(&self, $param: $param_type) -> ShapeBox {
                match self {
                    ShapeBox::Line(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Rect(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Triangle(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Circle(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Ellipse(s) => ShapeBox::from_drawable(s.$name($param)),
                    ShapeBox::Polygon(s) => ShapeBox::from_drawable(s.$name($param)),
                }
            }
        }

        impl ShapeCollection {
            pub fn $name(&self, $param: $param_type) -> ShapeCollection {
                let center = self.center();
                let shapes: Vec<ShapeBox> = self
                    .shapes
                    .iter()
                    .map(|shape| shape.$drawable_name($param, center))
                    .collect();
                let bounds = calc_bounds(&shapes);
                ShapeCollection { shapes, bounds }
            }
        }
    };
}

macro_rules! shapebox_mutate_two {
    ($name: ident, $param: ident, $param_type: ty) => {
        impl ShapeBox {
            pub fn $name<P: Into<Coord>>(&self, $param: $param_type, center: P) -> ShapeBox {
                match self {
                    ShapeBox::Line(s) => ShapeBox::from_drawable(s.$name($param, center)),
                    ShapeBox::Rect(s) => ShapeBox::from_drawable(s.$name($param, center)),
                    ShapeBox::Triangle(s) => ShapeBox::from_drawable(s.$name($param, center)),
                    ShapeBox::Circle(s) => ShapeBox::from_drawable(s.$name($param, center)),
                    ShapeBox::Ellipse(s) => ShapeBox::from_drawable(s.$name($param, center)),
                    ShapeBox::Polygon(s) => ShapeBox::from_drawable(s.$name($param, center)),
                }
            }
        }

        impl ShapeCollection {
            pub fn $name<P: Into<Coord>>(&self, $param: $param_type, center: P) -> ShapeCollection {
                let center = center.into();
                let shapes: Vec<ShapeBox> = self
                    .shapes
                    .iter()
                    .map(|shape| shape.$name($param, center))
                    .collect();
                let bounds = calc_bounds(&shapes);
                ShapeCollection { shapes, bounds }
            }
        }
    };
}

shapebox_mutate_coord!(with_translation, delta);
shapebox_mutate_coord!(with_move, xy);
shapebox_mutate_one!(with_scale, with_scale_around, scale, f32);
shapebox_mutate_one!(with_rotation, with_rotation_around, degrees, isize);
shapebox_mutate_two!(with_scale_around, scale, f32);
shapebox_mutate_two!(with_rotation_around, degrees, isize);

#[cfg(test)]
mod test {
    use crate::prelude::{fill, FromShape, ShapeBox, ShapeCollection, RED};
    use crate::shapes::collection::InsertShape;
    use graphics_shapes::prelude::*;

    #[test]
    fn check_with_translation() {
        let mut collection = ShapeCollection::new();
        collection.insert_above(Rect::new((20, 20), (40, 40)), fill(RED));
        collection.insert_above(Rect::new((60, 20), (80, 40)), fill(RED));

        let moved = collection.with_translation((0, 20));

        assert_eq!(
            moved.shapes,
            vec![
                ShapeBox::from_shape(Rect::new((20, 40), (40, 60)), fill(RED)),
                ShapeBox::from_shape(Rect::new((60, 40), (80, 60)), fill(RED)),
            ]
        );
    }

    #[test]
    fn check_with_move() {
        let mut collection = ShapeCollection::new();
        collection.insert_above(Rect::new((20, 20), (40, 40)), fill(RED));
        collection.insert_above(Rect::new((60, 20), (80, 40)), fill(RED));

        let moved = collection.with_move((0, 0));

        assert_eq!(
            moved.shapes,
            vec![
                ShapeBox::from_shape(Rect::new((0, 0), (20, 20)), fill(RED)),
                ShapeBox::from_shape(Rect::new((40, 0), (60, 20)), fill(RED)),
            ]
        );
    }
}
