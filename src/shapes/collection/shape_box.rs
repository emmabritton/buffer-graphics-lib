use crate::drawable::{DrawType, Drawable};
use crate::drawing::Renderable;
use crate::shapes::CreateDrawable;
use crate::Graphics;
use graphics_shapes::circle::Circle;
use graphics_shapes::coord::Coord;
use graphics_shapes::ellipse::Ellipse;
use graphics_shapes::line::Line;
use graphics_shapes::polygon::Polygon;
use graphics_shapes::rect::Rect;
use graphics_shapes::triangle::Triangle;
use graphics_shapes::Shape;

/// Represents one of the shapes from graphics-shapes so they be used in a collection with
/// generic or type issues
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeBox {
    Line(Drawable<Line>),
    Rect(Drawable<Rect>),
    Triangle(Drawable<Triangle>),
    Circle(Drawable<Circle>),
    Ellipse(Drawable<Ellipse>),
    Polygon(Drawable<Polygon>),
}

impl ShapeBox {
    pub(crate) fn with_draw_type(&self, draw_type: DrawType) -> ShapeBox {
        match self {
            ShapeBox::Line(d) => ShapeBox::from_drawable(d.with_draw_type(draw_type)),
            ShapeBox::Rect(d) => ShapeBox::from_drawable(d.with_draw_type(draw_type)),
            ShapeBox::Triangle(d) => ShapeBox::from_drawable(d.with_draw_type(draw_type)),
            ShapeBox::Circle(d) => ShapeBox::from_drawable(d.with_draw_type(draw_type)),
            ShapeBox::Ellipse(d) => ShapeBox::from_drawable(d.with_draw_type(draw_type)),
            ShapeBox::Polygon(d) => ShapeBox::from_drawable(d.with_draw_type(draw_type)),
        }
    }
}

pub trait FromDrawable<S: Clone> {
    fn from_drawable(drawable: Drawable<S>) -> ShapeBox;
}

pub trait FromShape<S> {
    fn from_shape(shape: S, draw_type: DrawType) -> ShapeBox;
}

impl<S: Clone> From<Drawable<S>> for ShapeBox
where
    ShapeBox: FromDrawable<S>,
{
    fn from(value: Drawable<S>) -> Self {
        ShapeBox::from_drawable(value)
    }
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

impl ShapeBox {
    fn bounds(&self) -> Rect {
        match self {
            ShapeBox::Line(d) => d.obj().as_rect(),
            ShapeBox::Rect(d) => d.obj().clone(),
            ShapeBox::Triangle(d) => d.obj().as_rect(),
            ShapeBox::Circle(d) => d.obj().as_rect(),
            ShapeBox::Ellipse(d) => d.obj().as_rect(),
            ShapeBox::Polygon(d) => d.obj().as_rect(),
        }
    }

    pub fn left(&self) -> isize {
        self.bounds().left()
    }

    pub fn top(&self) -> isize {
        self.bounds().top()
    }

    pub fn bottom(&self) -> isize {
        self.bounds().bottom()
    }

    pub fn right(&self) -> isize {
        self.bounds().right()
    }

    pub fn center(&self) -> Coord {
        self.bounds().center()
    }
}
