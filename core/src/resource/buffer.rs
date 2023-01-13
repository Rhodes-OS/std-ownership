#![deny(unsafe_op_in_unsafe_fn)]

use std_ownership_api::resource::Resource;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buffer {
    size: u8
}

impl Buffer {
    #[must_use]
    pub fn new(size: u8) -> Buffer {
        Buffer { size: size }
    }
}

impl Resource for Buffer {
    #[inline]
    fn size(&self) -> u8 {
        self.size
    }
}