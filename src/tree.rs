use std::fmt::Debug;

use crate::{
    element::Element,
    layout::{CalculatedLayout, Layout, LayoutDirection, Length, Padding},
    quad::{Quad, QuadStyle},
    state_tree::StateNode,
};

pub struct Node<Message, LayoutStage> {
    pub children: Vec<Node<Message, LayoutStage>>,
    pub element: Box<dyn Element<Message>>,
    pub layout: LayoutStage, //Option<CalculatedLayout>,
    pub name: Option<String>,
}

impl<Message, LayoutStage: Debug> Debug for Node<Message, LayoutStage> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("name", &self.name)
            .field("layout", &self.layout)
            .field("children", &self.children)
            .finish()
    }
}

impl<Message> Node<Message, Layout> {
    pub fn root_node(width: usize, height: usize) -> Self {
        let window = Quad::new().style(QuadStyle {
            fill_style: None, // TODO should this be setting the background?
            border_style: None,
        });

        Self {
            name: Some("root".to_string()),
            layout: Layout {
                width: Length::Fixed(width as u32),
                height: Length::Fixed(height as u32),
                //TODO: create an API that automatically keeps quad border thickness and padding in
                //sync
                padding: 1.into(),
                ..Layout::default()
            },
            ..Self::new(window)
        }
    }

    pub fn new(element: impl Element<Message> + 'static) -> Self {
        Self {
            children: vec![],
            element: Box::new(element),
            layout: Layout {
                ..Layout::default()
            },
            name: None,
        }
    }

    pub fn spacer() -> Self {
        Self {
            children: vec![],
            element: Box::new(Quad::new()),
            layout: Layout {
                width: Length::Grow,
                height: Length::Grow,
                ..Layout::default()
            },
            name: None,
        }
    }

    pub fn width(self, width: impl Into<Length>) -> Self {
        // TODO make Length implement from u32
        Self {
            layout: Layout {
                width: width.into(),
                ..self.layout
            },
            ..self
        }
    }

    pub fn height(self, height: impl Into<Length>) -> Self {
        Self {
            layout: Layout {
                height: height.into(),
                ..self.layout
            },
            ..self
        }
    }

    pub fn padding(self, padding: impl Into<Padding>) -> Self {
        Self {
            layout: Layout {
                padding: padding.into(),
                ..self.layout
            },
            ..self
        }
    }

    pub fn spacing(self, spacing: u32) -> Self {
        Self {
            layout: Layout {
                spacing,
                ..self.layout
            },
            ..self
        }
    }

    pub fn stack(self) -> Self {
        Self {
            layout: Layout {
                direction: LayoutDirection::Stack,
                ..self.layout
            },
            ..self
        }
    }

    pub fn row(self) -> Self {
        Self {
            layout: Layout {
                direction: LayoutDirection::Row,
                ..self.layout
            },
            ..self
        }
    }

    pub fn column(self) -> Self {
        Self {
            layout: Layout {
                direction: LayoutDirection::Column,
                ..self.layout
            },
            ..self
        }
    }

    pub fn push(&mut self, child: impl Into<Self>) {
        self.children.push(child.into());
    }

    pub fn with_element(self, element: impl Element<Message> + 'static) -> Self {
        Self {
            element: Box::new(element),
            ..self
        }
    }

    /*
    pub fn draw_recursive(&self, frame: &mut [u8], accum_position: (u32, u32)) {
        self.element.draw(frame, accum_position);
        let new_position = (
            accum_position.0 + self.element.padding().left,
            accum_position.1 + self.element.padding().top,
        );

        let positions = self
            .children
            .iter()
            .scan(new_position, |child_position, child_node| {
                let start_position = *child_position;
                match self.layout {
                    Layout::Row => {
                        child_position.0 += child_node.element.width();
                    }
                    Layout::Column => {
                        child_position.1 += child_node.element.height();
                    }
                }
                //TODO: check if drawing off the screen and warn if so
                Some(start_position)
            });
        self.children
            .iter()
            .zip(positions)
            .for_each(|(node, position)| node.draw_recursive(frame, position));
    }
    */
}

impl<Message> Node<Message, CalculatedLayout> {
    pub fn draw_recursive(&self, frame: &mut [u8], _accum_position: (u32, u32)) {
        // TODO can we remove mut from self?
        self.element.draw(frame, self.layout);
        self.children
            .iter() // TODO mut bad
            .for_each(|node| node.draw_recursive(frame, (0, 0)));
    }

    pub fn get_message(
        &mut self,
        state_tree: &mut StateNode,
        input: &crate::Input,
    ) -> Option<Message> {
        let message = self.element.get_message(state_tree, input, self.layout);
        if message.is_some() {
            message
        } else {
            self.children
                .iter_mut()
                .zip(&mut state_tree.children)
                .filter_map(|(child, child_state)| child.get_message(child_state, input))
                .next()
        }
    }
}
