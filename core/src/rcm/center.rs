use std_ownership_api::model::Role;
use crate::rcm::contract::ResourceContract;
use std_ownership_api::model::Resource;
use crossbeam::utils::CachePadded;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct ResourceCenter<R> {
    resource_contracts: HashMap<u8, CachePadded<ResourceContract<R>>>
}

type ResContractMap<R> = HashMap<u8, CachePadded<ResourceContract<R>>>;

impl<R> ResourceCenter<R> 
where
    R: Resource
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
        O: Resource,
    {
            self.borrow_exact_role(applier, resource_id, buf, Role::ACCESS)
    }

    #[inline]
    pub fn borrow_exact_role<O>(&mut self, applier: O, resource_id: u8, buf: &[u8], role: Role) -> bool 
    where
        O: Resource
    {
        let contract = self.get_resource_contract(resource_id).unwrap();
        if contract.contain_lifecycle_obj(applier.id(), role) {
            return true;
        }

        if !contract.resource().check(buf) {
            return false;
        }

        contract.add_lifecycle_obj(role, applier.id())
    }

    #[inline]
    pub fn free<O>(&mut self, applier: O, resource_id: u8) 
    where
        O: Resource
    {
        self.free_exact_role(applier, resource_id, Role::ACCESS)
    }

    #[inline]
    pub fn free_exact_role<O>(&mut self, applier: O, resource_id: u8, role: Role) 
    where
        O: Resource
    {
        let contract = self.get_resource_contract(resource_id).unwrap();
        contract.remove_lifecycle_obj(applier.id(), role);
    }

    pub fn register(&mut self, resource_contract: ResourceContract<R>) {
        self.resource_contracts.insert(resource_contract.resource().id(), CachePadded::new(resource_contract));
    }

    #[inline]
    pub fn get_resource_contract(&mut self, resource_id: u8) -> io::Result<&mut CachePadded<ResourceContract<R>>> {
        match self.resource_contracts.get_mut(&resource_id) {
            Some(resource_contract) => Ok(resource_contract),
            _ => Err(io::Error::new(io::ErrorKind::NotFound, "Not exist resource contract in rcm"))
        }
    }

    #[inline]
    pub fn resource_contracts(&mut self) -> &mut ResContractMap<R> {
        &mut self.resource_contracts
    }    
}