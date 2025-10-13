use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::palette::{BLUE_DARK, MAIN_LIGHT};
use icecube::quad::Quad;
use icecube::tree::Node;

#[derive(Debug, Copy, Clone)]
pub enum Message {}

struct State {
    data: [usize; 12],
}

impl Default for State {
    fn default() -> Self {
        Self {
            data: [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        }
    }
}

fn update(_m: Message, _state: &mut State) {}

fn view(state: &State) -> Node<Message, Layout> {
    //TODO: width height here, but height width in padding
    let mut root = Node::root_node(320, 240).row();

    // This fills the screen, causing the screen to clear each frame
    let mut container = Node::new(Quad::new().fill(MAIN_LIGHT))
        .column()
        .padding([100, 140]);

    let image = Node::new(Image::new(state.data.into(), 3, 4).scale_factor(1))
        .height(Length::Shrink)
        .width(Length::Shrink);

    container.push(Node::new(Quad::new().fill(BLUE_DARK)).height(Length::Fixed(50)));
    container.push(image);
    container.push(Node::new(Quad::new().fill(BLUE_DARK)).height(Length::Fixed(50)));
    root.push(container);
    root
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT)
}
