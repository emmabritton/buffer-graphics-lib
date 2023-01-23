use graphics_shapes::circle::Circle;
use graphics_shapes::ellipse::Ellipse;
use graphics_shapes::line::Line;
use graphics_shapes::polygon::Polygon;
use graphics_shapes::rect::Rect;
use graphics_shapes::triangle::Triangle;
use crate::drawable::{Drawable, DrawType};
use crate::drawing::Renderable;
use crate::Graphics;
use crate::shapes::CreateDrawable;

/// Represents one of the shapes from graphics-shapes so they be used in a collection with
/// generic or type issues
#[derive(Debug, Clone)]
pub enum ShapeBox {
    Line(Drawable<Line>),
    Rect(Drawable<Rect>),
    Triangle(Drawable<Triangle>),
    Circle(Drawable<Circle>),
    Ellipse(Drawable<Ellipse>),
    Polygon(Drawable<Polygon>)
}

pub trait FromDrawable<S: Clone> {
    fn from_drawable(drawable: Drawable<S>) -> ShapeBox;
}

pub trait FromShape<S> {
    fn from_shape(shape: S, draw_type: DrawType) -> ShapeBox;
}

macro_rules! from_drawable {
    ($shape: ty, $shape_box: expr) => {
        impl FromDrawable<$shape> for ShapeBox {
            fn from_drawable(drawable: Drawable<$shape>) -> ShapeBox {
                $shape_box(drawable)
            }
        }

        impl FromShape<$shape> for ShapeBox {
            fn from_shape(shape: $shape, draw_type: DrawType) -> ShapeBox {
                $shape_box(Drawable::from_obj(shape, draw_type))
            }
        }
    };
}

from_drawable!(Line, ShapeBox::Line);
from_drawable!(Rect, ShapeBox::Rect);
from_drawable!(Circle, ShapeBox::Circle);
from_drawable!(Ellipse, ShapeBox::Ellipse);
from_drawable!(Triangle, ShapeBox::Triangle);
from_drawable!(Polygon, ShapeBox::Polygon);

impl Renderable<ShapeBox> for ShapeBox {
    fn render(&self, graphics: &mut Graphics) {
        match self {
            ShapeBox::Line(d) => graphics.draw(d),
            ShapeBox::Rect(d) => graphics.draw(d),
            ShapeBox::Triangle(d) => graphics.draw(d),
            ShapeBox::Circle(d) => graphics.draw(d),
            ShapeBox::Ellipse(d) => graphics.draw(d),
            ShapeBox::Polygon(d) => graphics.draw(d),
        }
    }
}
