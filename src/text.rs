use std::sync::LazyLock;

use crate::buffer::Buffer;
use crate::element::Element;
use crate::font::{self, Font, FontType};
use crate::layout::CalculatedLayout;

const WIDTH: u32 = 320; // TODO make this metadata for the frame buffer
                        //
#[derive(Clone)]
pub struct Text {
    pub content: String,
    pub font: &'static LazyLock<FontType>,
    x_spacing: u32,
    y_spacing: u32,
    //pub font: &'static FontType,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            content,
            font: &font::OLDSCHOOL,
            x_spacing: 1,
            y_spacing: 1,
        }
    }

    fn usable_width(width: u32) -> u32 {
        width.next_multiple_of(6) - 6
    }

    pub fn with_font(self, font: &'static LazyLock<FontType>) -> Self {
        Self { font, ..self }
    }
}

impl Element for Text {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
        let font = &self.font;

        self.content
            .chars()
            .enumerate()
            .for_each(|(character_index, character)| {
                let (frame_x, frame_y) = (region.x, region.y);
                let linear_progress =
                    character_index as u32 * (self.font.width() as u32 + self.x_spacing);
                let usable_width = Self::usable_width(region.w);

                let line_number = linear_progress / usable_width;
                let column_number = linear_progress % usable_width;

                let char_x = frame_x + column_number;
                let char_y = frame_y + line_number * (self.font.height() as u32 + self.y_spacing);

                font.draw_character(
                    &mut Buffer {
                        data: frame,
                        width: WIDTH as usize,
                    },
                    char_x as usize,
                    char_y as usize,
                    character,
                );
            });
    }

    fn wrap(&self, width: u32) -> Option<u32> {
        let length = self.content.len() as u32 * (self.font.width() as u32 + self.x_spacing);
        let lines = length / Self::usable_width(width) + 1; // TODO + 1 is a hack

        Some(lines * (self.font.height() as u32 + self.y_spacing) - self.y_spacing)
    }
}
