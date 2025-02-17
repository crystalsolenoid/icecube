use crate::quad::{BorderStyle, Quad, QuadStyle};

pub struct Node {
    pub children: Vec<Node>,
    pub relative_position: (u32, u32),
    pub element: Quad,
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
        Self::new(window, (0, 0))
    }

    pub fn new(element: Quad, relative_position: (u32, u32)) -> Self {
        Self {
            children: vec![],
            relative_position,
            element,
        }
    }

    pub fn push(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn draw_recursive(&self, frame: &mut [u8], accum_position: (u32, u32)) {
        let new_position = (
            accum_position.0 + self.relative_position.0,
            accum_position.1 + self.relative_position.1,
        );
        self.element.draw(frame, new_position);
        self.children
            .iter()
            .for_each(|node| node.draw_recursive(frame, new_position));
    }
}
