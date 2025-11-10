use crate::{
    layout::{Layout, Length},
    quad::Quad,
    tree::Node,
};

pub fn create_row<Message>(elements: Vec<Node<Message, Layout>>) -> Node<Message, Layout> {
    let mut row = Node::new(Quad::new())
        .row()
        .width(Length::Grow)
        .height(Length::Shrink);

    for element in elements.into_iter() {
        row.push(element);
    }
    row
}

pub fn create_column<Message>(elements: Vec<Node<Message, Layout>>) -> Node<Message, Layout> {
    let mut col = Node::new(Quad::new())
        .column()
        .height(Length::Grow)
        .width(Length::Shrink);

    for element in elements.into_iter() {
        col.push(element);
    }
    col
}

#[macro_export]
macro_rules! row {
    () => (
        $crate::widget::create_row(std::vec::Vec::new())
    );
    ($($x:expr),+ $(,)?) => (
        $crate::widget::create_row(vec![$($x),+])
    );
}
#[macro_export]
macro_rules! col {
    () => (
        $crate::widget::create_column(std::vec::Vec::new())
    );
    ($($x:expr),+ $(,)?) => (
        $crate::widget::create_column(vec![$($x),+])
    );
}
