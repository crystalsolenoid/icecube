#![deny(clippy::all)]
#![forbid(unsafe_code)]

/*
* our_quad.fill(red).border_color(blue)
*/

use crate::element::{Element, Padding};
use crate::palette::Color;

const WIDTH: u32 = 320; // TODO make this metadata for the frame buffer

#[derive(Clone, Default)]
pub struct QuadStyle {
    pub fill_style: Option<Color>,
    pub border_style: Option<BorderStyle>,
}

#[derive(Clone, Default)]
pub struct BorderStyle {
    pub color: Color,
    pub thickness: usize,
}

#[derive(Clone)]
pub struct Quad {
    pub width: usize,
    pub height: usize,
    pub padding: Padding,
    pub style: QuadStyle,
}

impl Quad {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as usize,
            height: height as usize,
            padding: Padding::default(),
            style: QuadStyle::default(),
        }
    }

    pub fn fill(mut self, color: Color) -> Self {
        self.style.fill_style = Some(color);
        self
    }

    pub fn style(mut self, style: QuadStyle) -> Self {
        self.style = style;
        self
    }
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        match self.style.border_style {
            Some(ref mut style) => {
                style.color = color;
            }
            None => {
                self.style.border_style = Some(BorderStyle {
                    color,
                    ..BorderStyle::default()
                });
            }
        };
        self
    }

    pub fn border_thickness(mut self, thickness: usize) -> Self {
        match self.style.border_style {
            Some(ref mut style) => {
                style.thickness = thickness;
            }
            None => {
                self.style.border_style = Some(BorderStyle {
                    thickness,
                    ..BorderStyle::default()
                });
            }
        };
        self
    }
}

impl Element for Quad {
    fn draw(&self, frame: &mut [u8], position: (u32, u32)) {
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
    fn width(&self) -> u32 {
        self.width as u32
    }
    fn height(&self) -> u32 {
        self.height as u32
    }
    fn padding(&self) -> Padding {
        self.padding
    }
}
