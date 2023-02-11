use std_ownership_api::model::{Role, Resource};
use strum::IntoEnumIterator;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::collections::hash_set::HashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct ResourceContract<R> {
    resource: R,
    lifecycle: HashMap<Role, Arc<Mutex<HashSet<u8>>>>
}

impl<R> ResourceContract<R>
where
    R: Resource,
{
    #[cfg(not(no_global_oom_handling))]
    #[must_use]
    pub fn new(resource: R) -> ResourceContract<R> {
        ResourceContract { 
            resource,
            lifecycle: {
                let mut lifecycle_map = HashMap::with_capacity(8);
                for role in Role::iter() {
                    lifecycle_map.insert(role, Arc::new(Mutex::new(HashSet::with_capacity(1000))));
                }
                lifecycle_map
            }
        }
    }

    #[cfg(not(no_global_oom_handling))]
    pub fn add_owner_lifecycle(&mut self, owner_id: u8) {
        //init role lifecycle
        self.init_lifecycle(Role::OWNER);
        self.add_lifecycle_obj(Role::OWNER, owner_id);
    }

    #[inline]
    pub fn add_lifecycle_obj(&mut self, role: Role, owner_id: u8) -> bool {
        match self.lifecycle_objs(role) {
            Some(mutex_owners) => mutex_owners.lock().insert(owner_id),
            None => false
        }
    }

    #[inline]
    pub fn contain_lifecycle_obj(&mut self, owner_id: u8, role: Role) -> bool {
        match self.lifecycle_objs(role) {
            Some(mutex_owners) => mutex_owners.lock().contains(&owner_id),
            None => false
        }
    }

    #[inline]
    pub fn remove_lifecycle_obj(&mut self, owner_id: u8, role: Role) -> bool {
        match self.lifecycle_objs(role) {
            Some(mutex_owners) => mutex_owners.lock().remove(&owner_id),
            None => false
        }
    }

    pub fn init_lifecycle(&mut self, role: Role) {
        self.lifecycle.insert(role, Arc::new(Mutex::new(HashSet::with_capacity(1000))));
    }

    #[inline]
    pub fn lifecycle_objs(&mut self, role: Role) -> Option<&mut Arc<Mutex<HashSet<u8>>>> {
        match self.lifecycle.get_mut(&role) {
            Some(role_entry_owners) => Some(role_entry_owners),
            _ => None
        }
    }

    #[inline]
    pub fn resource(&self) -> &R {
        &self.resource
    }
}

