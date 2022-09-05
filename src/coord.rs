use mint::Point2;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Neg, Sub};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    #[inline]
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn from_angle(center: Coord, distance: isize, degrees: f32) -> Self {
        let x = (distance as f32) * degrees.to_radians().cos() + (center.x as f32);
        let y = (distance as f32) * degrees.to_radians().cos() + (center.y as f32);
        Coord::new(x as isize, y as isize)
    }
}

impl Coord {
    pub fn diff(self, rhs: Self) -> Self {
        Coord::new((self.x - rhs.x).abs(), (self.y - rhs.y).abs())
    }

    pub fn distance(self, rhs: Self) -> usize {
        let x = (rhs.x - self.x) as f32;
        let y = (rhs.y - self.y) as f32;
        x.hypot(y).abs() as usize
    }

    pub fn mid_point(self, rhs: Self) -> Coord {
        let x = (self.x + rhs.x) / 2;
        let y = (self.y + rhs.y) / 2;
        Coord::new(x, y)
    }

    pub fn angle_to(self, rhs: Self) -> f32 {
        let x = (rhs.x - self.x) as f32;
        let y = (rhs.y - self.y) as f32;
        y.atan2(x).to_degrees()
    }

    pub fn cross_product(self, rhs: Self) -> isize {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn dot_product(self, rhs: Self) -> isize {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn perpendicular(self) -> Coord {
        Coord::new(self.y, -self.x)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Self::Output {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Point2<isize>> for Coord {
    fn from(point: Point2<isize>) -> Self {
        Coord {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<Coord> for Point2<isize> {
    fn from(coord: Coord) -> Self {
        Point2 {
            x: coord.x,
            y: coord.y,
        }
    }
}

macro_rules! impl_from_num {
    ($num_type:ty) => {
        impl From<($num_type, $num_type)> for Coord {
            fn from(nums: ($num_type, $num_type)) -> Coord {
                Coord {
                    x: nums.0 as isize,
                    y: nums.1 as isize,
                }
            }
        }
    };
}

impl_from_num!(u8);
impl_from_num!(i8);
impl_from_num!(u16);
impl_from_num!(i16);
impl_from_num!(u32);
impl_from_num!(i32);
impl_from_num!(u64);
impl_from_num!(i64);
impl_from_num!(u128);
impl_from_num!(i128);
impl_from_num!(usize);
impl_from_num!(isize);
impl_from_num!(f32);
impl_from_num!(f64);
