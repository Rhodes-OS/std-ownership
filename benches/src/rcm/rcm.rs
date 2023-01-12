use std_ownership::rcm::rcm::RCM;
use std_ownership::rcm::checker::buffer_checker::BufferChecker;
use std_ownership::resource::buffer::Buffer;
use std_ownership::privilege::owner::Owner;
use std_ownership::privilege::role::Role;
use std_ownership::rcm::certificate::Certificate;
use std_ownership::api::OwnerChecker;
use criterion::Bencher;

pub fn bench_check(b: &mut Bencher) {
    b.iter(|| {
        let buffer = Buffer::new(8);
        let mut certificate: Certificate<Buffer, BufferChecker, MySQLChecker> = Certificate::new(1, Buffer::new(8));
        let cid = certificate.id();
        certificate.register_resource_checker(BufferChecker::new(buffer));
        certificate.register_owner_checker(MySQLChecker{});
        let applier = Owner::new(1, Role::WRITE);
        let mut rcm = RCM::new();
        rcm.push_certificate(certificate);
        rcm.borrow(applier, cid, Role::SYS);
    });
}

struct MySQLChecker;
impl OwnerChecker for MySQLChecker {
    #[inline]
    fn check(&self) -> bool {
        true
    }
}  
