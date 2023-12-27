use crate::color::Color;
use crate::text::wrapping::WrappingStrategy;
use crate::text::TextSize;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Characters be drawn be at idx * char_width, idx * char_height
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextFormat {
    color: Color,
    size: TextSize,
    wrap_at: WrappingStrategy,
    char_height: isize,
    char_width: isize,
    positioning: Positioning,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Copy, Default)]
pub enum Positioning {
    #[default]
    LeftTop,
    CenterTop,
    RightTop,
    LeftCenter,
    Center,
    RightCenter,
    LeftBottom,
    CenterBottom,
    RightBottom,
}

impl Positioning {
    pub fn calc(&self, xy: (isize, isize), width: usize, height: usize) -> (isize, isize) {
        let factor = match self {
            Positioning::LeftTop => (0.0, 0.0),
            Positioning::CenterTop => (-0.5, 0.0),
            Positioning::RightTop => (-1.0, 0.0),
            Positioning::LeftCenter => (0.0, -0.5),
            Positioning::Center => (-0.5, -0.5),
            Positioning::RightCenter => (-1.0, -0.5),
            Positioning::LeftBottom => (0.0, -1.0),
            Positioning::CenterBottom => (-0.5, -1.0),
            Positioning::RightBottom => (-1.0, -1.0),
        };
        (
            xy.0 + (factor.0 * width as f32) as isize,
            xy.1 + (factor.1 * height as f32) as isize,
        )
    }
}

impl TextFormat {
    pub const fn new(
        wrap_at: WrappingStrategy,
        size: TextSize,
        color: Color,
        positioning: Positioning,
    ) -> Self {
        Self {
            wrap_at,
            size,
            color,
            char_height: (size.get_size().1 + size.get_spacing()) as isize,
            char_width: (size.get_size().0 + size.get_spacing()) as isize,
            positioning,
        }
    }

    pub const fn new_with_spacing(
        wrap_at: WrappingStrategy,
        size: TextSize,
        color: Color,
        char_height: isize,
        char_width: isize,
        positioning: Positioning,
    ) -> Self {
        Self {
            wrap_at,
            size,
            color,
            char_height,
            char_width,
            positioning,
        }
    }
}

impl Default for TextFormat {
    fn default() -> Self {
        let size = TextSize::default();
        Self {
            wrap_at: WrappingStrategy::default(),
            size,
            color: Color::default(),
            char_height: (size.get_size().1 + size.get_spacing()) as isize,
            char_width: (size.get_size().0 + size.get_spacing()) as isize,
            positioning: Positioning::LeftTop,
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
        self.char_height
    }

    #[inline]
    pub fn letter_spacing(&self) -> isize {
        self.char_width
    }

    #[inline]
    pub fn positioning(&self) -> Positioning {
        self.positioning
    }

    #[inline]
    pub fn with_color(&self, color: Color) -> Self {
        TextFormat { color, ..*self }
    }
}

impl From<(Color, TextSize, WrappingStrategy, isize, isize, Positioning)> for TextFormat {
    fn from(
        (color, size, wrap_at, char_height, char_width, positioning): (
            Color,
            TextSize,
            WrappingStrategy,
            isize,
            isize,
            Positioning,
        ),
    ) -> Self {
        TextFormat {
            color,
            size,
            wrap_at,
            char_height,
            char_width,
            positioning,
        }
    }
}

impl From<(Color, TextSize, WrappingStrategy, isize, isize)> for TextFormat {
    fn from(
        (color, size, wrap_at, char_height, char_width): (
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
            char_height,
            char_width,
            ..Self::default()
        }
    }
}

impl From<(Color, TextSize, WrappingStrategy, isize)> for TextFormat {
    fn from(
        (color, size, wrap_at, char_height): (Color, TextSize, WrappingStrategy, isize),
    ) -> Self {
        TextFormat {
            color,
            size,
            wrap_at,
            char_height,
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

impl From<(Color, TextSize, WrappingStrategy, Positioning)> for TextFormat {
    fn from(
        (color, size, wrap_at, positioning): (Color, TextSize, WrappingStrategy, Positioning),
    ) -> Self {
        TextFormat {
            color,
            size,
            wrap_at,
            positioning,
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

impl From<(Color, TextSize, Positioning)> for TextFormat {
    fn from((color, size, positioning): (Color, TextSize, Positioning)) -> Self {
        TextFormat {
            color,
            size,
            positioning,
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
