use std_ownership_api::model::Role;
use std_ownership_api::model::Resource;
use crate::util::collection;
use std::vec::Vec;
use std::collections::HashMap;

pub struct ResourceLifecycle<R> {
    resource: R,
    role_entry_owners_map: HashMap<Role, Vec<u8>>
}

impl<R> ResourceLifecycle<R>
where
    R: Resource,
{
    #[must_use]
    pub fn new(resource: R) -> ResourceLifecycle<R> {
        Self { 
            resource,
            role_entry_owners_map: HashMap::new()
        }
    }

    pub fn init(resource: R) -> Self {
        let mut lifecycle = Self::new(resource);
        lifecycle.role_entry_owners_map.insert(Role::default(), Vec::new());
        lifecycle
    }

    #[inline]
    pub fn borrow(&mut self, owner_id: u8, role: Role) {
        self.role_entry_owners(role).to_vec().push(owner_id)
    }

    #[inline]
    pub fn contain_owner(&mut self, owner_id: u8, role: Role) -> bool {
        self.role_entry_owners(role).contains(&owner_id)
    }

    #[inline]
    pub fn remove_owner(&mut self, owner_id: u8, role: Role) -> u8 {
        collection::remove_element_in_vec(self.role_entry_owners(role).to_vec(), owner_id)
    }

    #[inline]
    pub fn role_entry_owners(&mut self, role: Role) -> &mut Vec<u8> {
        match self.role_entry_owners_map.get_mut(&role) {
            Some(role_entry_owners) => role_entry_owners,
            _ => panic!("Not exist owners in the role entry")
        }
    }

    #[inline]
    pub fn role_entry_owners_map(&self) -> &HashMap<Role, Vec<u8>> {
        &self.role_entry_owners_map
    }

    #[inline]
    pub fn resource(&self) -> &R {
        &self.resource
    }
}

