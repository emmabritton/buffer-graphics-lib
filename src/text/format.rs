use crate::color::Color;
use crate::text::wrapping::WrappingStrategy;
use crate::text::TextSize;

#[derive(Debug, Clone, Default)]
pub struct TextFormat {
    color: Color,
    size: TextSize,
    wrap_at: WrappingStrategy,
    line_spacing: isize,
    letter_spacing: isize,
    per_letter_adjustment: (isize, isize),
}

impl TextFormat {
    pub fn new(wrap_at: WrappingStrategy, size: TextSize, color: Color) -> Self {
        Self {
            wrap_at,
            size,
            color,
            line_spacing: (size.get_size().1 + size.get_spacing()) as isize,
            letter_spacing: size.get_spacing() as isize,
            per_letter_adjustment: (0, 0),
        }
    }

    pub fn new_with_spacing(
        wrap_at: WrappingStrategy,
        size: TextSize,
        color: Color,
        line_height: isize,
        kerning: isize,
        per_letter_adjustment: (isize, isize),
    ) -> Self {
        Self {
            wrap_at,
            size,
            color,
            line_spacing: line_height,
            letter_spacing: kerning,
            per_letter_adjustment,
        }
    }
}

impl TextFormat {
    #[inline]
    pub fn wrapping(&self) -> WrappingStrategy {
        self.wrap_at
    }

    #[inline]
    pub fn size(&self) -> TextSize {
        self.size
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn line_spacing(&self) -> isize {
        self.line_spacing
    }
    #[inline]
    pub fn letter_spacing(&self) -> isize {
        self.letter_spacing
    }
    #[inline]
    pub fn per_letter_adjustment(&self) -> (isize, isize) {
        self.per_letter_adjustment
    }
    pub fn with_color(&self, color: Color) -> Self {
        TextFormat { color, ..*self }
    }
}

impl
    From<(
        Color,
        TextSize,
        WrappingStrategy,
        isize,
        isize,
        (isize, isize),
    )> for TextFormat
{
    fn from(
        (color, size, wrap_at, line_height, kerning, per_letter_adjustment): (
            Color,
            TextSize,
            WrappingStrategy,
            isize,
            isize,
            (isize, isize),
        ),
    ) -> Self {
        TextFormat {
            color,
            size,
            wrap_at,
            line_spacing: line_height,
            letter_spacing: kerning,
            per_letter_adjustment,
        }
    }
}

impl From<(Color, TextSize, WrappingStrategy, isize, isize)> for TextFormat {
    fn from(
        (color, size, wrap_at, line_height, kerning): (
            Color,
            TextSize,
            WrappingStrategy,
            isize,
            isize,
        ),
    ) -> Self {
        TextFormat {
            color,
            size,
            wrap_at,
            line_spacing: line_height,
            letter_spacing: kerning,
            ..Self::default()
        }
    }
}

impl From<(Color, TextSize, WrappingStrategy, isize)> for TextFormat {
    fn from(
        (color, size, wrap_at, line_height): (Color, TextSize, WrappingStrategy, isize),
    ) -> Self {
        TextFormat {
            color,
            size,
            wrap_at,
            line_spacing: line_height,
            ..Self::default()
        }
    }
}

impl From<(Color, TextSize, WrappingStrategy)> for TextFormat {
    fn from((color, size, wrap_at): (Color, TextSize, WrappingStrategy)) -> Self {
        TextFormat {
            color,
            size,
            wrap_at,
            ..Self::default()
        }
    }
}

impl From<(Color, TextSize)> for TextFormat {
    fn from((color, size): (Color, TextSize)) -> Self {
        TextFormat {
            color,
            size,
            ..Self::default()
        }
    }
}

impl From<Color> for TextFormat {
    fn from(color: Color) -> Self {
        TextFormat {
            color,
            ..Self::default()
        }
    }
}
