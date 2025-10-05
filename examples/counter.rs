use icecube::button::Button;
use icecube::font::{self};
use icecube::layout::{Layout, Length};
use icecube::palette::{BLUE_DARK, BLUE_LIGHT, MAIN_LIGHT, RED_DARK, RED_LIGHT};
use icecube::quad::Quad;
use icecube::text::Text;
use icecube::tree::Node;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct State {
    count: i32,
}

fn update(m: Message, state: &mut State) {
    match m {
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
    }
}

fn view(state: &State) -> Node<Message, Layout> {
    let mut root = Node::root_node(320, 240).row();

    // This fills the screen, causing the screen to clear each frame
    let mut container = Node::new(Quad::new().fill(MAIN_LIGHT)).column();

    let font = &font::BLACKLETTER;

    let mut count_row = Node::new(Quad::new().border_color(RED_DARK).border_thickness(1))
        .row()
        .height(Length::Shrink);
    let mut count =
        Node::new(Text::new(format!("{}", state.count)).with_font(font)).width(Length::Shrink);
    count.name = Some("counter value".to_string());

    count_row.push(Node::new(
        Quad::new().border_color(RED_LIGHT).border_thickness(1),
    ));
    count_row.push(count);
    count_row.push(Node::new(
        Quad::new().border_color(RED_LIGHT).border_thickness(1),
    ));

    let increment = make_button("+".into(), Message::Increment);
    let decrement = make_button("-".into(), Message::Decrement);

    let mut button_row = Node::new(Quad::new().border_color(RED_DARK).border_thickness(1)).row();
    button_row.push(Node::new(Quad::new()));
    button_row.push(increment);
    button_row.push(Node::new(Quad::new()).width(Length::Fixed(2)));
    button_row.push(decrement);
    button_row.push(Node::new(Quad::new()));

    container.push(Node::new(Quad::new()));
    container.push(count_row);
    container.push(button_row);
    container.push(Node::new(Quad::new()));

    root.push(Node::new(Quad::new()));
    root.push(container);
    root.push(Node::new(Quad::new()));
    root
}

//TODO: Make an alias for LazyLock<FontType> and for Node
fn make_button(label: String, action: Message) -> Node<Message, Layout> {
    let button_text = Node::new(Text::new(label).with_font(&font::BLACKLETTER));
    let mut button_quad = Node::new(
        Quad::new()
            .fill(BLUE_DARK)
            .border_thickness(1)
            .border_color(BLUE_LIGHT),
    )
    .width(Length::Grow) // TODO when this is shrink, the text has zero room. Why?
    .height(Length::Shrink)
    .padding(2)
    .row();
    button_quad.push(button_text);
    let mut button_node = Node::new(Button::new().on_press(action))
        .width(Length::Fixed(20))
        .height(Length::Shrink);
    button_node.push(button_quad);
    button_node
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT)
}
