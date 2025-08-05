//! Buffer Management (dummy)

pub struct Buffer(Vec<u8>);

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer(vec![0; size])
    }
}
