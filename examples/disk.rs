use std_ownership_api::model::Resource;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Disk {
    used_capacity: u64,
    capacity: u64
}

impl Disk {
    #[must_use]
    pub fn new(capacity: u64) -> Disk {
        Disk { used_capacity: 0, capacity }
    }

    #[inline]
    pub fn used_capacity(&self) -> u64 {
        self.used_capacity
    }

    #[inline]
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
}

impl Resource for Disk {
    #[inline]
    fn id(&self) -> u8 {
        2
    }

    #[inline]
    fn check(&self, data: &[u8]) -> bool {
        if self.capacity() <= self.used_capacity() + data.len() as u64 {
            return false;
        }
        true
    }
}