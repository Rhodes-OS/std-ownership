use std::str;
use std_ownership_api::model::{Owner, Role};
use std_ownership_api::checker::Checker;

use crate::file::File;

pub struct MySQL<'a> {
    pub(crate) file: File<'a>
}

impl<'a> MySQL<'a> {
    fn file(&self) -> File {
        self.file
    }
}

impl<'a> Owner for MySQL<'a> {
    fn id(&self) -> u8 {
        1
    }

    fn role(&self) -> Role {
        Role::UNASSIGNED
    }
}

impl<'a> Checker for MySQL<'a> {
    #[inline]
    fn check(&self, table_name: &[u8]) -> bool {
        self.file().name() != str::from_utf8(table_name).unwrap()
    }
}