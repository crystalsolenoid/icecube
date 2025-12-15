use crate::{
    constants::WIDTH,
    element::Element,
    layout::{CalculatedLayout, Layout},
    palette::{Color, BLUE_DARK, BLUE_LIGHT},
    state_tree::{self, StateNode},
    tree::Node,
    Input,
};

// TODO make generic so that user can define Message
pub struct Slider<Message> {
    /// Pressed on the most recent frame
    on_drag: Option<Box<dyn Fn(f32) -> Message>>,
    value: f32,
    // 0.0..1.0
    range: std::ops::Range<f32>,
    primary_color: Color,
    secondary_color: Color, // granularity...
                            // on_finish_drag...
}

pub struct State {
    is_dragging: bool,
}

impl<Message> Slider<Message> {
    pub fn new(range: std::ops::Range<f32>, value: f32) -> Self {
        Self {
            on_drag: None,
            value,
            range,
            primary_color: BLUE_LIGHT,
            secondary_color: BLUE_DARK,
        }
    }

    // TODO allow unsetting a message?
    pub fn on_drag<F>(mut self, m: F) -> Self
    where
        F: Fn(f32) -> Message + 'static,
    {
        self.on_drag = Some(Box::new(m));
        self
    }

    pub fn set_color(mut self, primary: Color, secondary: Color) -> Self {
        self.primary_color = primary;
        self.secondary_color = secondary;
        self
    }
}

impl<Message> Element<Message> for Slider<Message> {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
        let percent = (self.value - self.range.start) / (self.range.end - self.range.start);

        for j in 0..region.h {
            for i in 0..region.w {
                let frame_index = (((region.x + i) + (region.y + j) * WIDTH) * 4) as usize;

                let pixel = if (i as f32 / region.w as f32) < percent {
                    self.primary_color
                } else {
                    self.secondary_color
                };
                frame[frame_index..(frame_index + 4)].copy_from_slice(&pixel);
            }
        }
    }

    fn get_message(
        &mut self,
        tree: &mut StateNode,
        input: &Input,
        region: CalculatedLayout,
    ) -> Option<Message> {
        let state = tree.state.downcast_mut::<State>();

        if let Some(mouse_pos) = input.mouse_pos {
            if region.contains(mouse_pos) && input.mouse_down {
                state.is_dragging = true;
            }
            if input.mouse_down && state.is_dragging {
                if let Some(on_drag) = &self.on_drag {
                    let percent = mouse_pos.0.saturating_sub(region.x) as f32 / region.w as f32;
                    let new_value = ((self.range.end - self.range.start) * percent
                        + self.range.start)
                        .clamp(self.range.start, self.range.end);
                    self.value = new_value;
                    return Some((on_drag)(new_value));
                }
            } else {
                state.is_dragging = false;
            }
        }
        return None;
    }

    fn get_initial_state(&self) -> state_tree::State {
        state_tree::State::new(State { is_dragging: false })
    }
}

impl<Message: 'static> Into<Node<Message, Layout>> for Slider<Message> {
    fn into(self) -> Node<Message, Layout> {
        Node::new(self)
    }
}
