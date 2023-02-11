use std_ownership_api::model::Resource;

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

pub struct Applier;

impl Resource for Applier {
    #[inline]
    fn id(&self) -> u8 {
        100
    }

    #[inline]
    #[allow(unused_variables)]
    fn check(&self, buf: &[u8]) -> bool {
        true
    }
}