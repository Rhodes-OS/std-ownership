#![deny(unsafe_op_in_unsafe_fn)]

use std_ownership_api::model::Resource;
use std_ownership_api::checker::ResourceChecker;
use crate::model::resource::buffer::Buffer;

pub struct BufferChecker {
    buffer: Buffer,
}

impl BufferChecker {
    #[must_use]
    pub fn new(buffer: Buffer) -> BufferChecker {
        BufferChecker { buffer }
    }

    #[must_use]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
}

impl ResourceChecker for BufferChecker {
    #[inline]
    fn check(&self) -> bool {
        if self.buffer.size() >= self.buffer.used_size() {
            return false;
        }
        true
    }
}