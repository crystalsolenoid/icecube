use crate::layout::{LayoutParameters, Padding};

pub trait Element {
    fn draw(&self, frame: &mut [u8], position: (u32, u32));
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn padding(&self) -> Padding;
    fn on_click(&self, _position: (u32, u32)) {}
    fn layout_parameters(&self) -> LayoutParameters {
        // TODO this was for quick compiling. Do we still want it long-term?
        LayoutParameters::default()
    }
}
