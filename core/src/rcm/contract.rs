use std_ownership_api::model::Role;
use std_ownership_api::model::Resource;
use crate::model::role::RoleEntry;
use crate::util::collection;
use std::vec::Vec;
use std::collections::HashMap;
use std::sync::Mutex;
use std::io;

#[derive(Debug)]
pub struct ResourceContract<R, C> {
    resource: R,
    role_entries: HashMap<Role, RoleEntry<C>>, 
    lifecycle: HashMap<Role, Mutex<Vec<u8>>>
}

impl<R, C> ResourceContract<R, C>
where
    R: Resource,
{
    #[must_use]
    pub fn new(resource: R) -> ResourceContract<R, C> {
        ResourceContract { 
            resource,
            role_entries: HashMap::new(),
            lifecycle: HashMap::new()
        }
    }

    #[inline]
    pub fn borrow(&mut self, owner_id: u8, role: Role) -> io::Result<bool> {
        match self.role_entry_owners(role) {
            Some(mutex_owners) => { mutex_owners.lock().unwrap().push(owner_id); Ok(true) },
            None => Err(io::Error::new(io::ErrorKind::WouldBlock, "Borrow error"))
        }
        
    }

    pub fn add_role_entry(&mut self, role: Role, checkers: Vec<C>) {
        self.role_entries.insert(role, RoleEntry::new(role, checkers));
    }

    pub fn add_lifecycle(&mut self, role: Role) {
        self.lifecycle.insert(role, Mutex::new(Vec::new()));
    }

    #[inline]
    pub fn contain_owner(&mut self, owner_id: u8, role: Role) -> bool {
        match self.role_entry_owners(role) {
            Some(mutex_owners) => mutex_owners.get_mut().unwrap().contains(&owner_id),
            None => false
        }
    }

    #[inline]
    pub fn remove_owner(&mut self, owner_id: u8, role: Role) -> u8 {
        match self.role_entry_owners(role) {
            Some(mutex_owners) => collection::remove_element_in_vec(mutex_owners.lock().unwrap().to_vec(), owner_id),
            None => 0
        }
        
    }

    #[inline]
    pub fn role_entry_owners(&mut self, role: Role) -> Option<&mut Mutex<Vec<u8>>> {
        match self.lifecycle.get_mut(&role) {
            Some(role_entry_owners) => Some(role_entry_owners),
            _ => None
        }
    }

    #[inline]
    pub fn role_entry(&mut self, role: Role) -> io::Result<&mut RoleEntry<C>> {
        match self.role_entries.get_mut(&role) {
            Some(role_entry) => Ok(role_entry),
            _ => Err(io::Error::new(io::ErrorKind::NotFound, "Not exist role entry"))
        }
    }

    #[inline]
    pub fn role_entries(&self) -> &HashMap<Role, RoleEntry<C>> {
        &self.role_entries
    }

    #[inline]
    pub fn resource(&self) -> &R {
        &self.resource
    }
}

