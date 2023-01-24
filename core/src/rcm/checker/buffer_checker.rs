#![deny(unsafe_op_in_unsafe_fn)]

use std_ownership_api::model::Resource;
use std_ownership_api::checker::Checker;
use crate::model::resource::buffer::Buffer;

#[derive(Debug, Clone, Copy)]
pub struct BufferChecker<'a> {
    buffer: Buffer<'a>,
}

impl<'a> BufferChecker<'a> {
    #[must_use]
    pub fn new(buffer: Buffer) -> BufferChecker {
        BufferChecker { buffer }
    }

    #[must_use]
    pub fn buffer(&self) -> &Buffer<'a> {
        &self.buffer
    }
}

impl<'a> Checker for BufferChecker<'a> {
    #[inline]
    fn check(&self, buf: &[u8]) -> bool {
        if self.buffer.size() <= self.buffer.used_size() + buf.len() as u64 {
            return false;
        }
        true
    }
}