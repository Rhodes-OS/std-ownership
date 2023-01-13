pub trait ResourceChecker {
    fn check(&self, res: u8) -> bool;
}

pub trait OwnerChecker {
    fn check(&self) -> bool;
}