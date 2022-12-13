use crate::text::TextSize;
use graphics_shapes::coord::Coord;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TextPos {
    Px(isize, isize),
    /// See [TextSize::get_max_characters] for maximum x and y
    ColRow(usize, usize),
}

impl TextPos {
    pub fn to_coord(&self, size: TextSize) -> (isize, isize) {
        match self {
            TextPos::Px(x, y) => (*x, *y),
            TextPos::ColRow(col, row) => (
                (col * (size.get_size().0 + size.get_spacing())) as isize,
                (row * (size.get_size().1 + size.get_spacing())) as isize,
            ),
        }
    }

    pub fn px<P: Into<Coord>>(coord: P) -> TextPos {
        let coord = coord.into();
        TextPos::Px(coord.x, coord.y)
    }
}

pub trait CoordIntoTextPos {
    fn textpos(self) -> TextPos;
}

impl CoordIntoTextPos for Coord {
    fn textpos(self) -> TextPos {
        TextPos::px(self)
    }
}

pub trait NewTextPos<T> {
    /// Creates a new TextPos::ColRow
    fn cr(xy: (T, T)) -> TextPos;
    /// Creates a new TextPos::Px
    fn px(xy: (T, T)) -> TextPos;
}

macro_rules! impl_to_px {
    ($num_type: ty) => {
        impl NewTextPos<$num_type> for TextPos {
            /// Calls abs() on x and y
            fn cr((x, y): ($num_type, $num_type)) -> Self {
                TextPos::ColRow(((x as isize).abs()) as usize, ((y as isize).abs()) as usize)
            }

            fn px((x, y): ($num_type, $num_type)) -> Self {
                TextPos::Px(x as isize, y as isize)
            }
        }
    };
}

impl_to_px!(isize);
impl_to_px!(i8);
impl_to_px!(i16);
impl_to_px!(i32);
impl_to_px!(i64);
impl_to_px!(i128);
impl_to_px!(usize);
impl_to_px!(u8);
impl_to_px!(u16);
impl_to_px!(u32);
impl_to_px!(u64);
impl_to_px!(u128);
impl_to_px!(f32);
impl_to_px!(f64);

#[cfg(test)]
mod test {
    use crate::text::pos::{CoordIntoTextPos, TextPos};
    use graphics_shapes::coord::Coord;

    #[test]
    fn coord_textpos() {
        let coord = Coord::new(0, 0);
        let test = coord.textpos();
        assert_eq!(test, TextPos::Px(0, 0))
    }
}
