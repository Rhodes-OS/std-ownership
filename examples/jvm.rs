use std_ownership_api::model::Resource;

pub struct JVM;

impl<'a> Resource for JVM {
    fn id(&self) -> u8 {
        4
    }

    #[allow(unused_variables)]
    fn check(&self, buf: &[u8]) -> bool {
        true
    }
}