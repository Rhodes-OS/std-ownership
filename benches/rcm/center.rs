use std_ownership_api::model::Owner;
use std_ownership_api::model::Role;
use std_ownership_api::checker::Checker;
use std_ownership::model::resource::buffer::Buffer;
use std_ownership::rcm::checker::buffer_checker::BufferChecker;
use std_ownership::rcm::center::ResourceCenter;
use criterion::Bencher;

pub fn bench_borrow(b: &mut Bencher) {
    b.iter(|| {
        let buffer = Buffer::new(1024);
        let mut rc = ResourceCenter::build(buffer);

        let owner_checkers = vec![BufferChecker::new(buffer)];
        rc.init_owner_checkers(0, buffer, owner_checkers);

        let applier = MySQL{};
        rc.borrow(applier, buffer, "MySQL".as_bytes());
    });
}

pub struct MySQL;

impl Owner for MySQL {
    fn id(&self) -> u8 {
        1
    }

    fn role(&self) -> Role {
        Role::UNASSIGNED
    }
}

use std::str;

impl Checker for MySQL {
    #[inline]
    fn check(&self, table_name: &[u8]) -> bool {
        "MySQL" == str::from_utf8(table_name).unwrap()
    }
}
