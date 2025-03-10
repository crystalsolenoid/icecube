pub trait Element {
    fn draw(&self, frame: &mut [u8], position: (u32, u32));
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn padding(&self) -> Padding;
}

#[derive(Clone, Copy, Default)]
pub struct Padding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
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
