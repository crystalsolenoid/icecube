use std::ops::{Index, IndexMut};

pub struct Buffer<'a> {
    pub data: &'a mut [u8],
    pub width: usize,
}

impl<'a> Buffer<'a> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

//TODO: Consider implementing Index for range<usize>
impl<'a> IndexMut<usize> for Buffer<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<'a> Index<usize> for Buffer<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
