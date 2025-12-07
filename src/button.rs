use crate::{element::Element, layout::CalculatedLayout, state_tree::StateNode, Input};

// TODO make generic so that user can define Message
pub struct Button<Message> {
    /// Pressed on the most recent frame
    on_press: Option<Message>,
    on_hover: Option<Message>,
    whenever_down: Option<Message>,
}

impl<Message> Button<Message> {
    pub fn new() -> Self {
        Self {
            on_press: None,
            on_hover: None,
            whenever_down: None,
        }
    }

    // TODO allow unsetting a message?
    pub fn on_press(mut self, m: Message) -> Self {
        self.on_press = Some(m);
        self
    }

    // TODO allow unsetting a message?
    pub fn on_hover(mut self, m: Message) -> Self {
        self.on_hover = Some(m);
        self
    }

    pub fn whenever_down(mut self, m: Message) -> Self {
        self.whenever_down = Some(m);
        self
    }
}

impl<Message: Clone> Element<Message> for Button<Message> {
    fn draw(&self, _frame: &mut [u8], _region: CalculatedLayout) {}
    fn get_message(
        &mut self,
        _tree: &mut StateNode,
        input: &Input,
        region: CalculatedLayout,
    ) -> Option<Message> {
        if let Some(mouse_pos) = input.mouse_pos {
            if region.contains(mouse_pos) {
                if input.mouse_released {
                    return self.on_press.clone();
                };
                if input.mouse_down {
                    return self.whenever_down.clone();
                }
            }
        }
        return None;
    }
}
