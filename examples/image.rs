use icecube::button::Button;
use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::palette::MAIN_LIGHT;
use icecube::tree::Node;
use icecube::{col, row};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Invert,
}

struct State {
    data: Vec<usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            data: vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        }
    }
}

impl State {
    fn invert(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|px| match px {
                0 => 1,
                1 => 0,
                _ => 0,
            })
            .collect();
    }
}

fn update(m: Message, state: &mut State) {
    match m {
        Message::Invert => state.invert(),
    }
}

fn view<'a>(state: &State) -> Node<'a, Message, Layout> {
    let image = Node::new(Image::new(state.data.clone(), 3, 4).scale_factor(8))
        .height(Length::Shrink)
        .width(Length::Shrink);

    let mut button = Node::new(Button::new().on_press(Message::Invert))
        .height(Length::Shrink)
        .width(Length::Shrink);
    button.push(image);

    row![
        Node::spacer(),
        col![Node::spacer(), button, Node::spacer()],
        Node::spacer()
    ]
    .height(Length::Grow)
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT, |_| None)
}
