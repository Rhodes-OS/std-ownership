use strum_macros::EnumIter;

pub trait Resource {
    fn id(&self) -> u8;
}
pub trait Owner {
    fn id(&self) -> u8;
    fn role(&self) -> Role;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter)]
pub enum Role {
    SYS,
    OWNER,
    ACCESS,
    UNASSIGNED
}