use crate::{element::Element, layout::CalculatedLayout};

// TODO user-defined
#[derive(Debug, Copy, Clone)]
pub enum Message {
    ButtonClick,
    ButtonHover,
}

/// Holds all of the current frame's input state
#[derive(Debug)]
pub struct Input {
    pub mouse_released: bool,
    pub mouse_pos: Option<(u32, u32)>,
}

// TODO make generic so that user can define Message
pub struct Button {
    /// Pressed on the most recent frame
    on_press: Option<Message>,
    on_hover: Option<Message>,
}

impl Button {
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

impl Element for Button {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {}
    fn get_message(
        &self,
        input: &Input,
        region: CalculatedLayout,
    ) -> Option<crate::button::Message> {
        if let Some(mouse_pos) = input.mouse_pos {
            if input.mouse_released && region.contains(mouse_pos) {
                self.on_press
            } else {
                None
            }
        } else {
            None
        }
    }
}
