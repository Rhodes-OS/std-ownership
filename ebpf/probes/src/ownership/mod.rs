use std_ownership_api::model::{Role, Resource};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BorrowResult {
    applier: u8,
    state: bool,
}

impl BorrowResult {
    pub fn new(applier: u8, state: bool) -> BorrowResult {
        BorrowResult { applier, state }
    }
}

pub struct Applier {
    id: u8,
    role: Role
}

impl Applier {
    #[must_use]
    pub fn new(id: u8, role: Role) -> Applier {
        Applier { id, role }
    }
}

impl Resource for Applier {
    #[inline]
    fn id(&self) -> u8 {
        self.id
    }

    #[inline]
    #[allow(unused_variables)]
    fn check(&self, buf: &[u8]) -> bool {
        true
    }
}