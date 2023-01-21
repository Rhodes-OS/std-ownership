use std_ownership_api::model::Role;
use std_ownership_api::model::Resource;
use crate::model::role::RoleEntry;
use strum::IntoEnumIterator;
use std::collections::HashMap;
use std::collections::hash_set::HashSet;
use std::sync::Mutex;
use std::io;

#[derive(Debug)]
pub struct ResourceContract<R, C> {
    resource: R,
    role_entries: HashMap<Role, RoleEntry<C>>, 
    lifecycle: HashMap<Role, Mutex<HashSet<u8>>>
}

impl<R, C> ResourceContract<R, C>
where
    R: Resource,
{
    #[must_use]
    pub fn new(resource: R) -> ResourceContract<R, C> {
        ResourceContract { 
            resource,
            role_entries: HashMap::with_capacity(8),
            lifecycle: {
                let mut lifecycle_map = HashMap::with_capacity(8);
                for role in Role::iter() {
                    lifecycle_map.insert(role, Mutex::new(HashSet::with_capacity(1000)));
                }
                lifecycle_map
            }
        }
    }

    #[inline]
    pub fn add_lifecycle_owner(&mut self, role: Role, owner_id: u8) -> bool {
        match self.lifecycle_owners(role) {
            Some(mutex_owners) => mutex_owners.lock().unwrap().insert(owner_id),
            None => false
        }
    }

    #[inline]
    pub fn contain_lifecycle_owner(&mut self, owner_id: u8, role: Role) -> bool {
        match self.lifecycle_owners(role) {
            Some(mutex_owners) => mutex_owners.get_mut().unwrap().contains(&owner_id),
            None => false
        }
    }

    #[inline]
    pub fn remove_lifecycle_owner(&mut self, owner_id: u8, role: Role) -> bool {
        match self.lifecycle_owners(role) {
            Some(mutex_owners) => mutex_owners.lock().unwrap().remove(&owner_id),
            None => false
        }
    }

    pub fn add_lifecycle(&mut self, role: Role) {
        self.lifecycle.insert(role, Mutex::new(HashSet::new()));
    }

    #[inline]
    pub fn lifecycle_owners(&mut self, role: Role) -> Option<&mut Mutex<HashSet<u8>>> {
        match self.lifecycle.get_mut(&role) {
            Some(role_entry_owners) => Some(role_entry_owners),
            _ => None
        }
    }

    pub fn add_role_entry(&mut self, role: Role, checkers: Vec<C>) {
        self.role_entries.insert(role, RoleEntry::new(role, checkers));
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

