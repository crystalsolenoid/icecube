#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::element::Element;
use crate::layout::CalculatedLayout;
use crate::palette::color_from_index;

const WIDTH: u32 = 320; // TODO make this metadata for the frame buffer

#[derive(Clone)]
pub struct Image<'a> {
    pub data: &'a [usize],
    pub width: usize,
    pub height: usize,
}

impl<'a> Image<'a> {
    pub fn new(data: &'a [usize], width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}

impl<'a, Message> Element<Message> for Image<'a> {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
        for j in 0..self.height {
            for i in 0..self.width {
                let frame_index =
                    ((region.x as usize + i) + (region.y as usize + j) * WIDTH as usize) * 4;

                let pixel_index = self.data[i + j * self.width];
                let pixel = color_from_index(pixel_index);
                if frame_index + 4 < frame.len()
                // our current workaround for out of
                // bounds crashing
                {
                    //TODO: look up actual pixel value
                    frame[frame_index..(frame_index + 4)].copy_from_slice(&pixel);
                }
            }
        }
    }

    fn get_message(
        &self,
        _input: &crate::button::Input,
        _region: CalculatedLayout,
    ) -> Option<Message> {
        None
    }
}
