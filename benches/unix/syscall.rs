use criterion::Bencher;

extern crate libc;
use libc::geteuid;

pub fn bench_geteuid(b: &mut Bencher) {
    b.iter(|| {
        unsafe {
            geteuid();
        }
    });
}