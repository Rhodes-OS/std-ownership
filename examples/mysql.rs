use std::str;
use std_ownership_api::model::Resource;

use crate::disk::Disk;

#[derive(Debug, Clone, Copy)]
pub struct MySQL<'a> {
    sys_files: &'a str,
    disk: Disk
}

impl<'a> MySQL<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self { 
            sys_files: "mysql.ibd",
            disk: Disk::new(1024)
        }
    }

    #[inline]
    pub fn sys_files(&self) -> &'a str {
        &self.sys_files
    }

    #[inline]
    pub fn disk(&self) -> Disk {
        self.disk
    }
}

impl<'a> Resource for MySQL<'a> {
    #[inline]
    fn id(&self) -> u8 {
        3
    }

    #[inline]
    fn check(&self, table_name: &[u8]) -> bool {
        if !self.disk().check(table_name) {
            return false;
        }
        for file in self.sys_files().split(',').collect::<Vec<&str>>() {
            if file == str::from_utf8(table_name).unwrap() {
                return false;
            }
        }
        true
    }
}