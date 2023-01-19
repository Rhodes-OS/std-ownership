use std_ownership_api::model::{Owner, Role};

pub struct Unix;

impl Owner for Unix {
    fn id(&self) -> u8 {
        0
    }

    fn role(&self) -> Role {
        Role::UNASSIGNED
    }
}