use crate::element::Element;
use crate::layout::CalculatedLayout;
use crate::palette::BLUE_LIGHT;

const WIDTH: u32 = 320; // TODO make this metadata for the frame buffer
                        //
#[derive(Clone)]
pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Element for Text {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
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
                let (frame_x, frame_y) = (region.x, region.y);
                let linear_progress = character_index as u32 * 6;
                let usable_width = region.w.next_multiple_of(6) - 6;

                let line_number = linear_progress / usable_width;
                let column_number = linear_progress % usable_width;

                let char_x = frame_x + column_number;
                let char_y = frame_y + line_number * (8 + 1);

                for j in 0..=7 {
                    for i in 0..=5 {
                        let frame_index = 4 * ((char_y + j) * WIDTH + i + char_x) as usize;

                        let font_pixel = font[(x as u32 + i, y as u32 + j)];

                        if font_pixel.0[0] > 0 {
                            frame[frame_index..(frame_index + 4)].copy_from_slice(&BLUE_LIGHT);
                        }
                    }
                }
            });
    }
}
