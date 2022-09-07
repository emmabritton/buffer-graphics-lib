use crate::text::TextSize;

#[derive(Copy, Clone, Debug)]
pub enum TextPos {
    Px(isize, isize),
    /// See [TextSize::get_max_characters] for maximum x and y
    ColRow(usize, usize),
}

impl TextPos {
    pub fn to_px(&self, size: TextSize) -> (isize, isize) {
        match self {
            TextPos::Px(x, y) => (*x,*y),
            TextPos::ColRow(col, row) => ((col * (size.get_size().0 + size.get_spacing())) as isize, (row * size.get_size().1) as isize)
        }
    }
}

macro_rules! impl_to_px {
    ($num_type: ty) => {
        impl From<($num_type, $num_type)> for TextPos {
            fn from((x, y): ($num_type, $num_type)) -> Self {
                TextPos::Px(x as isize, y as isize)
            }
        }
    };
}

macro_rules! impl_to_colrow {
    ($num_type: ty) => {
        impl From<($num_type, $num_type)> for TextPos {
            fn from((x, y): ($num_type, $num_type)) -> Self {
                TextPos::ColRow(x as usize, y as usize)
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

impl_to_colrow!(usize);
impl_to_colrow!(u8);
impl_to_colrow!(u16);
impl_to_colrow!(u32);
impl_to_colrow!(u64);
impl_to_colrow!(u128);

impl TextPos {
    pub fn usize_px(x: usize, y: usize) -> TextPos {
        TextPos::Px(x as isize, y as isize)
    }

    pub fn f32_px(x: f32, y: f32) -> TextPos {
        TextPos::Px(x as isize, y as isize)
    }
}
