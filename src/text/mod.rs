pub mod format;
pub mod large;
pub mod pos;
pub mod small;
pub mod wrapping;
pub mod normal;

use crate::drawing::Renderable;
use crate::text::format::TextFormat;
use crate::text::pos::TextPos;
use crate::Graphics;

#[derive(Debug, Clone)]
pub struct Text {
    content: Vec<Vec<u8>>,
    pos: TextPos,
    formatting: TextFormat,
}

impl Text {
    pub fn new<P: Into<TextPos>, F: Into<TextFormat>>(content: String, pos: P, formatting: F) -> Self {
        let formatting = formatting.into();
        let content = formatting.wrapping().wrap(&content);
        let content = content.iter().map(|line| line.chars().map(|c| {
            if c.is_ascii() {
                c as u8
            } else if c == '…' {
                31
            } else{
                0
            }
        }).collect::<Vec<u8>>()).collect();
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
}

impl Renderable for Text {
    fn render(&self, graphics: &mut Graphics) {
        graphics.draw_ascii(&self.content, self.pos, self.formatting.clone());
    }
}

/// TextSize is used to set the size and positioning in pixels of text
#[derive(Copy, Clone, Debug, Hash)]
pub enum TextSize {
    Small,
    Normal,
    Large,
}

impl Default for TextSize {
    fn default() -> Self {
        TextSize::Normal
    }
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
    pub const fn get_px(&self, chr: char) -> &[bool] {
        match self {
            TextSize::Small => small::get_px(chr),
            TextSize::Large => large::get_px(chr)
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
