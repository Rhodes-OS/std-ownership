use criterion::Criterion;
use pprof::criterion::{Output, PProfProfiler};

use std_ownership::model::resource::buffer::Buffer;
use std_ownership::rcm::checker::buffer_checker::BufferChecker;
use std_ownership::rcm::center::ResourceCenter;

fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_compare");
    // group.sample_size(2000);

    group.bench_function("bench_rcm_borrow", |b| {
        let buffer = Buffer::new(1024);
        
        let owner_checkers = vec![BufferChecker::new(buffer)];
        let mut rc = ResourceCenter::builder();
        rc.build_owner_checkers(0, buffer, owner_checkers);

        crate::rcm::center::bench_borrow(b, &mut rc);
    });
    group.bench_function("bench_unix_geteuid", |b| {
        crate::unix::syscall::bench_geteuid(b);
    });

    group.finish();
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_compare
}

criterion::criterion_main!(benches);

pub mod rcm;
pub mod unix;