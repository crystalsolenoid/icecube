use crate::{element::Element, layout::CalculatedLayout, quad::Quad, text::Text};

pub struct Button {}

impl Button {
    pub fn new() -> Self {
        Self {}
    }
}

impl Element for Button {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {}

    fn on_click(&self, _position: (u32, u32)) {
        println!("button press")
    }
}
