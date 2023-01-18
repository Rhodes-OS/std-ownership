use crate::file::File;
use crate::mysql::MySQL;
use crate::unix::Unix;

use std_ownership::rcm::center::ResourceCenter;
use std_ownership_api::model::Owner;


pub fn main() {
    let file = File::new(1024);
    let mut rc = ResourceCenter::build(file);

    let owner_checkers = vec![file];
    let owner = Unix{};
    rc.init_owner_checkers(owner.id(), file, owner_checkers);

    let applier = MySQL{ file };
    rc.borrow(applier, file, "MySQL".as_bytes());
}

mod file;
mod mysql;
mod unix;