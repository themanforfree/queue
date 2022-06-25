use std::{sync::Arc, thread};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use queue::lock_free_queue::Queue;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lock_free_queue", |b| {
        b.iter(|| {
            let q = Arc::new(Queue::new());

            let q1 = q.clone();
            let t1 = thread::spawn(move || {
                for i in 0..1000 {
                    q1.enqueue(black_box(i));
                }
            });

            let q2 = q.clone();
            let t2 = thread::spawn(move || {
                for i in 1000..2000 {
                    q2.enqueue(black_box(i));
                }
            });

            let q3 = q.clone();
            let t3 = thread::spawn(move || {
                for i in 2000..3000 {
                    q3.enqueue(black_box(i));
                }
            });

            let _ = t1.join();
            let _ = t2.join();
            let _ = t3.join();

            let mut sum = 0;
            while let Some(v) = q.dequeue() {
                sum += v;
            }

            assert_eq!(sum, (0i32..3000).sum::<i32>());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
