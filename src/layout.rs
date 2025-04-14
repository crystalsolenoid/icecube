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
    pub cross_length: Length,
    pub flow_length: Length,
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
    pub cross_length: ShrunkLength,
    pub flow_length: ShrunkLength,
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

    fn flow_cross_to_xy(&self, flow_cross: FlowCross) -> XY {
        let FlowCross(flow, cross) = flow_cross;
        match self.direction {
            LayoutDirection::Column => XY(cross, flow),
            LayoutDirection::Row => XY(flow, cross),
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
    pub cross_length: GrownLength,
    pub flow_length: GrownLength,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}
impl GrownLayout {
    fn xy_size(&self) -> XY {
        let flow = self.flow_length;
        let cross = self.cross_length;
        match self.direction {
            LayoutDirection::Column => XY(cross, flow),
            LayoutDirection::Row => XY(flow, cross),
        }
    }
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
        let root_length = match self.layout.flow_length {
            Length::Grow => panic!(),
            Length::Shrink => panic!(),
            Length::Fixed(l) => l,
        };
        let root_cross_length = match self.layout.cross_length {
            Length::Grow => panic!(),
            Length::Shrink => panic!(),
            Length::Fixed(l) => l,
        };
        self.shrink_pass()
            .grow_pass(XY(root_length, root_cross_length))
            .position_pass((0, 0))
    }

    /// Render pass 1/3
    /// bottom-up pass
    fn shrink_pass(self) -> Node<ShrinkLayout> {
        let new_children: Vec<_> = self.children.into_iter().map(|c| c.shrink_pass()).collect();
        let flow_cross_padding = self.layout.summed_padding();
        //// Flow
        let new_flow_length = match self.layout.flow_length {
            Length::Shrink => {
                let l: u32 = new_children
                    .iter()
                    .map(
                        |child| match self.layout.direction == child.layout.direction {
                            true => match child.layout.flow_length {
                                ShrunkLength::Grow => 0,
                                ShrunkLength::Fixed(l) => l,
                            },
                            false => match child.layout.cross_length {
                                ShrunkLength::Grow => 0,
                                ShrunkLength::Fixed(l) => l,
                            },
                        },
                    )
                    .sum();
                let total_spacing =
                    new_children.len().saturating_sub(1) as u32 * self.layout.spacing;
                ShrunkLength::Fixed(l + flow_cross_padding.0 + total_spacing)
            }
            Length::Grow => ShrunkLength::Grow,
            Length::Fixed(l) => ShrunkLength::Fixed(l),
        };

        //// Cross
        let new_cross_length = match self.layout.cross_length {
            Length::Shrink => {
                let max_child_cross_length: u32 = new_children
                    .iter()
                    .map(
                        |child| match self.layout.direction == child.layout.direction {
                            true => match child.layout.cross_length {
                                ShrunkLength::Grow => 0,
                                ShrunkLength::Fixed(l) => l,
                            },
                            false => match child.layout.flow_length {
                                ShrunkLength::Grow => 0,
                                ShrunkLength::Fixed(l) => l,
                            },
                        },
                    )
                    .max()
                    .unwrap_or(0);
                ShrunkLength::Fixed(max_child_cross_length + flow_cross_padding.1)
            }
            Length::Grow => ShrunkLength::Grow,
            Length::Fixed(l) => ShrunkLength::Fixed(l),
        };

        Node {
            layout: ShrinkLayout {
                flow_length: new_flow_length,
                cross_length: new_cross_length,
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
    fn grow_pass(
        self,
        assigned_xy: XY,
        //assigned_flow_length: GrownLength,
        //assigned_cross_length: GrownLength,
    ) -> Node<GrownLayout> {
        let FlowCross(assigned_flow_length, assigned_cross_length) =
            self.layout.xy_to_flow_cross(assigned_xy);
        let flow_cross_padding = self.layout.summed_padding();
        let remaining_length = assigned_flow_length
            .saturating_sub(
                self.children
                    .iter()
                    .map(|c| match self.layout.direction == c.layout.direction {
                        true => match c.layout.flow_length {
                            ShrunkLength::Grow => 0,
                            ShrunkLength::Fixed(l) => l,
                        },
                        false => match c.layout.cross_length {
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
            .filter(|c| c.layout.flow_length == ShrunkLength::Grow)
            .count()
            .try_into()
            .unwrap();

        let available_cross_length: GrownLength = assigned_cross_length - flow_cross_padding.1;

        let new_children: Vec<_> = self
            .children
            .into_iter()
            .map(|c| {
                let child_flow_length = match self.layout.direction == c.layout.direction {
                    true => match c.layout.flow_length {
                        ShrunkLength::Grow => remaining_length / child_grow_number,
                        ShrunkLength::Fixed(l) => l,
                    },
                    false => match c.layout.cross_length {
                        ShrunkLength::Grow => remaining_length / child_grow_number,
                        ShrunkLength::Fixed(l) => l,
                    },
                };

                let child_cross_length = match self.layout.direction == c.layout.direction {
                    true => match c.layout.cross_length {
                        ShrunkLength::Grow => available_cross_length,
                        ShrunkLength::Fixed(l) => l,
                    },
                    false => match c.layout.flow_length {
                        ShrunkLength::Grow => available_cross_length,
                        ShrunkLength::Fixed(l) => l,
                    },
                };

                let child_size_xy = c
                    .layout
                    .flow_cross_to_xy(FlowCross(child_flow_length, child_cross_length));
                c.grow_pass(child_size_xy)
            })
            .collect();

        Node {
            layout: GrownLayout {
                flow_length: assigned_flow_length,
                cross_length: assigned_cross_length,
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
                        accumulated_position.0 +=
                            child_node.layout.xy_size().0 + self.layout.spacing;
                    }
                    LayoutDirection::Column => {
                        accumulated_position.1 +=
                            child_node.layout.xy_size().1 + self.layout.spacing;
                    }
                };
                Some(child_node.position_pass(start_position))
            })
            .collect();

        let x = parent_position.0;
        let y = parent_position.1;

        let xy_size = self.layout.xy_size();
        Node {
            layout: CalculatedLayout::test(x, y, xy_size.0, xy_size.1),
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
