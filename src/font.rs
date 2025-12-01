use std::{char, sync::LazyLock};

use bdf2;
use image::{ImageBuffer, Luma};

use crate::{buffer::Buffer, palette::BLUE_LIGHT};

pub static OLDSCHOOL: LazyLock<FontType> =
    std::sync::LazyLock::new(|| FontType::Image(ImageFont::oldschool()));
pub static MONO_5_8: LazyLock<FontType> =
    std::sync::LazyLock::new(|| FontType::Image(ImageFont::mono_5_8()));
pub static BLACKLETTER: LazyLock<FontType> =
    std::sync::LazyLock::new(|| FontType::Bdf(BdfFont::blackletter()));
pub static SCRAWL: LazyLock<FontType> =
    std::sync::LazyLock::new(|| FontType::Bdf(BdfFont::scrawl()));

//pub static A_FONT: LazyLock<FontType> = std::sync::LazyLock::new(|| FontType::Image(&*TEST_FONT2));

//            font: FontType::Image(&*TEST_FONT2),

pub enum FontType {
    Image(ImageFont),
    Bdf(BdfFont),
}

pub struct BdfFont {
    font: bdf2::Font,
    space_width: usize,
    line_height: usize,
}

impl BdfFont {
    fn blackletter() -> Self {
        Self {
            //TODO: Consider adding a font feature flag to exclude this data from the binary
            font: bdf2::read(
                &include_bytes!("../src/resources/NotJam/Blackletter/NotJamBlkltr13-13.bdf")[..],
            )
            .unwrap(),
            space_width: 8,
            line_height: 14,
        }
    }
    fn scrawl() -> Self {
        Self {
            font: bdf2::read(&include_bytes!("../src/resources/NotJam/Scrawl/scrawl9-9.bdf")[..])
                .unwrap(),
            space_width: 6,
            line_height: 10,
        }
    }
}

impl Font for BdfFont {
    fn draw_character(
        &self,
        buffer: &mut Buffer,
        screen_x: usize,
        screen_y: usize,
        character: char,
    ) -> usize {
        let glyph = self.font.glyphs().get(&character).unwrap();
        let y_offset = glyph.bounds().y;
        let glyph_height = glyph.bounds().height;

        let baseline = screen_y as i32 + (self.height() as u32 - glyph_height) as i32;

        glyph.pixels().for_each(|((x, y), value)| {
            // let above_baseline = y - (glyph_height - self.height());
            let frame_index = ((screen_x + x as usize)
                + ((baseline - y_offset) as usize + y as usize) * buffer.width)
                * 4;

            if value && frame_index + 4 < buffer.len()
            // our current workaround for out of
            // bounds crashing
            {
                buffer.data[frame_index..(frame_index + 4)].copy_from_slice(&BLUE_LIGHT);
            }
        });

        self.glyph_width(character)
    }

    fn glyph_width(&self, character: char) -> usize {
        let glyph = self.font.glyphs().get(&character).unwrap();

        // special handling here, maybe just because our test bdf font is broken
        if character == ' ' {
            self.space_width
        } else {
            glyph.width() as usize
        }
    }

    fn width(&self) -> usize {
        14
    }

    fn height(&self) -> usize {
        self.line_height
    }

    fn fallback_character(&self) -> char {
        '?'
    }
}

impl Font for FontType {
    fn draw_character(
        &self,
        buffer: &mut Buffer,
        screen_x: usize,
        screen_y: usize,
        character: char,
    ) -> usize {
        match self {
            Self::Image(f) => f.draw_character(buffer, screen_x, screen_y, character),
            Self::Bdf(f) => f.draw_character(buffer, screen_x, screen_y, character),
        }
    }
    fn glyph_width(&self, character: char) -> usize {
        match self {
            Self::Image(f) => f.glyph_width(character),
            Self::Bdf(f) => f.glyph_width(character),
        }
    }
    fn width(&self) -> usize {
        match self {
            Self::Image(f) => f.width(),
            Self::Bdf(f) => f.width(),
        }
    }
    fn height(&self) -> usize {
        match self {
            Self::Image(f) => f.height(),
            Self::Bdf(f) => f.height(),
        }
    }
    fn fallback_character(&self) -> char {
        match self {
            Self::Image(f) => f.fallback_character(),
            Self::Bdf(f) => f.fallback_character(),
        }
    }
}

pub struct ImageFont {
    font_image: ImageBuffer<Luma<u8>, Vec<u8>>,
    character_width: usize,
    character_height: usize,
    first_char: u8,
    last_char: u8,
    characters_per_row: usize,
    x_image_spacing: usize,
    y_image_spacing: usize,
    y_image_margin: usize,
    x_image_margin: usize,
}

impl ImageFont {
    pub fn oldschool() -> Self {
        let font = include_bytes!("resources/domsson_oldschool.png");
        let font_image = image::load_from_memory(font)
            .expect("failed to parse test font")
            .into_luma8();

        Self {
            font_image,
            first_char: b' ',
            last_char: b'~',
            character_width: 5,
            character_height: 7,
            characters_per_row: 18,
            x_image_spacing: 2,
            y_image_spacing: 2,
            y_image_margin: 1,
            x_image_margin: 1,
        }
    }
    pub fn mono_5_8() -> Self {
        // let font = include_bytes!("resources/debug.png");
        let font = include_bytes!("resources/5x8mono.png");
        let font_image = image::load_from_memory(font)
            .expect("failed to parse test font")
            .into_luma8();

        Self {
            font_image,
            first_char: b'!',
            last_char: b'~',
            character_width: 4,
            character_height: 8,
            characters_per_row: (b'~' - b'!' + 1) as usize,
            x_image_spacing: 2,
            y_image_spacing: 1,
            y_image_margin: 1,
            x_image_margin: 1,
        }
    }
}

impl Font for ImageFont {
    fn glyph_width(&self, _: char) -> usize {
        self.character_width
    }
    fn width(&self) -> usize {
        self.character_width
    }
    fn height(&self) -> usize {
        self.character_height
    }
    fn fallback_character(&self) -> char {
        '?'
    }

    fn draw_character(
        &self,
        buffer: &mut Buffer,
        screen_x: usize,
        screen_y: usize,
        character: char,
    ) -> usize {
        let index = match character {
            ' ' => return self.character_width,
            c if self.first_char <= c as u8 && self.last_char >= c as u8 => c,
            _ => self.fallback_character(),
        } as u8
            - self.first_char;

        let x = index as usize % self.characters_per_row;
        let y = index as usize / self.characters_per_row;

        let (img_x, img_y) = (
            (x * (self.width() + self.x_image_spacing)),
            (y * (self.height() + self.y_image_spacing)),
        );

        for j in 0..self.height() {
            for i in 0..self.width() {
                let frame_index = ((screen_x + i) + (screen_y + j) * buffer.width) * 4;

                let font_pixel = self.font_image[(
                    img_x as u32 + i as u32 + self.x_image_margin as u32,
                    img_y as u32 + j as u32 + self.y_image_margin as u32,
                )];

                if font_pixel.0[0] > 0 && frame_index + 4 < buffer.len()
                // our current workaround for out of
                // bounds crashing
                {
                    buffer.data[frame_index..(frame_index + 4)].copy_from_slice(&BLUE_LIGHT);
                }
            }
        }
        self.character_width
    }
}

pub trait Font {
    fn draw_character(
        &self,
        buffer: &mut Buffer,
        screen_x: usize,
        screen_y: usize,
        character: char,
    ) -> usize;

    fn glyph_width(&self, character: char) -> usize;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn fallback_character(&self) -> char;
}
