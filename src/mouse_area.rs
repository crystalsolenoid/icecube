use crate::{
    button::Input,
    element::Element,
    layout::{CalculatedLayout, Layout},
    tree::Node,
};

// TODO make generic so that user can define Message
pub struct MouseArea<Message> {
    /// Pressed on the most recent frame
    on_press: Option<Box<dyn Fn((usize, usize)) -> Message>>,
    on_hover: Option<Message>,
}

impl<Message> MouseArea<Message> {
    pub fn new() -> Self {
        Self {
            on_press: None,
            on_hover: None,
        }
    }

    // TODO allow unsetting a message?
    pub fn on_press<F>(mut self, m: F) -> Self
    where
        F: Fn((usize, usize)) -> Message + 'static,
    {
        self.on_press = Some(Box::new(m));
        self
    }

    // TODO allow unsetting a message?
    pub fn on_hover(mut self, m: Message) -> Self {
        self.on_hover = Some(m);
        self
    }
}

impl<Message> Element<Message> for MouseArea<Message> {
    fn draw(&self, _frame: &mut [u8], _region: CalculatedLayout) {}

    fn get_message(&self, input: &Input, region: CalculatedLayout) -> Option<Message> {
        if let Some(mouse_pos) = input.mouse_pos {
            if input.mouse_released && region.contains(mouse_pos) {
                if let Some(on_press) = &self.on_press {
                    Some((on_press)((mouse_pos.0 as usize, mouse_pos.1 as usize)))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<Message: 'static> Into<Node<Message, Layout>> for MouseArea<Message> {
    fn into(self) -> Node<Message, Layout> {
        Node::new(self)
    }
}
