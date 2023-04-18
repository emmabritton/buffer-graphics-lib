use crate::color::Color;
use ici_files::IciColor;

pub trait ToIci {
    fn to_ici(&self) -> IciColor;
}

pub trait ToColor {
    fn to_color(&self) -> Color;
}

impl ToColor for IciColor {
    #[inline]
    fn to_color(&self) -> Color {
        Color::rgba(self.r, self.g, self.b, self.a)
    }
}

impl ToIci for Color {
    #[inline]
    fn to_ici(&self) -> IciColor {
        IciColor::new(self.r, self.g, self.b, self.a)
    }
}
