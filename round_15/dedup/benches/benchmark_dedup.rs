use criterion::{criterion_group, criterion_main, Criterion};
use dedup::{dedup_via_btreeset, dedup_via_hashset};

fn bench_dedup_via_hashset(c: &mut Criterion) {
    let sample = vec![1, 1, 2, 3, 5];
    c.bench_function("dedup_via_hashset int", move |b| {
        b.iter(|| dedup_via_hashset(&sample))
    });
}

fn bench_dedup_via_hashset_2(c: &mut Criterion) {
    c.bench_function("dedup_via_hashset_2 int", |b| {
        let sample = vec![1, 1, 2, 3, 5];
        b.iter(|| dedup_via_hashset(&sample))
    });
}

fn bench_dedup_via_btreeset(c: &mut Criterion) {
    let sample = vec![1, 1, 2, 3, 5];
    c.bench_function("dedup_via_btreeset int", move |b| {
        b.iter(|| dedup_via_btreeset(&sample))
    });
}

criterion_group!(
    benches,
    bench_dedup_via_hashset,
    bench_dedup_via_hashset_2,
    bench_dedup_via_btreeset
);
criterion_main!(benches);
