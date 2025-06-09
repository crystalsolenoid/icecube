#[derive(Clone, Copy, Debug)]
pub(super) struct FlowCross(pub u32, pub u32);
#[derive(Clone, Copy, Debug)]
pub(super) struct XY(pub u32, pub u32);

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub(super) enum ShrunkLength {
    #[default]
    Grow,
    Fixed(u32),
}

pub(super) type GrownLength = u32;

#[derive(Clone, Copy, Default, Debug)]
pub enum Length {
    #[default]
    Grow,
    Shrink,
    Fixed(u32),
}
