use criterion::Criterion;
use pprof::criterion::{Output, PProfProfiler};

use std_ownership::model::resource::buffer::Buffer;
use std_ownership::rcm::center::ResourceCenter;
use std_ownership::rcm::contract::ResourceContract;
use std_ownership_api::model::Resource;

fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_compare");
    // group.sample_size(2000);

    group.bench_function("bench_rcm_borrow", |b| {
        let mut rc = ResourceCenter::<Buffer>::new();
    
        let buffer = Buffer::new(1024);

        let mut res_contract = ResourceContract::new(buffer);
        res_contract.add_owner_lifecycle(buffer.id());
        rc.register(res_contract);

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