use criterion::{black_box, criterion_group, criterion_main, Criterion};

use queue::lock_based_queue::Queue;

fn criterion_benchmark(c: &mut Criterion) {
    let mut q = Queue::new();
    c.bench_function("lock_based_queue", |b| {
        b.iter(|| {
            for i in 0..black_box(100) {
                q.enqueue(i);
                assert_eq!(q.dequeue(), Some(i));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
