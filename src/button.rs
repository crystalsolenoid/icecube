use crate::{element::Element, layout::Padding, quad::Quad, text::Text};

pub struct Button {
    pub text: Text,
    pub quad: Quad,
}

impl Element for Button {
    fn draw(&self, frame: &mut [u8], position: (u32, u32)) {
        self.quad.draw(frame, position);
        self.text.draw(
            frame,
            (
                position.0 + self.quad.padding().left,
                position.1 + self.quad.padding().top,
            ),
        );
    }

    fn width(&self) -> u32 {
        self.quad.width()
    }

    fn height(&self) -> u32 {
        self.quad.height()
    }

    fn padding(&self) -> Padding {
        self.quad.padding()
    }
    fn on_click(&self, _position: (u32, u32)) {
        println!("{}", self.text.content)
    }
}
