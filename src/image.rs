#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::element::Element;
use crate::layout::CalculatedLayout;
use crate::palette::color_from_index;

const WIDTH: u32 = 320; // TODO make this metadata for the frame buffer

#[derive(Clone)]
pub struct Image<'a> {
    pub data: &'a [usize],
    pub scale_factor: usize,
    /// image width before scaling
    pub width: usize,
    /// image height before scaling
    pub height: usize,
}

impl<'a> Image<'a> {
    pub fn new(data: &'a [usize], width: usize, height: usize) -> Image<'a> {
        Self {
            data,
            width,
            height,
            scale_factor: 1,
        }
    }
    pub fn scale_factor(mut self, scale_factor: usize) -> Self {
        assert!(scale_factor != 0);
        self.scale_factor = scale_factor;
        self
    }
}

impl<'a, Message> Element<'a, Message> for Image<'a> {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
        for j in 0..self.height {
            for i in 0..self.width {
                let frame_index = ((region.x as usize + i * self.scale_factor)
                    + (region.y as usize + j * self.scale_factor) * WIDTH as usize)
                    * 4;

                let pixel_index = self.data[i + j * self.width];
                let pixel = color_from_index(pixel_index);
                if frame_index + 4 < frame.len() {
                    // our current workaround for out of
                    // bounds crashing
                    match self.scale_factor {
                        1 => {
                            //TODO: look up actual pixel value
                            frame[frame_index..(frame_index + 4)].copy_from_slice(&pixel);
                        }
                        _ => {
                            for sj in 0..self.scale_factor {
                                for si in 0..self.scale_factor {
                                    let scaled_frame_index =
                                        frame_index + (si + sj * WIDTH as usize) * 4;
                                    frame[scaled_frame_index..(scaled_frame_index + 4)]
                                        .copy_from_slice(&pixel);
                                }
                            }
                        }
                    };
                }
            }
        }
    }

    fn min_width(&self) -> u32 {
        (self.width * self.scale_factor) as u32
    }

    fn min_height(&self, _width: u32) -> u32 {
        (self.height * self.scale_factor) as u32
    }

    fn get_message(
        &self,
        _input: &crate::button::Input,
        _region: CalculatedLayout,
    ) -> Option<Message> {
        None
    }
}
