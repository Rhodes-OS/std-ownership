use std_ownership_api::model::{Owner, Role};

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

impl Owner for Applier {
    #[inline]
    fn id(&self) -> u8 {
        self.id
    }
    #[inline]
    fn role(&self) -> Role {
        self.role
    }    
}