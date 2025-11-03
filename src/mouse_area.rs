use crate::{
    element::Element,
    layout::{CalculatedLayout, Layout},
    tree::Node,
    Input,
};

// TODO make generic so that user can define Message
pub struct MouseArea<Message> {
    /// Pressed on the most recent frame
    on_press: Option<Box<dyn Fn((usize, usize)) -> Message>>,
    whenever_down: Option<Box<dyn Fn((usize, usize)) -> Message>>,
    on_hover: Option<Box<dyn Fn((usize, usize)) -> Message>>,
    on_exit: Option<Box<dyn Fn() -> Message>>,
}

impl<Message> MouseArea<Message> {
    pub fn new() -> Self {
        Self {
            on_press: None,
            on_hover: None,
            on_exit: None,
            whenever_down: None,
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
    pub fn whenever_down<F>(mut self, m: F) -> Self
    where
        F: Fn((usize, usize)) -> Message + 'static,
    {
        self.whenever_down = Some(Box::new(m));
        self
    }

    // TODO allow unsetting a message?
    pub fn on_hover<F>(mut self, m: F) -> Self
    where
        F: Fn((usize, usize)) -> Message + 'static,
    {
        self.on_hover = Some(Box::new(m));
        self
    }

    // TODO allow unsetting a message?
    pub fn on_exit<F>(mut self, m: F) -> Self
    where
        F: Fn() -> Message + 'static,
    {
        self.on_exit = Some(Box::new(m));
        self
    }
}

impl<Message> Element<Message> for MouseArea<Message> {
    fn draw(&self, _frame: &mut [u8], _region: CalculatedLayout) {}

    fn get_message(&mut self, input: &Input, region: CalculatedLayout) -> Option<Message> {
        if let Some(mouse_pos) = input.mouse_pos {
            if region.contains(mouse_pos) {
                if input.mouse_released {
                    if let Some(on_press) = &self.on_press {
                        return Some((on_press)((
                            (mouse_pos.0 - region.x) as usize,
                            (mouse_pos.1 - region.y) as usize,
                        )));
                    }
                }
                if let Some(whenever_down) = &self.whenever_down {
                    if input.mouse_down {
                        return Some((whenever_down)((
                            (mouse_pos.0 - region.x) as usize,
                            (mouse_pos.1 - region.y) as usize,
                        )));
                    }
                }
                if let Some(on_hover) = &self.on_hover {
                    return Some((on_hover)((
                        (mouse_pos.0 - region.x) as usize,
                        (mouse_pos.1 - region.y) as usize,
                    )));
                }
            } else {
                let previously_in_bounds = match input.prev_mouse_pos {
                    Some(pos) => region.contains(pos),
                    None => false,
                };
                if previously_in_bounds {
                    if let Some(on_exit) = &self.on_exit {
                        return Some((on_exit)());
                    }
                }
            }
        }
        return None;
    }
}

impl<Message: 'static> Into<Node<Message, Layout>> for MouseArea<Message> {
    fn into(self) -> Node<Message, Layout> {
        Node::new(self)
    }
}
