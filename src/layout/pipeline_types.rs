use super::{
    length_types::{FlowCross, GrownLength, Length, ShrunkLength},
    Padding,
};

#[derive(Clone, Copy, Default, Debug)]
pub struct LayoutTemplate<Width, Height> {
    // about the node itself
    pub width: Width,
    pub height: Height,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum LayoutDirection {
    Column,
    #[default]
    Row,
    Stack,
}

pub type Layout = LayoutTemplate<Length, Length>;
pub type ShrinkWidthLayout = LayoutTemplate<ShrunkLength, Length>;
pub type GrownWidthLayout = LayoutTemplate<GrownLength, Length>;
pub type ShrinkHeightLayout = LayoutTemplate<GrownLength, ShrunkLength>;
pub type GrownHeightLayout = LayoutTemplate<GrownLength, GrownLength>;

impl<W, H> LayoutTemplate<W, H> {
    pub(super) fn summed_padding(&self) -> FlowCross {
        let width = self.padding.left + self.padding.right;
        let height = self.padding.top + self.padding.bottom;
        match self.direction {
            LayoutDirection::Column => FlowCross(height, width),
            LayoutDirection::Row | LayoutDirection::Stack => FlowCross(width, height),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct CalculatedLayout {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
