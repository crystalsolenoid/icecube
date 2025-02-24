use crate::{quad::Quad, text::Text};

#[derive(Clone)]
pub enum Element {
    Quad(Quad),
    Text(Text),
}

impl Element {
    pub fn draw(&self, frame: &mut [u8], position: (u32, u32)) {
        match self {
            Element::Quad(quad) => quad.draw(frame, position),
            Element::Text(text) => text.draw(frame, position),
        }
    }
    pub fn width(&self) -> u32 {
        match self {
            Element::Quad(quad) => quad.width as u32,
            Element::Text(text) => text.width(),
        }
    }
    pub fn height(&self) -> u32 {
        match self {
            Element::Quad(quad) => quad.height as u32,
            Element::Text(text) => text.height(),
        }
    }
}
