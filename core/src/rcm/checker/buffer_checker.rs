#![deny(unsafe_op_in_unsafe_fn)]

use crate::api::ResourceChecker;
use crate::resource::buffer::Buffer;
use crate::api::Resource;

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