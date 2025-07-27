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
        let y_per_char = self.font.height() as u32 + self.y_spacing;
        let wrap_points = wrap_variable_width(
            font,
            &self.content,
            self.x_spacing as usize,
            region.w as usize,
        );

        // wrap_points and self.content.split(" ") should always be
        // the same length. are we correct on that assumption?
        assert_eq!(wrap_points.len(), self.content.split(' ').count());

        self.content
            .split(' ')
            .zip(wrap_points)
            .for_each(|(word, (start_x_pos, current_row))| {
                word.chars()
                    .enumerate()
                    .scan(start_x_pos, |x_pos, (_, character)| {
                        let (frame_x, frame_y) = (region.x, region.y);

                        let line_number = current_row as u32;

                        let char_x = frame_x + *x_pos as u32;
                        let char_y = frame_y + line_number * y_per_char;

                        let char_width = font.draw_character(
                            &mut Buffer {
                                data: frame,
                                width: WIDTH as usize,
                            },
                            char_x as usize,
                            char_y as usize,
                            character,
                        );

                        *x_pos += char_width + self.x_spacing as usize;
                        Some(*x_pos)
                    })
                    .last();
            });
    }

    fn wrap(&self, width: u32) -> Option<u32> {
        let hard_wrap = false;
        if hard_wrap {
            self.hard_wrap(width)
        } else {
            let positions = wrap_variable_width(
                self.font,
                &self.content,
                self.x_spacing as usize,
                width as usize,
            );
            //TODO: don't add final character spacing
            let (_, num_rows) = *positions.last().unwrap_or(&(0, 0));

            Some((num_rows as u32 + 1) * (self.font.height() as u32 + self.y_spacing))
        }
    }
}

pub fn wrap_variable_width(
    font: &FontType,
    content: &str,
    character_padding: usize,
    width_px: usize,
) -> Vec<(usize, usize)> {
    let space_width = font.glyph_width(' ');
    content
        .split(' ')
        .scan((0, 0), |(current_column, current_row), word| {
            let word_length = word_length(font, word, character_padding);
            // We don't handle this yet
            assert!(word_length <= width_px);

            // 3 cases:
            let (next_row, start_column) = if *current_column == 0 {
                // Very first word
                (0, 0)
            } else if (*current_column + word_length + space_width) >= width_px {
                // We need to wrap
                (*current_row + 1, 0)
            } else {
                // Move along the row
                (*current_row, *current_column + space_width)
            };

            *current_row = next_row;
            *current_column = start_column + word_length;
            Some((start_column, *current_row))
        })
        .collect()
}

fn word_length(font: &FontType, word: &str, character_padding: usize) -> usize {
    word.chars()
        .map(|c| font.glyph_width(c) + character_padding)
        .sum()
}

#[cfg(test)]
mod test {
    fn wrap(content: &str, width: usize) -> Vec<(usize, usize)> {
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
