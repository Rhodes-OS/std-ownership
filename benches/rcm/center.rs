use std_ownership_api::model::Resource;
use std_ownership::model::resource::buffer::Buffer;
use std_ownership::rcm::center::ResourceCenter;
use criterion::Bencher;

pub fn bench_borrow<R>(b: &mut Bencher, rc: &mut ResourceCenter<R>) 
where
    R: Resource + Eq + Copy
{
    b.iter(|| {
        rc.borrow(MySQL::new(), 1, "mysql.ibd".as_bytes());
        // rc.free(MySQL{}, 1);
    });
}

struct MySQL<'a> {
    used_bufs: Vec<&'a [u8]>,
    buf: Buffer
}

impl<'a> MySQL<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self { 
            used_bufs: vec!["mysql.ibd".as_bytes()],
            buf: Buffer::new(1024)
        }
    }

    #[inline]
    pub fn used_buf(&self) -> &Vec<&'a [u8]> {
        &self.used_bufs
    }

    #[inline]
    pub fn buf(&self) -> Buffer {
        self.buf
    }
}

use std::str;

impl<'a> Resource for MySQL<'a> {
    #[inline]
    fn id(&self) -> u8 {
        2
    }

    #[inline]
    fn check(&self, table_name: &[u8]) -> bool {
        if !self.buf().check(table_name) {
            return false;
        }
        for buf in self.used_buf() {
            if str::from_utf8(buf).unwrap() == str::from_utf8(table_name).unwrap() {
                return false;
            }
        }
        true
    }
}