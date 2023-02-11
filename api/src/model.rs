use strum_macros::EnumIter;

pub trait Resource {
    fn id(&self) -> u8;
    fn check(&self, buf: &[u8]) -> bool;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter)]
pub enum Role {
    SYS,
    OWNER,
    ACCESS,
    UNASSIGNED
}