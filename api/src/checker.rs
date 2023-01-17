pub trait ResourceChecker {
    fn check(&self) -> bool;
}

pub trait OwnerChecker {
    fn check(&self, buf: &[u8]) -> bool;
}