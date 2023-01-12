use criterion::Criterion;

fn bench_rcm(c: &mut Criterion) {
    let mut group = c.benchmark_group("rcm_bench");
    group.bench_function("rcm_bench_check", |b| {
        crate::rcm::rcm::bench_check(b);
    });
    group.finish();
}

criterion::criterion_group!(benches, bench_rcm);

criterion::criterion_main!(benches);

pub mod rcm;