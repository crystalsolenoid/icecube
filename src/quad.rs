#![deny(clippy::all)]
#![forbid(unsafe_code)]

/*
* our_quad.fill(red).border_color(blue)
*/

use crate::element::Element;
use crate::layout::CalculatedLayout;
use crate::palette::Color;

const WIDTH: u32 = 320; // TODO make this metadata for the frame buffer

#[derive(Clone, Default)]
pub struct QuadStyle {
    pub fill_style: Option<Color>,
    pub border_style: Option<BorderStyle>,
}

#[derive(Clone, Copy, Default)]
pub struct BorderStyle {
    pub color: Color,
    pub thickness: u32,
}

#[derive(Clone)]
pub struct Quad {
    pub style: QuadStyle,
}

impl Default for Quad {
    fn default() -> Self {
        Self::new()
    }
}

impl Quad {
    pub fn new() -> Self {
        Self {
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

    pub fn border_thickness(mut self, thickness: u32) -> Self {
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
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
        let position = (region.x, region.y); // TODO fix types mess
                                             //TODO: Consider optimizing this if it is a bottleneck
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i as u32 % WIDTH;
            let y = i as u32 / WIDTH;

            let inside_the_box = x >= position.0
                && x < position.0 + region.w
                && y >= position.1
                && y < position.1 + region.h;

            let rgba = if inside_the_box {
                match &self.style.border_style {
                    Some(border_style) => {
                        let border_thickness = border_style.thickness;
                        if x < position.0 + border_thickness
                            || x >= position.0 + region.w - border_thickness
                            || y < position.1 + border_thickness
                            || y >= position.1 + region.h - border_thickness
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

    fn get_message(
        &self,
        _input: &crate::button::Input,
        _region: CalculatedLayout,
    ) -> Option<crate::button::Message> {
        None
    }
}
