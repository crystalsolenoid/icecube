use crate::{
    element::Element,
    layout::{CalculatedLayout, Layout, LayoutDirection, Length, Padding},
    quad::{Quad, QuadStyle},
};

pub struct Node<LayoutStage> {
    pub children: Vec<Node<LayoutStage>>,
    pub element: Box<dyn Element>,
    pub layout: LayoutStage, //Option<CalculatedLayout>,
}

impl Node<Layout> {
    pub fn root_node(width: usize, height: usize) -> Self {
        let window = Quad::new()
            .style(QuadStyle {
                fill_style: None, // TODO should this be setting the background?
                border_style: None,
            })
            .border_thickness(1)
            .border_color(crate::palette::BLUE_LIGHT);
        Self {
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

    pub fn new(element: impl Element + 'static) -> Self {
        Self {
            children: vec![],
            element: Box::new(element),
            layout: Layout {
                ..Layout::default()
            },
        }
    }

    pub fn width(self, width: Length) -> Self {
        // TODO make Length implement from u32
        Self {
            layout: Layout {
                width,
                ..self.layout
            },
            ..self
        }
    }

    pub fn height(self, height: Length) -> Self {
        Self {
            layout: Layout {
                height,
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

    pub fn push(&mut self, child: Self) {
        self.children.push(child);
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

impl Node<CalculatedLayout> {
    pub fn draw_recursive(&self, frame: &mut [u8], _accum_position: (u32, u32)) {
        // TODO can we remove mut from self?
        self.element.draw(frame, self.layout);
        self.children
            .iter() // TODO mut bad
            .for_each(|node| node.draw_recursive(frame, (0, 0)));
    }
    pub fn on_click(&self, position: (u32, u32)) {
        self.children
            .iter()
            .filter(|child| child.layout.contains(position))
            //TODO: pass position relative to child
            .for_each(|child| child.on_click(position));
        self.element.on_click(position);
    }
}
