use std_ownership_api::model::Owner;
use std_ownership_api::model::Role;
use crate::model::role::RoleEntry;
use crate::rcm::lifecycle::ResourceLifecycle;
use std_ownership_api::checker::{ResourceChecker, OwnerChecker};
use std_ownership_api::model::Resource;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Manager<R, RC, OC> {
    role_entry: HashMap<Role, RoleEntry<RC, OC>>,
    resource_lifecycles: HashMap<R, ResourceLifecycle<R>>
}

impl<R, RC, OC> Manager<R, RC, OC> 
where
    R: Resource + Eq + Hash + Copy,
    RC: ResourceChecker,
    OC: OwnerChecker
{
    #[must_use]
    pub fn new() -> Self {
        Self { 
            role_entry: HashMap::new(), 
            resource_lifecycles: HashMap::new() 
        }
    }

    #[inline]
    pub fn borrow<O>(&mut self, applier: O, resource: R, buf: &[u8]) -> bool 
    where
        O: Owner
    {
            self.borrow_exact_role(applier, resource, buf, Role::ACCESS)
    }

    #[inline]
    pub fn borrow_exact_role<O>(&mut self, applier: O, resource: R, buf: &[u8], role: Role) -> bool 
    where
        O: Owner,
    {
        if applier.role() == Role::SYS {
            return true;
        }

        if self.get_resource_lifecycle(resource).contain_owner(applier.id(), role) {
            return true;
        }
  
        for checker in self.default_role_entry().resource_checkers() {
            if !checker.check() {
                return false;
            }
        }
        for checker in self.default_role_entry().owner_checkers() {
            if !checker.check(buf) {
                return false;
            }
        }

        self.get_resource_lifecycle(resource).borrow(applier.id(), role);
        true
    }

    pub fn init_resource_checkers(resource: R, resource_checkers: Vec<RC>) -> Self {
        //init resource checkers
        let mut role_entry = RoleEntry::default();
        role_entry.init_resource_checkers(resource_checkers);
        //init resource lifecycle
        let resource_lifecycle = ResourceLifecycle::init(resource);
        //init manager
        let mut manager = Manager::new();
        manager.role_entry.insert(Role::default(), role_entry);
        manager.resource_lifecycles.insert(resource, resource_lifecycle);
        manager
    }

    pub fn init_owner_checkers(&mut self, applier_id: u8, resource: R, owner_checkers: Vec<OC>) {
        self.default_role_entry().init_owner_checkers(owner_checkers);
        self.get_resource_lifecycle(resource).role_entry_owners(Role::default()).push(applier_id);
    }

    #[inline]
    pub fn default_role_entry(&mut self) -> &mut RoleEntry<RC, OC> {
        self.role_entry(Role::default())
    }

    #[inline]
    pub fn role_entry(&mut self, role: Role) -> &mut RoleEntry<RC, OC> {
        match self.role_entry.get_mut(&role) {
            Some(role_entry) => role_entry,
            _ => panic!("Not exist role entry in rcm")
        }
    }

    #[inline]
    pub fn get_resource_lifecycle(&mut self, resource: R) -> &mut ResourceLifecycle<R> {
        match self.resource_lifecycles.get_mut(&resource) {
            Some(resource_lifecycle) => resource_lifecycle,
            _ => panic!("Not exist resource lifecycle in rcm")
        }
    }
}

