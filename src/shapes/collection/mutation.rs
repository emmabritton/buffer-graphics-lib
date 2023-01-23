use graphics_shapes::coord::Coord;
use crate::shapes::collection::shape_box::{FromDrawable, ShapeBox};
use crate::shapes::collection::ShapeCollection;

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

        impl ShapeCollection {
            pub fn $name<P: Into<Coord>>(&self, $param: P) -> ShapeCollection {
                let $param = $param.into();
                ShapeCollection { shapes: self.shapes.iter().map(|shape| shape.$name($param)).collect() }
            }
        }
    };
}

macro_rules! shapebox_mutate_one {
    ($name: ident, $param: ident, $param_type: ty) => {
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
            pub fn $name<P: Into<Coord>>(&self, $param: $param_type) -> ShapeCollection {
                ShapeCollection { shapes: self.shapes.iter().map(|shape| shape.$name($param)).collect() }
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
                ShapeCollection { shapes: self.shapes.iter().map(|shape| shape.$name($param, center)).collect() }
            }
        }
    };
}

shapebox_mutate_coord!(with_translation, delta);
shapebox_mutate_coord!(with_move, xy);
shapebox_mutate_one!(with_scale, scale, f32);
shapebox_mutate_one!(with_rotation, degrees, isize);
shapebox_mutate_two!(with_scale_around, scale, f32);
shapebox_mutate_two!(with_rotation_around, degrees, isize);
