use std_ownership_api::model::Owner;
use std_ownership_api::model::Role;
use crate::rcm::contract::ResourceContract;
use std_ownership_api::checker::Checker;
use std_ownership_api::model::Resource;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct ResourceCenter<R, C> {
    resource_contracts: HashMap<u8, ResourceContract<R, C>>
}

impl<R, C> ResourceCenter<R, C> 
where
    R: Resource + Copy,
    C: Checker
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            resource_contracts: HashMap::with_capacity(1000)
        }
    }

    #[inline]
    pub fn borrow<O>(&mut self, applier: O, resource_id: u8, buf: &[u8]) -> bool 
    where
        O: Owner,
    {
            self.borrow_exact_role(applier, resource_id, buf, Role::ACCESS)
    }

    #[inline]
    pub fn borrow_exact_role<O>(&mut self, applier: O, resource_id: u8, buf: &[u8], role: Role) -> bool 
    where
        O: Owner
    {
        if applier.role() == Role::SYS {
            return true;
        }

        let contract = self.get_resource_contract(resource_id).unwrap();
        if contract.contain_lifecycle_owner(applier.id(), role) {
            return true;
        }

        for role_entry in contract.role_entries().values() {
            for checker in role_entry.checkers() {
                if !checker.check(buf) {
                    return false;
                }
            }
        }

        contract.borrow(applier.id(), role).unwrap()
    }

    #[inline]
    pub fn cancel_borrow<O>(&mut self, applier: O, resource_id: u8) 
    where
        O: Owner
    {
        self.cancel_exact_role_borrow(applier, resource_id, Role::ACCESS)
    }

    #[inline]
    pub fn cancel_exact_role_borrow<O>(&mut self, applier: O, resource_id: u8, role: Role) 
    where
        O: Owner
    {
        let contract = self.get_resource_contract(resource_id).unwrap();
        contract.remove_lifecycle_owner(applier.id(), role);
    }

    #[must_use]
    pub fn builder() -> ResourceCenter<R, C> {
        ResourceCenter::new()
    }

    pub fn build_owner_checkers(&mut self, applier_id: u8, resource: R, owner_checkers: Vec<C>) {
        let mut contract = ResourceContract::new(resource);

        //init role checkers
        contract.add_role_entry(Role::OWNER, owner_checkers);
        //init role lifecycle
        contract.add_lifecycle(Role::OWNER);
        let borrow_state = contract.borrow(applier_id, Role::OWNER).unwrap();

        if borrow_state {
            self.resource_contracts.insert(resource.id(), contract);
        }
    }

    #[inline]
    pub fn get_resource_contract(&mut self, resource_id: u8) -> io::Result<&mut ResourceContract<R, C>> {
        match self.resource_contracts.get_mut(&resource_id) {
            Some(resource_contract) => Ok(resource_contract),
            _ => Err(io::Error::new(io::ErrorKind::NotFound, "Not exist resource contract in rcm"))
        }
    }

    #[inline]
    pub fn resource_contracts(&mut self) -> &mut HashMap<u8, ResourceContract<R, C>> {
        &mut self.resource_contracts
    }    
}