use crate::{
    element::Element,
    layout::{CalculatedLayout, LayoutParameters},
    quad::{Quad, QuadStyle},
};

pub struct Node {
    pub children: Vec<Node>,
    pub element: Box<dyn Element>,
    pub layout: Option<CalculatedLayout>,
}

impl Node {
    pub fn root_node(width: usize, height: usize) -> Self {
        let window = Quad::new(width as u32, height as u32).style(QuadStyle {
            fill_style: None, // TODO should this be setting the background?
            border_style: None,
        });
        Self::new(
            window,
            CalculatedLayout::test(0, 0, width as u32, height as u32),
        )
    }

    pub fn new(element: impl Element + 'static, layout: CalculatedLayout) -> Self {
        Self {
            children: vec![],
            element: Box::new(element),
            layout: Some(layout),
        }
    }

    pub fn push(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn get_layout(&mut self) -> CalculatedLayout {
        // TODO this forces functions that use it to be mutating. How can we fix that?
        match self.layout {
            Some(layout) => layout,
            None => {
                self.calculate_layout();
                self.layout.expect("calculate_layout should have set this")
            }
        }
    }

    pub fn draw_recursive(&mut self, frame: &mut [u8], _accum_position: (u32, u32)) {
        // TODO can we remove mut from self?
        let CalculatedLayout { x, y, w, h } = self.get_layout();
        self.element.draw(frame, (x, y));
        self.children
            .iter_mut() // TODO mut bad
            .for_each(|node| node.draw_recursive(frame, (0, 0)));
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

    pub fn on_click(&self, position: (u32, u32)) {
        self.children
            .iter()
            .filter(|child| {
                child
                    .layout
                    .expect("Layout should be calculated at this point.")
                    .contains(position)
            })
            //TODO: pass position relative to child
            .for_each(|child| child.on_click(position));
        self.element.on_click(position);
    }
}
