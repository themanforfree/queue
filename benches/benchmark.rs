use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lock_free_queue", |b| {
        use queue::lock_free_queue::Queue;
        let q = Queue::new();
        b.iter(|| {
            q.enqueue(black_box(5));
            q.dequeue();
        });
    })
    .bench_function("lock_free_queue_no_aba", |b| {
        use queue::lock_free_queue_no_aba::Queue;
        let q = Queue::new();
        b.iter(|| {
            q.enqueue(black_box(5));
            q.dequeue();
        });
    })
    .bench_function("lock_free_queue_by_epoch", |b| {
        use queue::lock_free_queue_by_epoch::Queue;
        let q = Queue::new();
        b.iter(|| {
            q.enqueue(black_box(5));
            q.dequeue();
        });
    })
    .bench_function("lock_based_queue", |b| {
        use queue::lock_based_queue::Queue;
        let q = Queue::new();
        b.iter(|| {
            q.enqueue(black_box(5));
            q.dequeue();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
