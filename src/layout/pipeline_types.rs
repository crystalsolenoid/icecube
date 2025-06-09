use super::{
    length_types::{FlowCross, GrownLength, Length, ShrunkLength, XY},
    Padding,
};

#[derive(Clone, Copy, Default)]
pub struct LayoutTemplate<Width, Height> {
    // about the node itself
    pub width: Width,
    pub height: Height,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum LayoutDirection {
    Column,
    #[default]
    Row,
}

pub type Layout = LayoutTemplate<Length, Length>;
pub type ShrinkWidthLayout = LayoutTemplate<ShrunkLength, Length>;
pub type ShrinkHeightLayout = LayoutTemplate<ShrunkLength, ShrunkLength>;
pub type GrownLayout = LayoutTemplate<GrownLength, GrownLength>;

impl<W, H> LayoutTemplate<W, H> {
    pub(super) fn summed_padding(&self) -> FlowCross {
        let width = self.padding.left + self.padding.right;
        let height = self.padding.top + self.padding.bottom;
        match self.direction {
            LayoutDirection::Column => FlowCross(height, width),
            LayoutDirection::Row => FlowCross(width, height),
        }
    }
    pub(super) fn xy_to_flow_cross(&self, xy: XY) -> FlowCross {
        let XY(x, y) = xy;
        match self.direction {
            LayoutDirection::Column => FlowCross(y, x),
            LayoutDirection::Row => FlowCross(x, y),
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
