pub struct LayoutEngineResult {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl LayoutEngineResult {
    pub fn test(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, pos: (u32, u32)) -> bool {
        pos.0 >= self.x && pos.0 < self.x + self.w && pos.1 >= self.y && pos.1 < self.y + self.h
    }
}
