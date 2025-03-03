pub trait Element {
    fn draw(&self, frame: &mut [u8], position: (u32, u32));
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
