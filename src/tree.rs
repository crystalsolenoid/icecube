use crate::{
    element::Element,
    quad::{BorderStyle, Quad, QuadStyle},
};

pub struct Node {
    pub children: Vec<Node>,
    pub element: Box<dyn Element>,
    pub layout: Layout,
}

#[derive(Clone)]
pub enum Layout {
    Row,
    Column,
}

impl Node {
    pub fn root_node(width: usize, height: usize) -> Self {
        let window = Quad {
            width,
            height,
            style: QuadStyle {
                fill_style: None, // TODO should this be setting the background?
                border_style: Some(BorderStyle {
                    color: [0x15, 0x78, 0x8c, 0xff],
                    thickness: 1,
                }),
            },
        };
        Self::new(window, Layout::Row)
    }

    pub fn new(element: impl Element + 'static, layout: Layout) -> Self {
        Self {
            children: vec![],
            element: Box::new(element),
            layout,
        }
    }

    pub fn push(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn draw_recursive(&self, frame: &mut [u8], accum_position: (u32, u32)) {
        let new_position = accum_position;
        self.element.draw(frame, new_position);

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
}
