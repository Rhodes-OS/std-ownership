use std_ownership_api::model::Role;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RoleEntry<C> {
    role: Role,
    checkers: Vec<C>
}

impl<C> RoleEntry<C> {
    #[must_use]
    pub fn new(role: Role, checkers: Vec<C>) -> RoleEntry<C> {
        RoleEntry {
            role,
            checkers
        }
    }

    pub fn init_checkers(&mut self, checkers: Vec<C>) {
        self.checkers = checkers;
    }

    #[inline]
    pub fn register_checker(&mut self, checker: C) {
        self.checkers.push(checker);
    }

    #[inline]
    pub fn checkers(&self) -> &Vec<C> {
        &self.checkers
    }

    #[inline]
    pub fn role(&self) -> Role {
        self.role
    }
}

impl<C> Default for RoleEntry<C> {
    fn default() -> Self {
        Self {
            role: Role::OWNER,
            checkers: vec![]
        }
    }
}