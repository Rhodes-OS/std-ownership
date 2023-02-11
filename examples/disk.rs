use std_ownership_api::model::Resource;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Disk {
    used_size: u64,
    size: u64
}

impl Disk {
    #[must_use]
    pub fn new(size: u64) -> Disk {
        Disk { used_size: 0, size }
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

impl Resource for Disk {
    #[inline]
    fn id(&self) -> u8 {
        2
    }

    #[inline]
    fn check(&self, data: &[u8]) -> bool {
        if self.size() <= self.used_size() + data.len() as u64 {
            return false;
        }
        true
    }
}