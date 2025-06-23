use crate::layout::{CalculatedLayout, Layout};

pub trait Element {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout);
    fn on_click(&self, _position: (u32, u32)) {}
    fn layout_parameters(&self) -> Layout {
        // TODO this was for quick compiling. Do we still want it long-term?
        Layout::default()
    }
    /// Wrappable elements (maybe only text?) will return the height when wrapped at the given
    /// width.
    fn wrap(&self, _width: u32) -> Option<u32> {
        None
    }
}
