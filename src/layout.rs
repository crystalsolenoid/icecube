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
    pub width: Length,
    pub height: Length,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}
#[derive(Clone, Copy)]
struct FlowCross(u32, u32);
#[derive(Clone, Copy)]
struct XY(u32, u32);

impl Layout {
    fn summed_padding(&self) -> FlowCross {
        let width = self.padding.left + self.padding.right;
        let height = self.padding.top + self.padding.bottom;
        match self.direction {
            LayoutDirection::Column => FlowCross(height, width),
            LayoutDirection::Row => FlowCross(width, height),
        }
    }
}

struct ShrinkLayout {
    // about the node itself
    pub width: ShrunkLength,
    pub height: ShrunkLength,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}
//TODO: generalize Layout, ShrinkLayout, and GrownLayout to avoid repetition
impl ShrinkLayout {
    fn summed_padding(&self) -> FlowCross {
        let width = self.padding.left + self.padding.right;
        let height = self.padding.top + self.padding.bottom;
        match self.direction {
            LayoutDirection::Column => FlowCross(height, width),
            LayoutDirection::Row => FlowCross(width, height),
        }
    }

    fn xy_to_flow_cross(&self, xy: XY) -> FlowCross {
        let XY(x, y) = xy;
        match self.direction {
            LayoutDirection::Column => FlowCross(y, x),
            LayoutDirection::Row => FlowCross(x, y),
        }
    }
}
struct GrownLayout {
    // about the node itself
    pub width: GrownLength,
    pub height: GrownLength,
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

#[derive(Clone, Copy, Default, PartialEq)]
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
        let root_size = match (self.layout.width, self.layout.height) {
            (Length::Fixed(w), Length::Fixed(h)) => XY(w, h),
            (_, _) => panic!(),
        };
        self.shrink_pass()
            .grow_pass(root_size)
            .position_pass((0, 0))
    }

    /// Render pass 1/3
    /// bottom-up pass
    fn shrink_pass(self) -> Node<ShrinkLayout> {
        let new_children: Vec<_> = self.children.into_iter().map(|c| c.shrink_pass()).collect();
        let flow_cross_padding = self.layout.summed_padding();
        //TODO: calculate new values in this first match statement?
        let (flow_length, cross_length) = match self.layout.direction {
            LayoutDirection::Column => (self.layout.height, self.layout.width),
            LayoutDirection::Row => (self.layout.width, self.layout.height),
        };

        //// Flow
        let new_flow_length = match flow_length {
            Length::Shrink => {
                let l: u32 = new_children
                    .iter()
                    .map(|child| match self.layout.direction {
                        LayoutDirection::Column => match child.layout.height {
                            ShrunkLength::Grow => 0,
                            ShrunkLength::Fixed(l) => l,
                        },
                        LayoutDirection::Row => match child.layout.width {
                            ShrunkLength::Grow => 0,
                            ShrunkLength::Fixed(l) => l,
                        },
                    })
                    .sum();
                let total_spacing =
                    new_children.len().saturating_sub(1) as u32 * self.layout.spacing;
                ShrunkLength::Fixed(l + flow_cross_padding.0 + total_spacing)
            }
            Length::Grow => ShrunkLength::Grow,
            Length::Fixed(l) => ShrunkLength::Fixed(l),
        };

        //// Cross
        let new_cross_length = match cross_length {
            Length::Shrink => {
                let max_child_cross_length: u32 = new_children
                    .iter()
                    .map(|child| match self.layout.direction {
                        LayoutDirection::Column => match child.layout.width {
                            ShrunkLength::Grow => 0,
                            ShrunkLength::Fixed(l) => l,
                        },
                        LayoutDirection::Row => match child.layout.height {
                            ShrunkLength::Grow => 0,
                            ShrunkLength::Fixed(l) => l,
                        },
                    })
                    .max()
                    .unwrap_or(0);
                ShrunkLength::Fixed(max_child_cross_length + flow_cross_padding.1)
            }
            Length::Grow => ShrunkLength::Grow,
            Length::Fixed(l) => ShrunkLength::Fixed(l),
        };

        let (new_width, new_height) = match self.layout.direction {
            LayoutDirection::Column => (new_cross_length, new_flow_length),
            LayoutDirection::Row => (new_flow_length, new_cross_length),
        };
        Node {
            layout: ShrinkLayout {
                width: new_width,
                height: new_height,
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
    fn grow_pass(self, assigned_xy: XY) -> Node<GrownLayout> {
        let FlowCross(assigned_flow_length, assigned_cross_length) =
            self.layout.xy_to_flow_cross(assigned_xy);
        let flow_cross_padding = self.layout.summed_padding();
        let remaining_length = assigned_flow_length
            .saturating_sub(
                self.children
                    .iter()
                    .map(|c| match self.layout.direction {
                        LayoutDirection::Column => match c.layout.height {
                            ShrunkLength::Grow => 0,
                            ShrunkLength::Fixed(l) => l,
                        },
                        LayoutDirection::Row => match c.layout.width {
                            ShrunkLength::Grow => 0,
                            ShrunkLength::Fixed(l) => l,
                        },
                    })
                    .sum::<u32>(),
            )
            .saturating_sub(flow_cross_padding.0)
            .saturating_sub(self.layout.spacing * self.children.len().saturating_sub(1) as u32);

        let child_grow_number: u32 = self
            .children
            .iter()
            .filter(|c| match self.layout.direction {
                LayoutDirection::Column => c.layout.height == ShrunkLength::Grow,
                LayoutDirection::Row => c.layout.width == ShrunkLength::Grow,
            })
            .count()
            .try_into()
            .unwrap();

        let available_cross_length: GrownLength = assigned_cross_length - flow_cross_padding.1;

        let new_children: Vec<_> = self
            .children
            .into_iter()
            .map(|c| {
                let child_width = match (self.layout.direction, c.layout.width) {
                    (LayoutDirection::Column, ShrunkLength::Grow) => available_cross_length,
                    (LayoutDirection::Row, ShrunkLength::Grow) => {
                        remaining_length / child_grow_number
                    }
                    (_, ShrunkLength::Fixed(l)) => l,
                };
                let child_height = match (self.layout.direction, c.layout.height) {
                    (LayoutDirection::Column, ShrunkLength::Grow) => {
                        remaining_length / child_grow_number
                    }
                    (LayoutDirection::Row, ShrunkLength::Grow) => available_cross_length,
                    (_, ShrunkLength::Fixed(l)) => l,
                };

                c.grow_pass(XY(child_width, child_height))
            })
            .collect();

        Node {
            layout: GrownLayout {
                width: assigned_xy.0,
                height: assigned_xy.1,
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
            parent_position.0 + self.layout.padding.top,
            parent_position.1 + self.layout.padding.left,
        );
        let new_children: Vec<_> = self
            .children
            .into_iter()
            .scan(first_child_position, |accumulated_position, child_node| {
                let start_position = *accumulated_position;
                match self.layout.direction {
                    LayoutDirection::Row => {
                        accumulated_position.0 += child_node.layout.width + self.layout.spacing;
                    }
                    LayoutDirection::Column => {
                        accumulated_position.1 += child_node.layout.height + self.layout.spacing;
                    }
                };
                Some(child_node.position_pass(start_position))
            })
            .collect();

        let (x, y) = parent_position;

        Node {
            layout: CalculatedLayout::test(x, y, self.layout.width, self.layout.height),
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
