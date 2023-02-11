#![deny(unsafe_op_in_unsafe_fn)]

use std_ownership_api::model::Resource;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Buffer<'a> {
    used_buf: &'a [u8],
    used_size: u64,
    size: u64
}

impl<'a> Buffer<'a> {
    #[must_use]
    pub fn new(size: u64) -> Buffer<'a> {
        Buffer {
            used_buf: &[0],
            used_size: 0, 
            size
        }
    }

    pub fn used_buf(&self) -> &[u8] {
        self.used_buf
    }
}

impl<'a> Resource for Buffer<'a> {
    #[inline]
    fn id(&self) -> u8 {
        1
    }
}