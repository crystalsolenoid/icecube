use crate::tree::Node;

mod length_types;
mod padding;
mod pipeline_types;
pub use length_types::Length;
use length_types::{GrownLength, ShrunkLength, XY};
pub use padding::Padding;
pub use pipeline_types::{CalculatedLayout, Layout, LayoutDirection};
use pipeline_types::{GrownHeightLayout, GrownWidthLayout, ShrinkHeightLayout, ShrinkWidthLayout};

// advice from Clay https://www.youtube.com/watch?v=by9lQvpvMIc
// fit sizing widths
// grow and shrink sizing widths
// wrap text
// fit sizing heights
// grow and shrink sizing heights
// positions
// draw commands

impl<Message> Node<Message, Layout> {
    pub fn calculate_layout(self) -> Node<Message, CalculatedLayout> {
        // TODO: Use better types for root node, so we don't have to match for unsupported root
        // node length types
        let root_size = match (self.layout.width, self.layout.height) {
            (Length::Fixed(w), Length::Fixed(h)) => XY(w, h),
            (_, _) => panic!(),
        };
        dbg!(self
            .shrink_width_pass()
            .grow_width_pass(root_size.0)
            .wrap()
            .shrink_height_pass()
            .grow_height_pass(root_size.1)
            .position_pass((0, 0)))
    }

    /// Render pass 1/3
    /// bottom-up pass
    fn shrink_width_pass(self) -> Node<Message, ShrinkWidthLayout> {
        dbg!(self.element.min_width());
        let new_children: Vec<_> = self
            .children
            .into_iter()
            .map(|c| c.shrink_width_pass())
            .collect();

        let new_children_widths = new_children
            .iter()
            .map(|child| match dbg!(child.layout.width) {
                ShrunkLength::Grow => 0, //dbg!(child.element.min_width()),
                ShrunkLength::GrowWithMin(m) => m,
                ShrunkLength::Fixed(l) => l,
            });
        let new_width = match self.layout.width {
            Length::Grow => match self.element.min_width() {
                0 => ShrunkLength::Grow,
                w => ShrunkLength::GrowWithMin(w),
            },
            Length::Fixed(l) => ShrunkLength::Fixed(l),
            Length::Shrink => match self.layout.direction {
                LayoutDirection::Row => {
                    // Sum widths of children
                    let l: u32 = new_children_widths.sum();
                    let total_spacing =
                        new_children.len().saturating_sub(1) as u32 * self.layout.spacing;
                    ShrunkLength::Fixed(
                        l + self.layout.padding.left + self.layout.padding.right + total_spacing,
                    )
                }
                LayoutDirection::Column => {
                    // Get max child width
                    let max_child_cross_length: u32 = new_children_widths.max().unwrap_or(0);
                    ShrunkLength::Fixed(
                        max_child_cross_length
                            + self.layout.padding.left
                            + self.layout.padding.right,
                    )
                }
            },
        };

        Node {
            layout: ShrinkWidthLayout {
                width: new_width,
                height: self.layout.height, // keep old height
                padding: self.layout.padding,
                direction: self.layout.direction,
                spacing: self.layout.spacing,
            },
            children: new_children,
            element: self.element,
            name: self.name,
        }
    }
}

impl<Message> Node<Message, GrownWidthLayout> {
    fn wrap(self) -> Self {
        let height = match self.element.wrap(self.layout.width) {
            Some(h) => Length::Fixed(h),
            None => self.layout.height,
        };

        let new_children: Vec<_> = self.children.into_iter().map(|c| c.wrap()).collect();

        // TODO we're starting with making every text element have a fixed height that's its
        // minimum height after wrapping.

        Node {
            layout: GrownWidthLayout {
                width: self.layout.width,
                height,
                padding: self.layout.padding,
                direction: self.layout.direction,
                spacing: self.layout.spacing,
            },
            children: new_children,
            element: self.element,
            name: self.name,
        }
    }

    /// Render pass 1/3
    /// bottom-up pass
    fn shrink_height_pass(self) -> Node<Message, ShrinkHeightLayout> {
        let new_children: Vec<_> = self
            .children
            .into_iter()
            .map(|c| c.shrink_height_pass())
            .collect();

        let new_children_heights = new_children.iter().map(|child| match child.layout.height {
            ShrunkLength::Grow => 0,
            ShrunkLength::GrowWithMin(l) => l,
            ShrunkLength::Fixed(l) => l,
        });
        let new_height = match self.layout.height {
            Length::Grow => ShrunkLength::Grow,
            Length::Fixed(l) => ShrunkLength::Fixed(l),
            Length::Shrink => match self.layout.direction {
                LayoutDirection::Column => {
                    // Sum heights of children
                    let l: u32 = new_children_heights.sum();
                    let total_spacing =
                        new_children.len().saturating_sub(1) as u32 * self.layout.spacing;
                    ShrunkLength::Fixed(
                        l + self.layout.padding.top + self.layout.padding.bottom + total_spacing,
                    )
                }
                LayoutDirection::Row => {
                    // Get max child height
                    let max_child_cross_length: u32 = new_children_heights.max().unwrap_or(0);
                    ShrunkLength::Fixed(
                        max_child_cross_length
                            + self.layout.padding.top
                            + self.layout.padding.bottom,
                    )
                }
            },
        };

        Node {
            layout: ShrinkHeightLayout {
                width: self.layout.width,
                height: new_height,
                padding: self.layout.padding,
                direction: self.layout.direction,
                spacing: self.layout.spacing,
            },
            children: new_children,
            element: self.element,
            name: self.name,
        }
    }
}

impl<Message> Node<Message, ShrinkWidthLayout> {
    /// Render pass 2/3
    /// top-down
    fn grow_width_pass(self, assigned_width: GrownLength) -> Node<Message, GrownWidthLayout> {
        let new_children_widths: Vec<_> = match self.layout.direction {
            LayoutDirection::Column => self
                .children
                .iter()
                .map(|c| match c.layout.width {
                    ShrunkLength::Grow => {
                        // available width
                        assigned_width - (self.layout.padding.left + self.layout.padding.right)
                    }
                    ShrunkLength::GrowWithMin(l) => {
                        let new_width =
                            assigned_width - (self.layout.padding.left + self.layout.padding.right);
                        assert!(new_width >= l);
                        new_width
                    }
                    ShrunkLength::Fixed(l) => l,
                })
                .collect(),
            LayoutDirection::Row => {
                let remaining_length = assigned_width
                    .saturating_sub(
                        self.children
                            .iter()
                            .map(|c| match c.layout.width {
                                ShrunkLength::Grow => 0,
                                ShrunkLength::GrowWithMin(l) => l,
                                ShrunkLength::Fixed(l) => l,
                            })
                            .sum::<u32>(),
                    )
                    .saturating_sub(self.layout.padding.left + self.layout.padding.right)
                    .saturating_sub(
                        self.layout.spacing * self.children.len().saturating_sub(1) as u32,
                    );
                let child_grow_number: u32 = self
                    .children
                    .iter()
                    .filter(|c| match c.layout.width {
                        ShrunkLength::Grow => true,
                        ShrunkLength::GrowWithMin(_) => true,
                        ShrunkLength::Fixed(_) => false,
                    })
                    .count()
                    .try_into()
                    .unwrap();

                self.children
                    .iter()
                    .map(|c| match c.layout.width {
                        ShrunkLength::Grow => remaining_length / child_grow_number,
                        ShrunkLength::GrowWithMin(l) => l + (remaining_length / child_grow_number),
                        ShrunkLength::Fixed(l) => l,
                    })
                    .collect()
            }
        };

        let new_children: Vec<_> = self
            .children
            .into_iter()
            .zip(new_children_widths)
            .map(|(c, width)| c.grow_width_pass(width))
            .collect();

        Node {
            layout: GrownWidthLayout {
                width: assigned_width,
                height: self.layout.height,
                padding: self.layout.padding,
                direction: self.layout.direction,
                spacing: self.layout.spacing,
            },
            children: new_children,
            element: self.element,
            name: self.name,
        }
    }
}

impl<Message> Node<Message, ShrinkHeightLayout> {
    /// Render pass 2/3
    /// top-down
    fn grow_height_pass(self, assigned_height: GrownLength) -> Node<Message, GrownHeightLayout> {
        let flow_cross_padding = self.layout.summed_padding();

        let child_grow_number: u32 = self
            .children
            .iter()
            .filter(|c| match self.layout.direction {
                LayoutDirection::Column => c.layout.height == ShrunkLength::Grow,
                LayoutDirection::Row => false,
            })
            .count()
            .try_into()
            .unwrap();

        let remaining_length = assigned_height
            .saturating_sub(
                self.children
                    .iter()
                    .map(|c| match self.layout.direction {
                        LayoutDirection::Column => match c.layout.height {
                            ShrunkLength::Grow => 0,
                            ShrunkLength::GrowWithMin(_) => todo!(), // probably copy width logic
                            ShrunkLength::Fixed(l) => l,
                        },
                        LayoutDirection::Row => c.layout.width,
                    })
                    .sum::<u32>(),
            )
            .saturating_sub(flow_cross_padding.0)
            .saturating_sub(self.layout.spacing * self.children.len().saturating_sub(1) as u32);
        let new_children: Vec<_> = self
            .children
            .into_iter()
            .map(|c| {
                let child_height = match (self.layout.direction, c.layout.height) {
                    (LayoutDirection::Column, ShrunkLength::Grow) => {
                        remaining_length / child_grow_number
                    }
                    (LayoutDirection::Row, ShrunkLength::Grow) => {
                        assigned_height - flow_cross_padding.1
                    }

                    (_, ShrunkLength::Fixed(l)) => l,
                    _ => todo!(), // probably copy width logic
                };

                c.grow_height_pass(child_height)
            })
            .collect();

        Node {
            layout: GrownHeightLayout {
                width: self.layout.width,
                height: assigned_height,
                padding: self.layout.padding,
                direction: self.layout.direction,
                spacing: self.layout.spacing,
            },
            children: new_children,
            element: self.element,
            name: self.name,
        }
    }
}
impl<Message> Node<Message, GrownHeightLayout> {
    /// Render pass 3/3
    /// top-down
    fn position_pass(self, parent_position: (u32, u32)) -> Node<Message, CalculatedLayout> {
        let first_child_position = (
            parent_position.0 + self.layout.padding.left,
            parent_position.1 + self.layout.padding.top,
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
            layout: CalculatedLayout::new(x, y, self.layout.width, self.layout.height),
            children: new_children,
            element: self.element,
            name: self.name,
        }
    }
}

impl CalculatedLayout {
    fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, pos: (u32, u32)) -> bool {
        pos.0 >= self.x && pos.0 < self.x + self.w && pos.1 >= self.y && pos.1 < self.y + self.h
    }
}
