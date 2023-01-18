pub trait Checker {
    fn check(&self, buf: &[u8]) -> bool;
}