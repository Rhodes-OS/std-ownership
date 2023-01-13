#![deny(unsafe_op_in_unsafe_fn)]

use std_ownership_api::resource::Resource;
use std_ownership_api::checker::ResourceChecker;
use crate::resource::buffer::Buffer;

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
    fn check(&self, used_buffer_size: u8) -> bool {
        if self.buffer.size() >= used_buffer_size {
            return false;
        }
        true
    }
}