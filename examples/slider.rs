use icecube::font::{self};
use icecube::layout::{Layout, Length};
use icecube::palette::MAIN_LIGHT;
use icecube::slider::Slider;
use icecube::text::Text;
use icecube::tree::Node;
use icecube::{col, row};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Slide(f32),
}

#[derive(Default)]
struct State {
    value: f32,
}

fn update(m: Message, state: &mut State) {
    match m {
        Message::Slide(val) => state.value = val,
    }
}

fn view<'a>(state: &State) -> Node<'a, Message, Layout> {
    let font = &font::BLACKLETTER;

    let mut count =
        Node::new(Text::new(format!("{:.1}", state.value)).with_font(font)).width(Length::Shrink);
    count.name = Some("counter value".to_string());

    let count_row = row![Node::spacer(), count, Node::spacer()];

    let slider = Slider::new(0.0..10.0, state.value).on_drag(Message::Slide);

    let slider_row = row![
        Node::spacer(),
        Node::new(slider).width(50).height(10),
        Node::spacer(),
    ]
    .width(Length::Shrink)
    .padding(5);

    row![
        Node::spacer(),
        col![Node::spacer(), count_row, slider_row, Node::spacer()],
        Node::spacer(),
    ]
    .height(Length::Grow)
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT, |_| None)
}
