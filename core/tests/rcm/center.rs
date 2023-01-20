use std_ownership::rcm::center::ResourceCenter;
use std_ownership::model::resource::buffer::Buffer;
use std_ownership::rcm::checker::buffer_checker::BufferChecker;
use std_ownership_api::model::Owner;
use std_ownership_api::model::Role;
use std_ownership_api::checker::Checker;

#[test]
fn test_borrow() {
    let buffer = Buffer::new(1024);

    let owner_checkers = vec![BufferChecker::new(Buffer::new(1024))];
    let mut rc = ResourceCenter::builder();
    rc.build_owner_checkers(0, buffer, owner_checkers);

    let applier = MySQL{};
    assert_eq!(rc.borrow(applier, 1, "MySQL".as_bytes()), false);
}

struct MySQL;

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
        "MySQL" != str::from_utf8(table_name).unwrap()
    }
}