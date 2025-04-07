use crate::{element::Element, layout::CalculatedLayout, quad::Quad, text::Text};

pub struct Button {
    pub text: Text,
    pub quad: Quad,
}

impl Element for Button {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
        self.quad.draw(frame, region);
        self.text.draw(
            frame,
            region, //(
                   //    position.0 + self.quad.padding().left,
                   //    position.1 + self.quad.padding().top,
                   //),
        );
    }

    fn on_click(&self, _position: (u32, u32)) {
        println!("{}", self.text.content)
    }
}
