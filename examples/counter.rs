use icecube::button::Button;
use icecube::font::{self};
use icecube::layout::{Layout, Length};
use icecube::palette::{BLUE_DARK, BLUE_LIGHT, MAIN_LIGHT};
use icecube::quad::Quad;
use icecube::text::Text;
use icecube::tree::Node;
use icecube::widget::{column, row};

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
    let mut root = Node::root_node(320, 240, MAIN_LIGHT).row();

    let font = &font::BLACKLETTER;

    let mut count =
        Node::new(Text::new(format!("{}", state.count)).with_font(font)).width(Length::Shrink);
    count.name = Some("counter value".to_string());

    let count_row = row(vec![Node::spacer(), count, Node::spacer()]);

    let increment = make_button("+".into(), Message::Increment);
    let decrement = make_button("-".into(), Message::Decrement);

    let button_row = row(vec![
        Node::spacer(),
        increment,
        Node::spacer().width(2),
        decrement,
        Node::spacer(),
    ])
    .width(Length::Shrink)
    .padding(5);

    root.push(Node::spacer());
    root.push(column(vec![
        Node::spacer(),
        count_row,
        button_row,
        Node::spacer(),
    ]));
    root.push(Node::spacer());
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
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding([0, 6, 5, 6])
    .row();
    button_quad.push(button_text);

    let mut button_node = Node::new(Button::new().on_press(action))
        .width(Length::Shrink)
        .height(Length::Shrink);
    button_node.push(button_quad);
    button_node
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT, |_| None)
}
