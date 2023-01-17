use std_ownership::rcm::manager::Manager;
use std_ownership::rcm::checker::buffer_checker::BufferChecker;
use std_ownership::model::resource::buffer::Buffer;
use std_ownership_api::model::Owner;
use std_ownership_api::model::Role;
use std_ownership_api::checker::OwnerChecker;
use criterion::Bencher;

pub fn bench_borrow(b: &mut Bencher) {
    b.iter(|| {
        let buffer = Buffer::new(8);

        let mut resource_checkers = vec![];
        resource_checkers.push(BufferChecker::new(buffer));
        
        let mut rcm = Manager::init_resource_checkers(buffer, resource_checkers);

        let mut owner_checkers = vec![];
        let applier = MySQL{};
        owner_checkers.push(MySQL{});
        rcm.init_owner_checkers(applier.id(), buffer, owner_checkers);
        
        rcm.borrow(applier, buffer, "MySQL".as_bytes());
    });
}

struct MySQL;

impl Owner for MySQL {

    fn id(&self) -> u8 {
        1
    }

    fn role(&self) -> Role {
        panic!()
    }
}

use std::str;

impl OwnerChecker for MySQL {
    #[inline]
    fn check(&self, table_name: &[u8]) -> bool {
        "MySQL" == str::from_utf8(table_name).unwrap()
    }
}
