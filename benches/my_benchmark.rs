use criterion::{black_box, criterion_group, criterion_main, Criterion};

use queue::lock_based_queue::Queue;

fn criterion_benchmark(c: &mut Criterion) {
    let mut q = Queue::new();
    c.bench_function("lock_based_queue", |b| b.iter(|| q.enqueue(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
