use crate::layout::{CalculatedLayout, Layout, Padding};

pub trait Element {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout);
    // TODO: get rid of width/height
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn padding(&self) -> Padding;
    fn on_click(&self, _position: (u32, u32)) {}
    fn layout_parameters(&self) -> Layout {
        // TODO this was for quick compiling. Do we still want it long-term?
        Layout::default()
    }
}
