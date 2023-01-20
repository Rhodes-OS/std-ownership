pub trait Resource {
    fn id(&self) -> u8;
    fn used_size(&self) -> u64;
    fn size(&self) -> u64;
}
pub trait Owner {
    fn id(&self) -> u8;
    fn role(&self) -> Role;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Role {
    SYS,
    OWNER,
    ACCESS,
    UNASSIGNED
}