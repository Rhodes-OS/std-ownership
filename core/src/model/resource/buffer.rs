#![deny(unsafe_op_in_unsafe_fn)]
use std_ownership_api::model::Resource;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Buffer {
    used_size: u64,
    size: u64
}

impl Buffer {
    #[must_use]
    pub fn new(size: u64) -> Buffer {
        Buffer { used_size: 0, size }
    }

    #[inline]
    pub fn used_size(&self) -> u64 {
        self.used_size
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.size
    }
}

impl Resource for Buffer {
    #[inline]
    fn id(&self) -> u8 {
        1
    }

    #[inline]
    fn check(&self, buf: &[u8]) -> bool {
        if self.size() <= self.used_size() + buf.len() as u64 {
            return false;
        }
        true
    }
}