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

/*
<Length, Length>
self.shrink_width_pass()
<ShrunkLength, Length>
    .grow_width_pass(root_size.0)
<GrownLength, Length>
    .wrap()
<GrownLength, Length>
    .shrink_height_pass()
<GrownLength, ShrunkLength>
    .grow_height_pass(root_size.1)
<GrownLength, GrownLength>
    .position_pass((0, 0))
(u32, u32)
*/
