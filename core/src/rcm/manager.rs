use crate::privilege::owner::Owner;
use crate::privilege::role::Role;
use crate::rcm::certificate::Certificate;
use std_ownership_api::checker::{ResourceChecker, OwnerChecker};
use std_ownership_api::resource::Resource;
use std::collections::HashMap;

pub struct Manager<R, RC, OC> {
    certificates: HashMap<u8, Certificate<R, RC, OC>>
}

impl<R, RC, OC> Manager<R, RC, OC> 
where
    R: Resource,
    RC: ResourceChecker,
    OC: OwnerChecker,
{
    #[must_use]
    pub fn new() -> Manager<R, RC, OC> {
        Manager { 
            certificates: HashMap::new(),
        }
    }

    #[inline]
    pub fn borrow(&mut self, applier: Owner, certificate_id: u8, role: Role) -> bool {
        if applier.role() == Role::SYS {
            return true;
        }

        let certificate = self.get_certificate(certificate_id);
        if certificate.contain_owner(applier.id(), role) {
            return true;
        }

        let size = certificate.res().size();
        for checker in certificate.resource_checkers() {
            if !checker.check(size) {
                return false;
            }
        }
        for checker in certificate.owner_checkers() {
            if !checker.check() {
                return false;
            }
        }

        certificate.borrow(applier.id(), role);
        true
    }

    #[inline]
    pub fn push_certificate(&mut self, certificate: Certificate<R, RC, OC>) {
        self.certificates.insert(certificate.id(), certificate);
    }

    #[inline]
    pub fn get_certificate(&mut self, certificate_id: u8) -> &mut Certificate<R, RC, OC> {
        match self.certificates.get_mut(&certificate_id) {
            Some(certificate) => certificate,
            _ => panic!("Not exist certificate in rcm")
        }
    }
}

