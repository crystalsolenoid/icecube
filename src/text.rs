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
        let x_per_char = self.font.width() as u32 + self.x_spacing;
        let y_per_char = self.font.height() as u32 + self.y_spacing;
        let wrap_points = wrap(&self.content, region.w as usize / x_per_char as usize);

        // wrap_points and self.content.split(" ") should always be
        // the same length. are we correct on that assumption?
        assert_eq!(wrap_points.len(), self.content.split(' ').count());

        self.content
            .split(' ')
            .zip(wrap_points)
            .for_each(|(word, (start_column, current_row))| {
                word.chars()
                    .enumerate()
                    .for_each(|(character_index, character)| {
                        let (frame_x, frame_y) = (region.x, region.y);

                        let line_number = current_row as u32;
                        let column_number = (start_column + character_index) as u32;

                        let char_x = frame_x + column_number * x_per_char;
                        let char_y = frame_y + line_number * y_per_char;

                        font.draw_character(
                            &mut Buffer {
                                data: frame,
                                width: WIDTH as usize,
                            },
                            char_x as usize,
                            char_y as usize,
                            character,
                        );
                    })
            });
    }

    fn wrap(&self, width: u32) -> Option<u32> {
        let hard_wrap = false;
        if hard_wrap {
            self.hard_wrap(width)
        } else {
            let positions = wrap(
                &self.content,
                width as usize / (self.font.width() + self.x_spacing as usize),
            );
            //TODO: don't add final character spacing
            let (_, num_rows) = *positions.last().unwrap_or(&(0, 0));

            Some((num_rows as u32 + 1) * (self.font.height() as u32 + self.y_spacing))
        }
    }
}

pub fn wrap(content: &str, width: usize) -> Vec<(usize, usize)> {
    content
        .split(' ')
        .scan((0, 0), |(current_column, current_row), word| {
            // We don't handle this yet
            assert!(word.len() <= width);

            // 3 cases:
            let (next_row, start_column) = if *current_column == 0 {
                // Very first word
                (0, 0)
            } else if (*current_column + word.len()) >= width {
                // We need to wrap
                (*current_row + 1, 0)
            } else {
                // Move along the row
                (*current_row, *current_column + 1)
            };

            *current_row = next_row;
            *current_column = start_column + word.len();
            Some((start_column, *current_row))
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
        // the quick   |
        // brown fox   |
        // jumps over  |
        // the lazy dog|
        assert_eq!(
            wrap(test, width),
            vec![
                (0, 0), // the
                (4, 0), // quick
                (0, 1), // brown
                (6, 1), // fox
                (0, 2), // jumps
                (6, 2), // over
                (0, 3), // the
                (4, 3), // lazy
                (9, 3)  // dog
            ]
        )
    }
    #[test]
    fn long_first_word() {
        let test = "abcdefghijkl abc";
        let width = 12;
        // abcdefghijkl|
        // abc         |
        assert_eq!(wrap(test, width), vec![(0, 0), (0, 1)])
    }
    #[test]
    fn long_word() {
        let test = "the quick brown fox abcdefghijkl the lazy dog";
        let width = 12;
        // the quick   |
        // brown fox   |
        // abcdefghijkl|
        // the lazy dog|
        assert_eq!(
            wrap(test, width),
            vec![
                (0, 0), // the
                (4, 0), // quick
                (0, 1), // brown
                (6, 1), // fox
                (0, 2), // abcdefghijkl
                (0, 3), // the
                (4, 3), // lazy
                (9, 3)  // dog
            ]
        );
    }

    #[test]
    fn one_letter_start() {
        let test = "a quick brown fox abcdefghijkl the lazy dog";
        let width = 12;
        // a quick     |
        // brown fox   |
        // abcdefghijkl|
        // the lazy dog|
        assert_eq!(
            wrap(test, width),
            vec![
                (0, 0), // a
                (2, 0), // quick
                (0, 1), // brown
                (6, 1), // fox
                (0, 2), // abcdefghijkl
                (0, 3), // the
                (4, 3), // lazy
                (9, 3)  // dog
            ]
        );
    }
}
