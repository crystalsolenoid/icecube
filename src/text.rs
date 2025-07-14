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
    fn hard_wrap(&self, width: u32) -> Option<u32> {
        let length = self.content.len() as u32 * (self.font.width() as u32 + self.x_spacing);
        let lines = length / Self::usable_width(width) + 1; // TODO + 1 is a hack

        Some(lines * (self.font.height() as u32 + self.y_spacing) - self.y_spacing)
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
        let hard_wrap = false;
        if hard_wrap {
            self.hard_wrap(width)
        } else {
            dbg!(width);
            let positions = dbg!(wrap(
                &self.content,
                width as usize / (self.font.width() + self.x_spacing as usize)
            ));
            //TODO: don't add final character spacing
            let (_, num_rows) = *positions.last().unwrap_or(&(0, 0));

            dbg!(Some(
                (num_rows as u32 + 1) * (self.font.height() as u32 + self.y_spacing)
            ))
        }
    }
}
pub fn wrap(content: &str, width: usize) -> Vec<(usize, usize)> {
    content
        .split(' ')
        .scan((0, 0), |(current_column, current_row), word| {
            // We don't handle this yet
            assert!(word.len() < width);

            // 3 cases:
            let (next_column, next_row) = if *current_column == 0 {
                // Very first word
                (word.len() - 1, 0)
            } else if (*current_column + word.len()) >= width {
                // We need to wrap
                (word.len() - 1, *current_row + 1)
            } else {
                // Move along the row
                (*current_column + word.len() + 1, *current_row)
            };

            *current_row = next_row;
            *current_column = next_column;
            Some((*current_column, *current_row))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrap_quick() {
        let test = "the quick brown fox jumps over the lazy dog";
        let width = 12;
        assert_eq!(
            wrap(test, width),
            vec![
                (2, 0),
                (8, 0),
                (4, 1),
                (8, 1),
                (4, 2),
                (9, 2),
                (2, 3),
                (7, 3),
                (11, 3)
            ]
        )
    }
    #[test]
    fn long_first_word() {
        let test = "abcdefghijkl abc";
        let width = 12;
        assert_eq!(wrap(test, width), vec![(11, 0), (2, 1)])
    }
    #[test]
    fn long_word() {
        let test = "the quick brown fox abcdefghijkl the lazy dog";
        let width = 12;
        assert_eq!(
            wrap(test, width),
            vec![
                (2, 0),
                (8, 0),
                (4, 1),
                (8, 1),
                (11, 2),
                (2, 3),
                (7, 3),
                (11, 3)
            ]
        );
    }
}
