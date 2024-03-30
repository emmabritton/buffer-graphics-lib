pub mod format;
pub mod pos;
pub mod wrapping;
pub mod font;

use graphics_shapes::prelude::Rect;
use ici_files::prelude::*;
use crate::drawing::Renderable;
use crate::text::format::TextFormat;
use crate::text::pos::TextPos;
use crate::Graphics;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::prelude::font::*;

//from Latin-1/ISO 8859-1
pub const ASCII_DEGREE: u8 = 176;
pub const ASCII_CURRENCY: u8 = 164;
pub const ASCII_EURO: u8 = 127;
pub const ASCII_POUND: u8 = 163;
pub const ASCII_YEN: u8 = 165;
pub const ASCII_CENT: u8 = 162;

//override control codes
pub const ASCII_ELLIPSIS: u8 = 31;
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
        '€' => ASCII_EURO,
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
#[derive(Debug, PartialEq)]
pub struct Text {
    content: Vec<Vec<u8>>,
    pos: TextPos,
    formatting: TextFormat,
    bounds: Rect
}

impl Text {
    pub fn new<P: Into<TextPos>, F: Into<TextFormat>>(
        content: &str,
        pos: P,
        formatting: F,
    ) -> Self {
        let formatting = formatting.into();
        let content = formatting.wrapping().wrap(content);
        let content: Vec<Vec<u8>> = content
            .iter()
            .map(|line| line.chars().map(chr_to_code).collect::<Vec<u8>>())
            .collect();
        let pos = pos.into();
        let bounds = Self::calc_bounds(pos, &formatting, &content);
        Self {
            content,
            pos,
            formatting,
            bounds
        }
    }

    fn calc_bounds(pos: TextPos, formatting: &TextFormat, text: &[Vec<u8>]) -> Rect {
        let content =text.iter().map(|chrs| String::from_utf8_lossy(chrs).to_string()).collect::<Vec<String>>().join("\n");
        let pos_coord = pos.to_coord(formatting.font());
        let (w,h) = formatting.font().measure(&content);
        Rect::new_with_size(formatting.positioning().calc(pos_coord,w,h), w,h)
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

    #[inline]
    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }

    #[inline]
    pub fn contents(&self) -> &[Vec<u8>] {
        &self.content
    }

    fn with_formatting<F: Into<TextFormat>>(&self, format: F) -> Self {
        let format = format.into();
        Text {
            content: self.content.clone(),
            pos: self.pos,
            formatting: format,
            bounds: self.bounds.clone()
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
            bounds: Self::calc_bounds(pos, &self.formatting, &self.content)
        }
    }
}

impl Renderable<Text> for Text {
    fn render(&self, graphics: &mut Graphics) {
        graphics.draw_ascii(&self.content, self.pos, self.formatting.clone());
    }
}

/// Return size in pixels for text
/// Run `text` through wrapping strategy first
pub fn measure_text(text: &str, char_width: usize, line_height: usize) -> (usize, usize) {
    let lines = text.split('\n').collect::<Vec<_>>();
    let longest_len = lines.iter().map(|line| line.len()).max().unwrap();
    (longest_len * char_width, lines.len() * line_height)
}

/// PixelFont is used to set the size and positioning in pixels of text
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub enum PixelFont {
    Outline7x9,
    Standard4x4,
    Standard4x5,
    #[default]
    Standard6x7,
    Standard8x10,
    Script8x8,
}

impl PixelFont {
    /// Returns width, height of text size in pixels
    #[inline]
    pub const fn size(&self) -> (usize, usize) {
        match self {
            PixelFont::Outline7x9 => (outline_7x9::CHAR_WIDTH, outline_7x9::CHAR_HEIGHT),
            PixelFont::Standard4x4 => (standard_4x4::CHAR_WIDTH, standard_4x4::CHAR_HEIGHT),
            PixelFont::Standard4x5 => (standard_4x5::CHAR_WIDTH, standard_4x5::CHAR_HEIGHT),
            PixelFont::Standard6x7 => (standard_6x7::CHAR_WIDTH, standard_6x7::CHAR_HEIGHT),
            PixelFont::Standard8x10 => (standard_8x10::CHAR_WIDTH, standard_8x10::CHAR_HEIGHT),
            PixelFont::Script8x8 => (script_8x8::CHAR_WIDTH, script_8x8::CHAR_HEIGHT),
        }
    }

    /// Return size in pixels for text
    /// Run `text` through wrapping strategy first
    #[inline]
    pub fn measure(&self, text: &str) -> (usize, usize) {
        measure_text(text, self.char_width(), self.line_height())
    }

    #[inline]
    pub fn char_width(&self) -> usize {
        self.size().0 + self.spacing()
    }

    #[inline]
    pub fn line_height(&self) -> usize {
        self.size().1 + self.spacing()
    }

    /// Returns the spacing between letters in pixels
    #[inline]
    pub const fn spacing(&self) -> usize {
        match self {
            PixelFont::Outline7x9 => 0,
            PixelFont::Standard4x4 => 1,
            PixelFont::Standard4x5 => 1,
            PixelFont::Standard6x7 => 1,
            PixelFont::Standard8x10 => 2,
            PixelFont::Script8x8 => 1,
        }
    }

    #[inline]
    pub const fn pixels(&self, code: u8) -> &[bool] {
        match self {
            PixelFont::Outline7x9 => outline_7x9::get_px_ascii(code),
            PixelFont::Standard4x4 => standard_4x4::get_px_ascii(code),
            PixelFont::Standard4x5 => standard_4x5::get_px_ascii(code),
            PixelFont::Standard6x7 => standard_6x7::get_px_ascii(code),
            PixelFont::Standard8x10 => standard_8x10::get_px_ascii(code),
            PixelFont::Script8x8 => script_8x8::get_px_ascii(code),
        }
    }

    /// Converts pixels to columns
    #[inline]
    pub const fn px_to_cols(&self, px: usize) -> usize {
        px / (self.size().0 + self.spacing())
    }

    /// Returns the max of (columns, rows) for this text size for the specified screen size
    pub fn get_max_characters(&self, screen_width: usize, screen_height: usize) -> (usize, usize) {
        let size = self.size();
        if screen_width < size.0 || screen_height < size.1 {
            return (0, 0);
        }
        let sw = screen_width as f32;
        let cw = (size.0 + self.spacing()) as f32;
        let sh = screen_height as f32;
        let ch = (size.1 + self.spacing()) as f32;
        let columns = (sw / cw).floor() as usize;
        let rows = (sh / ch).floor() as usize;
        (columns - 1, rows - 1)
    }
}
