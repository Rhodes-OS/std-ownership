use super::role::Role;

pub struct Owner {
    id: u8,
    role: Role
}

impl Owner {
    #[must_use]
    pub fn new(id: u8, role: Role) -> Owner {
        Owner { id, role }
    }

    #[inline]
    pub fn id(&self) -> u8 {
        self.id
    }

    #[inline]
    pub fn role(&self) -> Role {
        self.role
    }
}