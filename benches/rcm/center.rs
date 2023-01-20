use std_ownership_api::model::Owner;
use std_ownership_api::model::Resource;
use std_ownership_api::model::Role;
use std_ownership_api::checker::Checker;
use std_ownership::rcm::center::ResourceCenter;
use criterion::Bencher;

pub fn bench_borrow<R, C>(b: &mut Bencher, rc: &mut ResourceCenter<R, C>) 
where
    R: Resource + Eq + Hash + Copy,
    C: Checker
{
    b.iter(|| {
        rc.borrow(MySQL{}, 1, "MySQL".as_bytes());
        rc.cancel_borrow(MySQL{}, 1);
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

use std::hash::Hash;
use std::str;

impl Checker for MySQL {
    #[inline]
    fn check(&self, table_name: &[u8]) -> bool {
        "MySQL" == str::from_utf8(table_name).unwrap()
    }
}
