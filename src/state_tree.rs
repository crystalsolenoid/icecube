// See Iced, which we took most of this from at least to start

use std::any::Any;

use crate::tree::Node;

// #[derive(Debug)]
// pub struct Tag(any::TypeId);

// impl Tag {
//     pub fn of<T>() -> Self
//     where
//         T: 'static,
//     {
//         Self(any::TypeId::of::<T>())
//     }
// }

pub enum State {
    None,
    Some(Box<dyn Any>),
}

impl State {
    pub fn new<T>(state: T) -> Self
    where
        T: 'static,
    {
        State::Some(Box::new(state))
    }

    pub fn downcast_mut<T>(&mut self) -> &mut T
    where
        T: 'static,
    {
        match self {
            State::None => panic!("Downcast on stateless state"),
            State::Some(state) => state.downcast_mut().expect("Downcast widget state"),
        }
    }
}

pub struct StateNode {
    // pub tag: Tag,
    pub state: State,
    pub children: Vec<StateNode>,
}

impl StateNode {
    pub fn new<Message, Layout>(node: &Node<Message, Layout>) -> Self {
        Self {
            // tag: , // TODO when changing layout
            state: node.element.get_initial_state(),
            children: node
                .children
                .iter()
                .map(|node| StateNode::new(node))
                .collect(),
        }
    }
}
