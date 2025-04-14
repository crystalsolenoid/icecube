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

#[derive(Clone, Copy, Default, PartialEq)]
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
        // TODO: Use better types for root node, so we don't have to match for unsupported root
        // node length types
        let root_length = match self.layout.flow_length {
            Length::Grow => todo!(),
            Length::Shrink => todo!(),
            Length::Fixed(l) => l,
        };
        self.shrink_pass()
            .grow_pass(root_length)
            .position_pass((0, 0))
    }

    /// Render pass 1/3
    /// bottom-up pass
    fn shrink_pass(self) -> Node<ShrinkLayout> {
        let new_children: Vec<_> = self.children.into_iter().map(|c| c.shrink_pass()).collect();
        let new_flow_length = match self.layout.flow_length {
            Length::Shrink => {
                let l: u32 = new_children
                    .iter()
                    .map(|child| match child.layout.flow_length {
                        ShrunkLength::Grow => 0,
                        ShrunkLength::Fixed(l) => l,
                    })
                    .sum();
                let total_spacing =
                    new_children.len().saturating_sub(1) as u32 * self.layout.spacing;
                ShrunkLength::Fixed(
                    l + self.layout.padding.right + self.layout.padding.left + total_spacing,
                )
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
    /// Render pass 2/3
    /// top-down
    fn grow_pass(self, assigned_length: GrownLength) -> Node<GrownLayout> {
        let remaining_length = assigned_length
            - self
                .children
                .iter()
                .map(|c| match c.layout.flow_length {
                    ShrunkLength::Grow => 0,
                    ShrunkLength::Fixed(l) => l,
                })
                .sum::<u32>()
            - self.layout.padding.left
            - self.layout.padding.right
            - self.layout.spacing * self.children.len().saturating_sub(1) as u32;

        let child_grow_number: u32 = self
            .children
            .iter()
            .filter(|c| c.layout.flow_length == ShrunkLength::Grow)
            .count()
            .try_into()
            .unwrap();

        let new_children: Vec<_> = self
            .children
            .into_iter()
            .map(|c| match c.layout.flow_length {
                //TODO: Potentially can lose up to 1 pixel of space per child
                ShrunkLength::Grow => c.grow_pass(remaining_length / child_grow_number),
                ShrunkLength::Fixed(l) => c.grow_pass(l),
            })
            .collect();

        Node {
            layout: GrownLayout {
                flow_length: assigned_length,
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
    /// Render pass 3/3
    /// top-down
    fn position_pass(self, parent_position: (u32, u32)) -> Node<CalculatedLayout> {
        let first_child_position = (
            parent_position.0 + self.layout.padding.left,
            parent_position.1,
        );
        let new_children: Vec<_> = self
            .children
            .into_iter()
            .scan(first_child_position, |accumulated_position, child_node| {
                let start_position = *accumulated_position;
                match self.layout.direction {
                    LayoutDirection::Row => {
                        accumulated_position.0 +=
                            child_node.layout.flow_length + self.layout.spacing;
                    }
                    LayoutDirection::Column => {
                        accumulated_position.1 +=
                            child_node.layout.flow_length + self.layout.spacing;
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

impl From<u32> for Padding {
    fn from(value: u32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
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
