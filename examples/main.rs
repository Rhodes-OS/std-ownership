use crate::mysql::MySQL;
use crate::jvm::JVM;
use std_ownership::rcm::center::ResourceCenter;
use std_ownership::rcm::contract::ResourceContract;
use std_ownership_api::model::Resource;

pub fn main() {
    let mut rc = ResourceCenter::<MySQL>::new();

    let mysql = MySQL::new();

    let mut res_contract = ResourceContract::new(mysql);
    res_contract.add_owner_lifecycle(mysql.id());
    rc.register(res_contract);

    let applier = JVM{};
    assert_eq!(false, rc.borrow(applier, 3, "mysql.ibd".as_bytes()));
}

mod disk;
mod mysql;
mod jvm;