#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::constants::WIDTH;
use crate::element::Element;
use crate::layout::CalculatedLayout;
use crate::palette::color_from_index;
use crate::state_tree::StateNode;

#[derive(Clone)]
pub struct Image<T> {
    pub data: Vec<T>,
    pub scale_factor: usize,
    /// image width before scaling
    pub width: usize,
    /// image height before scaling
    pub height: usize,
}

impl<T> Image<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self {
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

trait PixelColor {
    fn get_pixel_color(&self) -> [u8; 4];
}

impl PixelColor for [u8; 4] {
    fn get_pixel_color(&self) -> [u8; 4] {
        *self
    }
}

impl PixelColor for usize {
    fn get_pixel_color(&self) -> [u8; 4] {
        color_from_index(*self)
    }
}

impl<Message, T: PixelColor + Clone> Element<Message> for Image<T> {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
        for j in 0..self.height {
            for i in 0..self.width {
                let frame_index = ((region.x as usize + i * self.scale_factor)
                    + (region.y as usize + j * self.scale_factor) * WIDTH as usize)
                    * 4;

                let pixel = self.data[i + j * self.width].get_pixel_color();
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
        &mut self,
        _tree: &mut StateNode,
        _input: &crate::Input,
        _region: CalculatedLayout,
    ) -> Option<Message> {
        None
    }
}
