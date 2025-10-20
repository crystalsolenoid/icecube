use icecube::button::Button;
use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::palette::MAIN_LIGHT;
use icecube::quad::Quad;
use icecube::tree::Node;

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

fn view(state: &State) -> Node<Message, Layout> {
    //TODO: width height here, but height width in padding
    let mut root = Node::root_node(320, 240).row();

    // This fills the screen, causing the screen to clear each frame
    let mut container = Node::new(Quad::new().fill(MAIN_LIGHT))
        .width(Length::Grow)
        .height(Length::Grow)
        .column();
    let mut row = Node::new(Quad::new())
        .row()
        .width(Length::Grow)
        .height(Length::Shrink);

    let image = Node::new(Image::new(state.data.clone(), 3, 4).scale_factor(8))
        .height(Length::Shrink)
        .width(Length::Shrink);

    container.push(Node::spacer());

    let mut button = Node::new(Button::new().on_press(Message::Invert))
        .height(Length::Shrink)
        .width(Length::Shrink);
    button.push(image);

    row.push(Node::spacer());
    row.push(button);
    row.push(Node::spacer());
    container.push(row);

    container.push(Node::spacer());
    root.push(container);
    root
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT)
}
