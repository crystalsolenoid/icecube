use icecube::image::Image;
use icecube::layout::Layout;
use icecube::palette::MAIN_LIGHT;
use icecube::quad::Quad;
use icecube::tree::Node;

#[derive(Debug, Copy, Clone)]
pub enum Message {}

#[derive(Default)]
struct State {}

const DATA: [usize; 12] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];

fn update(_m: Message, _state: &mut State) {}

fn view(_state: &State) -> Node<Message, Layout> {
    //TODO: width height here, but height width in padding
    let mut root = Node::root_node(320, 240).row();

    // This fills the screen, causing the screen to clear each frame
    let mut container = Node::new(Quad::new().fill(MAIN_LIGHT))
        .column()
        .padding([110, 150]);

    let image = Node::new(Image::new(&DATA, 3, 4));

    container.push(image);
    root.push(container);
    root
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT)
}
