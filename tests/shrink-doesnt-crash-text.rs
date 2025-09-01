use icecube::layout::Length;
use icecube::quad::Quad;
use icecube::text::Text;
use icecube::tree::Node;

#[test]
fn shrink_doesnt_crash_text() {
    let mut root: Node<(), _> = Node::root_node(320, 240).row();
    let text = Node::new(Text::new("thisisalonglongword".to_string()));
    let mut shrink = Node::new(Quad::new()).width(Length::Shrink);
    shrink.push(text);
    root.push(shrink);

    let _rendered = root.calculate_layout();
}
