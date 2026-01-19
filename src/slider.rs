use crate::{
    constants::WIDTH,
    element::Element,
    layout::{CalculatedLayout, Layout},
    palette::{Color, BLUE_DARK, BLUE_LIGHT, MAIN_DARK, MAIN_LIGHT, RED_DARK, RED_LIGHT},
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
    active_bar: Color,
    inactive_bar: Color,
    handle: Color,
    // granularity...
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
            active_bar: RED_DARK,
            inactive_bar: RED_LIGHT,
            handle: MAIN_DARK,
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

    pub fn set_color(mut self, active: Color, inactive: Color, handle: Color) -> Self {
        self.active_bar = active;
        self.inactive_bar = inactive;
        self.handle = handle;
        self
    }
}

impl<Message> Element<Message> for Slider<Message> {
    fn draw(&self, frame: &mut [u8], region: CalculatedLayout) {
        let percent = (self.value - self.range.start) / (self.range.end - self.range.start);

        let bar_y_start = region.y + region.h / 2 - 1;

        let frame_index = |i: i32, j: i32| {
            ((((region.x as i32) + i) as u32 + (((bar_y_start as i32) + j) as u32) * WIDTH) * 4)
                as usize
        };
        for j in 0..2 {
            for i in 0..region.w {
                let pixel = if (i as f32 / region.w as f32) < percent {
                    self.active_bar
                } else {
                    self.inactive_bar
                };
                let index = frame_index(i as i32, j as i32);
                frame[index..(index + 4)].copy_from_slice(&pixel);
            }
        }

        let handle_i = (percent * region.w as f32).round() as i32;
        let handle_indexes = [
            frame_index(handle_i + 1, -1),
            frame_index(handle_i, 0),
            frame_index(handle_i, 1),
            frame_index(handle_i - 1, 2),
        ];
        handle_indexes
            .into_iter()
            .for_each(|i| frame[i..(i + 4)].copy_from_slice(&self.handle));
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

impl<'a, Message: 'static> Into<Node<'a, Message, Layout>> for Slider<Message> {
    fn into(self) -> Node<'a, Message, Layout> {
        Node::new(self)
    }
}
