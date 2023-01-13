#[repr(C)]
#[derive(Debug, Clone)]
pub struct BorrowResult {
    applier: str,
    state: bool,
}

impl BorrowResult {
    pub fn new(&self, applier: str, state: bool) -> BorrowResult {
        BorrowResult { applier, state }
    }
}