use std_ownership_api::checker::{ResourceChecker, OwnerChecker};
use std_ownership_api::model::Role;

#[derive(Hash, Eq, PartialEq)]
pub struct RoleEntry<RC, OC> {
    role: Role,
    resource_checkers: Vec<RC>,
    owner_checkers: Vec<OC>
}

impl<RC, OC> RoleEntry<RC, OC> 
where
    RC: ResourceChecker,
    OC: OwnerChecker
    // F: FnOnce() -> bool
{
    pub fn init_resource_checkers(&mut self, resource_checkers: Vec<RC>) {
        self.resource_checkers = resource_checkers;
    }

    pub fn init_owner_checkers(&mut self, owner_checkers: Vec<OC>) {
        self.owner_checkers = owner_checkers;
    }

    #[inline]
    pub fn register_resource_checker(&mut self, checker: RC) {
        self.resource_checkers.push(checker);
    }

    #[inline]
    pub fn register_owner_checker(&mut self, checker: OC) {
        self.owner_checkers.push(checker)
    }

    #[inline]
    pub fn resource_checkers(&self) -> &Vec<RC> {
        &self.resource_checkers
    }

    #[inline]
    pub fn owner_checkers(&self) -> &Vec<OC> {
        &self.owner_checkers
    }

    #[inline]
    pub fn role(&self) -> Role {
        self.role
    }
}

impl<RC, OC> Default for RoleEntry<RC, OC> {
    fn default() -> Self {
        Self {
            role: Role::OWNER,
            resource_checkers: vec![],
            owner_checkers: vec![]
        }
    }
}