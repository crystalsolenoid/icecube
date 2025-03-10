use crate::element::{Element, Padding};
use crate::palette::BLUE_LIGHT;

const WIDTH: u32 = 320; // TODO make this metadata for the frame buffer
                        //
#[derive(Clone)]
pub struct Text {
    pub content: String,
}

impl Element for Text {
    fn width(&self) -> u32 {
        100
    }
    fn height(&self) -> u32 {
        10
    }
    fn draw(&self, frame: &mut [u8], position: (u32, u32)) {
        let font = include_bytes!("resources/test_font.png");
        let font = image::load_from_memory(font)
            .expect("failed to parse test font")
            .into_luma8();

        self.content
            .as_bytes()
            //.expect("couldn't render non-ascii text")
            .iter()
            .map(|character| {
                let index = character - b' ';
                let x = index % 18;
                let y = index / 18;

                ((x * 7), (y * 9))
            })
            .enumerate()
            .for_each(|(character_index, (x, y))| {
                let (frame_x, frame_y) = position;

                for j in 0..=7 {
                    for i in 0..=5 {
                        let frame_index = 4
                            * ((frame_y + j) * WIDTH + frame_x + i + (character_index as u32 * 6))
                                as usize;

                        let font_pixel = font[(x as u32 + i, y as u32 + j)];

                        if font_pixel.0[0] > 0 {
                            frame[frame_index..(frame_index + 4)].copy_from_slice(&BLUE_LIGHT);
                        }
                    }
                }
            });
    }
    fn padding(&self) -> Padding {
        Padding::default()
    }
}
