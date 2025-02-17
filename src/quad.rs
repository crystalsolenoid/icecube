#![deny(clippy::all)]
#![forbid(unsafe_code)]

/*
* our_quad.fill(red).border_color(blue)
*/

use crate::palette::Color;

const WIDTH: u32 = 320; // TODO make this metadata for the frame buffer

#[derive(Clone)]
pub struct QuadStyle {
    pub fill_style: Option<Color>,
    pub border_style: Option<BorderStyle>,
}

#[derive(Clone)]
pub struct BorderStyle {
    pub color: Color,
    pub thickness: usize,
}

#[derive(Clone)]
pub struct Quad {
    pub width: usize,
    pub height: usize,
    pub style: QuadStyle,
}

impl Quad {
    pub fn draw(&self, frame: &mut [u8], position: (u32, u32)) {
        let position = (position.0 as usize, position.1 as usize); // TODO fix types mess
                                                                   //TODO: Consider optimizing this if it is a bottleneck
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH as usize;
            let y = i / WIDTH as usize;

            let inside_the_box = x >= position.0
                && x < position.0 + self.width
                && y >= position.1
                && y < position.1 + self.height;

            let rgba = if inside_the_box {
                match &self.style.border_style {
                    Some(border_style) => {
                        let border_thickness = border_style.thickness;
                        if x < position.0 + border_thickness
                            || x >= position.0 + self.width - border_thickness
                            || y < position.1 + border_thickness
                            || y >= position.1 + self.height - border_thickness
                        {
                            Some(border_style.color)
                        } else {
                            self.style.fill_style
                        }
                    }
                    None => self.style.fill_style,
                }
            } else {
                None
            };

            if let Some(color) = rgba {
                pixel.copy_from_slice(&color);
            }
        }
    }
}
