use crate::tree::Node;

#[derive(Clone, Copy, Default, Debug)]
pub struct CalculatedLayout {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, Default)]
pub struct Layout {
    // about the node itself
    pub cross_length: GrownLength,
    pub flow_length: Length,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}

struct ShrinkLayout {
    // about the node itself
    pub cross_length: GrownLength,
    pub flow_length: ShrunkLength,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}
struct GrownLayout {
    // about the node itself
    pub cross_length: GrownLength,
    pub flow_length: GrownLength,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}

#[derive(Clone, Copy, Default)]
enum ShrunkLength {
    #[default]
    Grow,
    Fixed(u32),
}

type GrownLength = u32;

#[derive(Clone, Copy, Default)]
pub enum Length {
    #[default]
    Grow,
    Shrink,
    Fixed(u32),
}

#[derive(Clone, Copy, Default)]
pub enum LayoutDirection {
    Column,
    #[default]
    Row,
}

#[derive(Clone, Copy, Default)]
pub struct Padding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

//    fn layout_parameters(&self) -> LayoutParameters {
impl Node<Layout> {
    pub fn calculate_layout(self) -> Node<CalculatedLayout> {
        self.shrink_pass().grow_pass().position_pass((0, 0))
    }

    fn shrink_pass(self) -> Node<ShrinkLayout> {
        let new_children: Vec<_> = self.children.into_iter().map(|c| c.shrink_pass()).collect();
        let new_flow_length = match self.layout.flow_length {
            Length::Shrink => {
                let l = new_children
                    .iter()
                    .map(|child| match child.layout.flow_length {
                        ShrunkLength::Grow => 0,
                        ShrunkLength::Fixed(l) => l,
                    })
                    .sum();
                ShrunkLength::Fixed(l)
            }
            Length::Grow => ShrunkLength::Grow,
            Length::Fixed(l) => ShrunkLength::Fixed(l),
        };

        Node {
            layout: ShrinkLayout {
                flow_length: new_flow_length,
                cross_length: self.layout.cross_length,
                padding: self.layout.padding,
                direction: self.layout.direction,
                spacing: self.layout.spacing,
            },
            children: new_children,
            element: self.element,
        }
    }
}

impl Node<ShrinkLayout> {
    fn grow_pass(self) -> Node<GrownLayout> {
        let new_flow_length = match self.layout.flow_length {
            ShrunkLength::Grow => 50,
            ShrunkLength::Fixed(l) => l,
        };
        let new_children: Vec<_> = self.children.into_iter().map(|c| c.grow_pass()).collect();

        Node {
            layout: GrownLayout {
                flow_length: new_flow_length,
                cross_length: self.layout.cross_length,
                padding: self.layout.padding,
                direction: self.layout.direction,
                spacing: self.layout.spacing,
            },
            children: new_children,
            element: self.element,
        }
    }
}
impl Node<GrownLayout> {
    fn position_pass(self, parent_position: (u32, u32)) -> Node<CalculatedLayout> {
        let new_children: Vec<_> = self
            .children
            .into_iter()
            .scan(parent_position, |accumulated_position, child_node| {
                let start_position = *accumulated_position;
                match self.layout.direction {
                    LayoutDirection::Row => {
                        accumulated_position.0 += child_node.element.width();
                    }
                    LayoutDirection::Column => {
                        accumulated_position.1 += child_node.element.height();
                    }
                };
                Some(child_node.position_pass(start_position))
            })
            .collect();

        let x = parent_position.0;
        let y = parent_position.1;
        let w = self.layout.flow_length;
        let h = self.layout.cross_length;
        Node {
            layout: CalculatedLayout::test(x, y, w, h),
            children: new_children,
            element: self.element,
        }
    }
}

impl CalculatedLayout {
    pub fn test(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, pos: (u32, u32)) -> bool {
        pos.0 >= self.x && pos.0 < self.x + self.w && pos.1 >= self.y && pos.1 < self.y + self.h
    }
}

impl From<[u32; 2]> for Padding {
    fn from(value: [u32; 2]) -> Self {
        Self {
            top: value[0],
            right: value[1],
            bottom: value[0],
            left: value[1],
        }
    }
}

impl From<[u32; 4]> for Padding {
    fn from(value: [u32; 4]) -> Self {
        Self {
            top: value[0],
            right: value[1],
            bottom: value[2],
            left: value[3],
        }
    }
}
