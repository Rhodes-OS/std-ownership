use crate::privilege::role::Role;
use std_ownership_api::resource::Resource;
use std_ownership_api::checker::{ResourceChecker, OwnerChecker};
use crate::util::collection;
use std::vec::Vec;

pub struct Certificate<R, RC, OC> {
    id: u8,
    res: R,
    read_owners: Vec<u8>,
    write_owners: Vec<u8>,
    sys_owners: Vec<u8>,
    resource_checkers: Vec<RC>,
    owner_checkers: Vec<OC>
}

impl<R, RC, OC> Certificate<R, RC, OC>
where
    R: Resource,
    RC: ResourceChecker,
    OC: OwnerChecker,
{
    #[must_use]
    pub fn new(id: u8, res: R) -> Certificate<R, RC, OC> {
        Self { 
            id, 
            res,
            read_owners: Vec::new(), 
            write_owners: Vec::new(),
            sys_owners: Vec::new(),
            resource_checkers: Vec::new(),
            owner_checkers: Vec::new()
        }
    }

    #[inline]
    pub fn borrow(&mut self, owner_id: u8, role: Role) {
        match role {
            Role::READ => self.read_owners.push(owner_id),
            Role::WRITE => self.write_owners.push(owner_id),
            Role::SYS => self.sys_owners.push(owner_id)
        }
    }

    pub fn register_resource_checker(&mut self, checker: RC) {
        self.resource_checkers.push(checker);
    }

    pub fn register_owner_checker(&mut self, checker: OC) {
        self.owner_checkers.push(checker)
    }

    #[inline]
    pub fn contain_owner(&self, owner_id: u8, role: Role) -> bool {
        match role {
            Role::SYS => self.sys_owners.contains(&owner_id),
            Role::READ => self.read_owners.contains(&owner_id),
            Role::WRITE => self.write_owners.contains(&owner_id)
        }
    }

    #[inline]
    pub fn remove_owner(&mut self, owner_id: u8, role: Role) -> u8 {
        match role {
            Role::SYS => collection::remove_element_in_vec(self.sys_owners.clone(), owner_id),
            Role::READ => collection::remove_element_in_vec(self.read_owners.clone(), owner_id),
            Role::WRITE => collection::remove_element_in_vec(self.write_owners.clone(), owner_id)
        }
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
    pub fn add_read_owner(&mut self, owner_id: u8) {
        self.read_owners.push(owner_id)
    }

    #[inline]
    pub fn add_write_owner(&mut self, owner_id: u8) {
        self.write_owners.push(owner_id)
    }

    #[inline]
    pub fn add_sys_owner(&mut self, owner_id: u8) {
        self.sys_owners.push(owner_id)
    }

    #[inline]
    pub fn res(&self) -> &R {
        &self.res
    }

    #[inline]
    pub fn id(&self) -> u8 {
        self.id
    }
}

