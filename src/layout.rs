use crate::tree::Node;

#[derive(Clone, Copy, Default)]
pub struct CalculatedLayout {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, Default)]
pub struct LayoutParameters {
    // about the node itself
    pub cross_length: Length,
    pub flow_length: Length,
    // about its children
    pub padding: Padding,
    pub direction: LayoutDirection,
    pub spacing: u32,
}

#[derive(Clone, Copy, Default)]
pub enum Length {
    #[default]
    Grow,
    Shrink,
    Fixed(u32),
}

#[derive(Clone, Copy, Default)]
pub enum LayoutDirection {
    #[default]
    Column,
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
impl Node {
    pub fn calculate_layout(&mut self) {
        let LayoutParameters {
            cross_length,
            flow_length,
            ..
            //_padding,
            //_direction,
            //_spacing,
        } = self.element.layout_parameters();
        self.layout = Some(CalculatedLayout::default());
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
