use crate::layout::{CalculatedLayout, Layout};

pub trait Element<Message> {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout);
    fn get_message(
        &self,
        input: &crate::button::Input,
        region: CalculatedLayout,
    ) -> Option<Message>;
    fn layout_parameters(&self) -> Layout {
        // TODO this was for quick compiling. Do we still want it long-term?
        Layout::default()
    }
    /// Wrappable elements (maybe only text?) will return the height when wrapped at the given
    /// width.
    fn wrap(&self, _width: u32) -> Option<u32> {
        None
    }
    fn min_width(&self) -> u32 {
        0
    }
}
