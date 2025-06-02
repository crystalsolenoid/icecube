use super::{GrownLength, LayoutDirection, Length, Padding, ShrunkLength};

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

#[derive(Clone, Copy, Default, Debug)]
pub struct CalculatedLayout {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub(super) struct ShrinkLayout {
    // about the node itself
    pub width: ShrunkLength,
    pub height: ShrunkLength,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}

pub struct ShrinkHeightLayout {
    // about the node itself
    pub width: ShrunkLength,
    pub height: ShrunkLength,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}

pub struct GrownLayout {
    // about the node itself
    pub width: GrownLength,
    pub height: GrownLength,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}
