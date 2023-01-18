use std_ownership_api::model::Owner;
use std_ownership_api::model::Role;
use crate::rcm::contract::ResourceContract;
use std_ownership_api::checker::Checker;
use std_ownership_api::model::Resource;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct ResourceCenter<R, C> {
    resource_contracts: HashMap<R, ResourceContract<R, C>>
}

impl<R, C> ResourceCenter<R, C> 
where
    R: Resource + Eq + Hash + Copy,
    C: Checker
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            resource_contracts: HashMap::new()
        }
    }

    #[inline]
    pub fn borrow<O>(&mut self, applier: O, resource: R, buf: &[u8]) -> bool 
    where
        O: Owner,
    {
            self.borrow_exact_role(applier, resource, buf, Role::ACCESS)
    }

    #[inline]
    pub fn borrow_exact_role<O>(&mut self, applier: O, resource: R, buf: &[u8], role: Role) -> bool 
    where
        O: Owner
    {
        if applier.role() == Role::SYS {
            return true;
        }

        let contract = self.get_resource_contract(resource);
        if contract.contain_owner(applier.id(), role) {
            return true;
        }

        for role_entry in contract.role_entries().values() {
            for checker in role_entry.checkers() {
                if !checker.check(buf) {
                    return false;
                }
            }
        }

        contract.borrow(applier.id(), role);
        true
    }

    #[must_use]
    pub fn build(resource: R) -> ResourceCenter<R, C> {
        let mut rc = ResourceCenter::new();
        rc.resource_contracts.insert(resource, ResourceContract::new(resource));
        rc
    }

    pub fn init_owner_checkers(&mut self, applier_id: u8, resource: R, owner_checkers: Vec<C>) {
        let contract = self.get_resource_contract(resource);
        //init role checkers
        contract.add_role_entry(Role::OWNER, owner_checkers);
        //init role lifecycle
        contract.add_lifecycle(Role::OWNER);
        contract.borrow(applier_id, Role::OWNER);
    }

    #[inline]
    pub fn get_resource_contract(&mut self, resource: R) -> &mut ResourceContract<R, C> {
        match self.resource_contracts.get_mut(&resource) {
            Some(resource_contract) => resource_contract,
            _ => panic!("Not exist resource contract in rcm")
        }
    }

    #[inline]
    pub fn resource_contracts(&mut self) -> &mut HashMap<R, ResourceContract<R, C>> {
        &mut self.resource_contracts
    }    
}

