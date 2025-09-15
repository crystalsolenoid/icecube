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
