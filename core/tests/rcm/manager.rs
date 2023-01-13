use std_ownership::rcm::manager::Manager;
use std_ownership::rcm::checker::buffer_checker::BufferChecker;
use std_ownership::resource::buffer::Buffer;
use std_ownership::privilege::owner::Owner;
use std_ownership::privilege::role::Role;
use std_ownership::rcm::certificate::Certificate;
use std_ownership_api::checker::OwnerChecker;

#[test]
fn test_borrow() {
    let buffer = Buffer::new(8);
    let mut certificate: Certificate<Buffer, BufferChecker, MySQLChecker> = Certificate::new(1, Buffer::new(8));
    let cid = certificate.id();
    certificate.register_resource_checker(BufferChecker::new(buffer));
    certificate.register_owner_checker(MySQLChecker{});
    let applier = Owner::new(1, Role::WRITE);
    let mut rcm = Manager::new();
    rcm.push_certificate(certificate);
    assert_eq!(rcm.borrow(applier, cid, Role::SYS), false);
}

struct MySQLChecker;
impl OwnerChecker for MySQLChecker {
    #[inline]
    fn check(&self) -> bool {
        true
    }
}