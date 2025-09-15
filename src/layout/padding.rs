#[derive(Clone, Copy, Default, Debug)]
pub struct Padding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
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
