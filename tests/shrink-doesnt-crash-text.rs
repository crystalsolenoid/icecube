use icecube::layout::Length;
use icecube::quad::Quad;
use icecube::text::Text;
use icecube::tree::Node;

#[test]
fn shrink_doesnt_crash_text() {
    let mut root: Node<(), _> = Node::root_node(320, 240).row();
    root.name = Some("root".to_string());
    let mut text = Node::new(Text::new("thisisalonglongword".to_string()));
    text.name = Some("text".to_string());
    let mut shrink = Node::new(Quad::new()).width(Length::Shrink);
    shrink.name = Some("shrink".to_string());
    shrink.push(text);
    root.push(shrink);

    let _rendered = root.calculate_layout();
}

#[test]
fn double_shrink_doesnt_crash_text() {
    let mut root: Node<(), _> = Node::root_node(320, 240).row();

    let mut container = Node::new(Quad::new()).column().width(Length::Shrink);

    let mut count_row = Node::new(Quad::new()).row().width(Length::Grow);
    let count = Node::new(Text::new(format!("{}", "123")));

    count_row.push(count);

    container.push(count_row);

    root.push(container);

    let _rendered = root.calculate_layout();
}

#[test]
fn shrink_height_text() {
    //TODO: Simplify test
    let mut root = Node::root_node(320, 240).row();

    // This fills the screen, causing the screen to clear each frame
    let mut container = Node::new(Quad::new().fill(MAIN_LIGHT)).column();

    let font = &font::BLACKLETTER;

    let mut count_row = Node::new(Quad::new().border_color(RED_DARK).border_thickness(1))
        .row()
        .height(Length::Shrink);
    let count = Node::new(Text::new(format!("{}", state.count)).with_font(font));
    let mut count_container = Node::new(Quad::new());
    count_container.push(count);

    count_row.push(Node::new(
        Quad::new().border_color(RED_LIGHT).border_thickness(1),
    ));
    count_row.push(count_container);
    count_row.push(Node::new(
        Quad::new().border_color(RED_LIGHT).border_thickness(1),
    ));

    let increment = make_button("+".into(), Message::Increment);
    let decrement = make_button("-".into(), Message::Decrement);

    let mut button_row = Node::new(Quad::new().border_color(RED_DARK).border_thickness(1)).row();
    button_row.push(increment);
    button_row.push(Node::new(Quad::new()).width(Length::Fixed(2)));
    button_row.push(decrement);

    container.push(Node::new(Quad::new()));
    container.push(count_row);
    container.push(button_row);
    container.push(Node::new(Quad::new()));

    root.push(Node::new(Quad::new()));
    root.push(container);
    root.push(Node::new(Quad::new()));
}
