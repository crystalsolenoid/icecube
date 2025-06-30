use std::{char, sync::LazyLock};

use image::{ImageBuffer, Luma};

use crate::{buffer::Buffer, palette::BLUE_LIGHT};

pub static TEST_FONT: LazyLock<ImageFont> = std::sync::LazyLock::new(|| ImageFont::test_font());

pub struct ImageFont {
    font_image: ImageBuffer<Luma<u8>, Vec<u8>>,
    character_width: usize,
    character_height: usize,
    first_char: u8,
    last_char: u8,
    characters_per_row: usize,
    x_spacing: usize,
    y_spacing: usize,
    y_margin: usize,
}

impl ImageFont {
    pub fn test_font() -> Self {
        let font = include_bytes!("resources/test_font.png");
        let font_image = image::load_from_memory(font)
            .expect("failed to parse test font")
            .into_luma8();

        Self {
            font_image,
            first_char: b' ',
            last_char: b'~',
            character_width: 6,
            character_height: 7,
            characters_per_row: 18,
            x_spacing: 1,
            y_spacing: 2,
            y_margin: 1,
        }
    }
    pub fn mono_5_8() -> Self {
        let font = include_bytes!("resources/5x8mono.png");
        let font_image = image::load_from_memory(font)
            .expect("failed to parse test font")
            .into_luma8();

        Self {
            font_image,
            first_char: b'!',
            last_char: b'~',
            character_width: 5,
            character_height: 8,
            characters_per_row: (b'~' - b'!' + 1) as usize,
            x_spacing: 1,
            y_spacing: 1,
            y_margin: 1,
        }
    }
}

impl Font for ImageFont {
    fn width(&self) -> usize {
        self.character_width
    }
    fn height(&self) -> usize {
        self.character_height
    }
    fn fallback_character() -> char {
        '?'
    }

    fn draw_character(
        &self,
        buffer: &mut Buffer,
        screen_x: usize,
        screen_y: usize,
        character: char,
    ) {
        let index = match character {
            ' ' => return,
            c if self.first_char <= c as u8 && self.last_char >= c as u8 => c,
            _ => Self::fallback_character(),
        } as u8
            - self.first_char;

        let x = index as usize % self.characters_per_row;
        let y = index as usize / self.characters_per_row;

        let (img_x, img_y) = (
            (x * (self.width() + self.x_spacing)),
            (y * (self.height() + self.y_spacing)),
        );

        for j in 0..self.height() {
            for i in 0..self.width() {
                let frame_index = ((screen_x + i) + (screen_y + j) * buffer.width) * 4;

                let font_pixel = self.font_image[(
                    img_x as u32 + i as u32,
                    img_y as u32 + j as u32 + self.y_margin as u32,
                )];

                if font_pixel.0[0] > 0 && frame_index + 4 < buffer.len()
                // our current workaround for out of
                // bounds crashing
                {
                    buffer.data[frame_index..(frame_index + 4)].copy_from_slice(&BLUE_LIGHT);
                }
            }
        }
    }
}

pub trait Font {
    fn draw_character(
        &self,
        buffer: &mut Buffer,
        screen_x: usize,
        screen_y: usize,
        character: char,
    );
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn fallback_character() -> char;
}
