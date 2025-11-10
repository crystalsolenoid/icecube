use crate::{
    layout::{Layout, Length},
    quad::Quad,
    tree::Node,
};

pub fn row<Message>(elements: Vec<Node<Message, Layout>>) -> Node<Message, Layout> {
    let mut row = Node::new(Quad::new())
        .row()
        .width(Length::Grow)
        .height(Length::Shrink);

    for element in elements.into_iter() {
        row.push(element);
    }
    row
}

pub fn column<Message>(elements: Vec<Node<Message, Layout>>) -> Node<Message, Layout> {
    let mut col = Node::new(Quad::new())
        .column()
        .height(Length::Grow)
        .width(Length::Shrink);

    for element in elements.into_iter() {
        col.push(element);
    }
    col
}
