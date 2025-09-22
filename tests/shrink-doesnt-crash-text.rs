use icecube::font;
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
    let mut root: Node<(), _> = Node::root_node(320, 240).row();

    let mut count_row = Node::new(Quad::new()).height(Length::Shrink);
    let mut count_container = Node::new(Quad::new());
    let mut count = Node::new(Text::new("12".to_string()));

    // label for testing
    count_row.name = Some("count row".to_string());
    count_container.name = Some("count container".to_string());
    count.name = Some("count".to_string());

    count_container.push(count);
    count_row.push(count_container);
    root.push(count_row);

    // these set up testing and make sure our setup was done right
    let rendered = root.calculate_layout();
    let r_count_row = &rendered.children[0];
    assert_eq!(r_count_row.name, Some("count row".to_string()));
    let r_count_container = &r_count_row.children[0];
    assert_eq!(r_count_container.name, Some("count container".to_string()));
    let r_count = &r_count_container.children[0];
    assert_eq!(r_count.name, Some("count".to_string()));

    // these are the real tests
    assert!(r_count_row.layout.h > 0); // count_row
    assert!(r_count_container.layout.h > 0); // count_container
    assert!(r_count.layout.h > 0); // count
}
