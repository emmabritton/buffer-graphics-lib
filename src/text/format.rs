use ici_files::prelude::*;
use crate::text::wrapping::WrappingStrategy;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::text::PixelFont;

/// Characters be drawn be at idx * char_width, idx * char_height
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TextFormat {
    color: Color,
    font: PixelFont,
    wrap_at: WrappingStrategy,
    line_height: f32,
    char_width: f32,
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
        size: PixelFont,
        color: Color,
        positioning: Positioning,
    ) -> Self {
        Self {
            wrap_at,
            font: size,
            color,
            line_height: 1.0,
            char_width: 1.0,
            positioning,
        }
    }

    pub const fn new_with_spacing(
        wrap_at: WrappingStrategy,
        size: PixelFont,
        color: Color,
        line_height: f32,
        char_width: f32,
        positioning: Positioning,
    ) -> Self {
        Self {
            wrap_at,
            font: size,
            color,
            line_height,
            char_width,
            positioning,
        }
    }
}

impl Default for TextFormat {
    fn default() -> Self {
        let size = PixelFont::default();
        Self {
            wrap_at: WrappingStrategy::default(),
            font: size,
            color: Color::default(),
            line_height: 1.0,
            char_width: 1.0,
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
    pub fn font(&self) -> PixelFont {
        self.font
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn line_height(&self) -> isize {
        (self.line_height * (self.font.size().1 + self.font.spacing()) as f32).round() as isize
    }

    #[inline]
    pub fn char_width(&self) -> isize {
        (self.char_width * (self.font.size().0 + self.font.spacing()) as f32).round() as isize
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

impl From<(Color, PixelFont, WrappingStrategy, f32, f32, Positioning)> for TextFormat {
    fn from(
        (color, size, wrap_at, line_height, char_width, positioning): (
            Color,
            PixelFont,
            WrappingStrategy,
            f32,
            f32,
            Positioning,
        ),
    ) -> Self {
        TextFormat {
            color,
            font: size,
            wrap_at,
            line_height,
            char_width,
            positioning,
        }
    }
}

impl From<(Color, PixelFont, WrappingStrategy, f32, f32)> for TextFormat {
    fn from(
        (color, size, wrap_at, line_height, char_width): (
            Color,
            PixelFont,
            WrappingStrategy,
            f32,
            f32,
        ),
    ) -> Self {
        TextFormat {
            color,
            font: size,
            wrap_at,
            line_height,
            char_width,
            ..Self::default()
        }
    }
}

impl From<(Color, PixelFont, WrappingStrategy, f32)> for TextFormat {
    fn from(
        (color, size, wrap_at, line_height): (Color, PixelFont, WrappingStrategy, f32),
    ) -> Self {
        TextFormat {
            color,
            font: size,
            wrap_at,
            line_height,
            ..Self::default()
        }
    }
}

impl From<(Color, PixelFont, WrappingStrategy)> for TextFormat {
    fn from((color, size, wrap_at): (Color, PixelFont, WrappingStrategy)) -> Self {
        TextFormat {
            color,
            font: size,
            wrap_at,
            ..Self::default()
        }
    }
}

impl From<(Color, PixelFont, WrappingStrategy, Positioning)> for TextFormat {
    fn from(
        (color, size, wrap_at, positioning): (Color, PixelFont, WrappingStrategy, Positioning),
    ) -> Self {
        TextFormat {
            color,
            font: size,
            wrap_at,
            positioning,
            ..Self::default()
        }
    }
}

impl From<(Color, PixelFont)> for TextFormat {
    fn from((color, size): (Color, PixelFont)) -> Self {
        TextFormat {
            color,
            font: size,
            ..Self::default()
        }
    }
}

impl From<(Color, PixelFont, Positioning)> for TextFormat {
    fn from((color, size, positioning): (Color, PixelFont, Positioning)) -> Self {
        TextFormat {
            color,
            font: size,
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
