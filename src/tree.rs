use crate::{
    element::Element,
    layout::LayoutEngineResult,
    quad::{Quad, QuadStyle},
};

pub struct Node {
    pub children: Vec<Node>,
    pub element: Box<dyn Element>,
    pub layout: Layout,
    pub layout_calc: LayoutEngineResult,
}

#[derive(Clone)]
pub enum Layout {
    Row,
    Column,
}

impl Node {
    pub fn root_node(width: usize, height: usize) -> Self {
        let window = Quad::new(width as u32, height as u32).style(QuadStyle {
            fill_style: None, // TODO should this be setting the background?
            border_style: None,
        });
        Self::new(
            window,
            Layout::Row,
            LayoutEngineResult::test(0, 0, width as u32, height as u32),
        )
    }

    pub fn new(
        element: impl Element + 'static,
        layout: Layout,
        layout_calc: LayoutEngineResult,
    ) -> Self {
        Self {
            children: vec![],
            element: Box::new(element),
            layout,
            layout_calc,
        }
    }

    pub fn push(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn draw_recursive(&self, frame: &mut [u8], _accum_position: (u32, u32)) {
        let LayoutEngineResult { x, y, w, h } = self.layout_calc;
        self.element.draw(frame, (x, y));
        self.children
            .iter()
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
            .filter(|child| child.layout_calc.contains(position))
            //TODO: pass position relative to child
            .for_each(|child| child.on_click(position));
        self.element.on_click(position);
    }
}
