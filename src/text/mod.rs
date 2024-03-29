pub mod format;
pub mod large;
pub mod normal;
pub mod pos;
pub mod small;
pub mod wrapping;

use crate::color::Color;
use crate::drawing::Renderable;
use crate::text::format::TextFormat;
use crate::text::pos::TextPos;
use crate::text::wrapping::WrappingStrategy;
use crate::Graphics;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const ASCII_DEGREE: u8 = 30;
pub const ASCII_ELLIPSIS: u8 = 31;
pub const ASCII_CURRENCY: u8 = 29;
pub const ASCII_POUND: u8 = 28;
pub const ASCII_YEN: u8 = 27;
pub const ASCII_CENT: u8 = 26;
pub const ASCII_CHECK: u8 = 25;

pub const SUPPORTED_SYMBOLS: [char; 38] = [
    '!', '@', '£', '$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=', '#', '{', '}', ':', '"',
    '|', '<', '?', '>', ',', '/', '.', ';', '\'', '\\', '[', ']', '`', '~', '°', '…', '¢', '¥',
    '✓',
];

const fn custom_ascii_code(chr: char) -> u8 {
    match chr {
        '°' => ASCII_DEGREE,
        '…' => ASCII_ELLIPSIS,
        '¤' => ASCII_CURRENCY,
        '£' => ASCII_POUND,
        '¥' => ASCII_YEN,
        '¢' => ASCII_CENT,
        '✓' => ASCII_CHECK,
        _ => 0,
    }
}

pub const fn chr_to_code(chr: char) -> u8 {
    if chr.is_ascii() {
        chr as u8
    } else {
        custom_ascii_code(chr)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Text {
    content: Vec<Vec<u8>>,
    pos: TextPos,
    formatting: TextFormat,
}

impl Text {
    pub fn new<P: Into<TextPos>, F: Into<TextFormat>>(
        content: &str,
        pos: P,
        formatting: F,
    ) -> Self {
        let formatting = formatting.into();
        let content = formatting.wrapping().wrap(content);
        let content = content
            .iter()
            .map(|line| line.chars().map(chr_to_code).collect::<Vec<u8>>())
            .collect();
        Self {
            content,
            pos: pos.into(),
            formatting,
        }
    }
}

impl Text {
    #[inline]
    pub fn pos(&self) -> TextPos {
        self.pos
    }

    #[inline]
    pub fn formatting(&self) -> &TextFormat {
        &self.formatting
    }

    /// As text wrapping is calculated when the text object is created
    /// changing it as no effect
    pub fn with_formatting<F: Into<TextFormat>>(&self, format: F) -> Self {
        let format = format.into();
        Text {
            content: self.content.clone(),
            pos: self.pos,
            formatting: format,
        }
    }

    #[inline]
    pub fn with_color(&self, color: Color) -> Self {
        self.with_formatting(self.formatting().with_color(color))
    }

    pub fn with_pos<P: Into<TextPos>>(&self, pos: P) -> Self {
        let pos = pos.into();
        Text {
            content: self.content.clone(),
            pos,
            formatting: self.formatting.clone(),
        }
    }
}

impl Renderable<Text> for Text {
    fn render(&self, graphics: &mut Graphics) {
        graphics.draw_ascii(&self.content, self.pos, self.formatting.clone());
    }
}

/// TextSize is used to set the size and positioning in pixels of text
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub enum TextSize {
    Small,
    #[default]
    Normal,
    Large,
}

impl TextSize {
    /// Returns width, height of text size in pixels
    #[inline]
    pub const fn get_size(&self) -> (usize, usize) {
        match self {
            TextSize::Small => (small::CHAR_WIDTH, small::CHAR_HEIGHT),
            TextSize::Normal => (normal::CHAR_WIDTH, normal::CHAR_HEIGHT),
            TextSize::Large => (large::CHAR_WIDTH, large::CHAR_HEIGHT),
        }
    }

    pub fn measure(&self, text: &str, wrapping: WrappingStrategy) -> (usize, usize) {
        let lines = wrapping.wrap(text);
        let longest_len = lines.iter().map(|line| line.len()).max().unwrap();
        let (w, h) = self.get_size();
        let spacing = self.get_spacing();
        (longest_len * (w + spacing), lines.len() * (h + spacing))
    }

    /// Returns the spacing between letters in pixels
    #[inline]
    pub const fn get_spacing(&self) -> usize {
        match self {
            TextSize::Small => 1,
            TextSize::Normal => 1,
            TextSize::Large => 2,
        }
    }

    #[inline]
    pub const fn get_px_ascii(&self, code: u8) -> &[bool] {
        match self {
            TextSize::Small => small::get_px_ascii(code),
            TextSize::Normal => normal::get_px_ascii(code),
            TextSize::Large => large::get_px_ascii(code),
        }
    }

    /// Converts pixels to columns
    #[inline]
    pub const fn px_to_cols(&self, px: usize) -> usize {
        px / (self.get_size().0 + self.get_spacing())
    }

    /// Returns the max of (columns, rows) for this text size for the specified screen size
    pub fn get_max_characters(&self, screen_width: usize, screen_height: usize) -> (usize, usize) {
        let size = self.get_size();
        if screen_width < size.0 || screen_height < size.1 {
            return (0, 0);
        }
        let sw = screen_width as f32;
        let cw = (size.0 + self.get_spacing()) as f32;
        let sh = screen_height as f32;
        let ch = (size.1 + self.get_spacing()) as f32;
        let columns = (sw / cw).floor() as usize;
        let rows = (sh / ch).floor() as usize;
        (columns - 1, rows - 1)
    }
}
