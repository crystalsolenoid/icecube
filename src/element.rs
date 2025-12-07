use crate::{
    layout::{CalculatedLayout, Layout},
    state_tree::{self, StateNode},
};

pub trait Element<Message> {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout);
    fn get_message(
        &mut self,
        tree: &mut StateNode,
        input: &crate::Input,
        region: CalculatedLayout,
    ) -> Option<Message>;
    fn layout_parameters(&self) -> Layout {
        // TODO this was for quick compiling. Do we still want it long-term?
        Layout::default()
    }
    //TODO: Dead code
    /// Wrappable elements (maybe only text?) will return the height when wrapped at the given
    /// width.
    fn wrap(&self, _width: u32) -> Option<u32> {
        None
    }
    fn min_width(&self) -> u32 {
        0
    }
    fn min_height(&self, width: u32) -> u32 {
        self.wrap(width).unwrap_or_default()
    }
    fn get_initial_state(&self) -> state_tree::State {
        state_tree::State::None
    }
}
