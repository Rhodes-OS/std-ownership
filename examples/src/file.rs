use std::u8;

use std_ownership_api::model::Resource;
use std_ownership_api::checker::Checker;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct File<'a> {
    name: &'a str,
    used_size: u64,
    size: u64
}

impl<'a> File<'a> {
    #[must_use]
    pub fn new(size: u64) -> File<'a> {
        File {
            name: "",
            used_size: 0, 
            size
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Resource for File<'a> {
    fn used_size(&self) -> u64 {
        self.used_size
    }

    fn size(&self) -> u64 {
        self.size
    }
}

impl<'a> Checker for File<'a> {
    #[inline]
    fn check(&self, buf: &[u8]) -> bool {
        if self.size <= self.used_size + buf.len() as u64 {
            return false;
        }
        true
    }
}