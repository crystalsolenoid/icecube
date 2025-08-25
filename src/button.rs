use crate::{element::Element, layout::CalculatedLayout};

/// Holds all of the current frame's input state
#[derive(Debug)]
pub struct Input {
    pub mouse_released: bool,
    pub mouse_pos: Option<(u32, u32)>,
}

// TODO make generic so that user can define Message
pub struct Button<Message> {
    /// Pressed on the most recent frame
    on_press: Option<Message>,
    on_hover: Option<Message>,
}

impl<Message> Button<Message> {
    pub fn new() -> Self {
        Self {
            on_press: None,
            on_hover: None,
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
}

impl<Message: Clone> Element<Message> for Button<Message> {
    fn draw(&self, _frame: &mut [u8], _region: CalculatedLayout) {}
    fn get_message(&self, input: &Input, region: CalculatedLayout) -> Option<Message> {
        if let Some(mouse_pos) = input.mouse_pos {
            if input.mouse_released && region.contains(mouse_pos) {
                self.on_press.clone()
            } else {
                None
            }
        } else {
            None
        }
    }
}
