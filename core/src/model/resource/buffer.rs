#![deny(unsafe_op_in_unsafe_fn)]

use std_ownership_api::model::Resource;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Buffer {
    used_size: u64,
    size: u64
}

impl Buffer {
    #[must_use]
    pub fn new(size: u64) -> Buffer {
        Buffer {
            used_size: 0, 
            size
        }
    }
}

impl Resource for Buffer {
    #[inline]
    fn used_size(&self) -> u64 {
        self.used_size
    }

    #[inline]
    fn size(&self) -> u64 {
        self.size
    }
}