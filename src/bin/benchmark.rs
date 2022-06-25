use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use queue::lock_based_queue::Queue as lbQueue;
use queue::lock_free_queue::Queue as lfQueue;
// use queue::lock_free_queue_by_epoch::Queue as lfeQueue;
use queue::lock_free_queue_no_aba::Queue as lfQueue2;

fn main() {
    let lfq = Arc::new(lfQueue::new());
    let lfq1 = lfq.clone();
    let lfq_cnt = Arc::new(AtomicUsize::new(0));
    let lfq_cnt_1 = lfq_cnt.clone();
    thread::spawn(move || loop {
        lfq.enqueue(1);
        lfq_cnt_1.fetch_add(1, Ordering::Release);
    });
    thread::spawn(move || loop {
        lfq1.dequeue();
    });

    let lbq = Arc::new(lbQueue::new());
    let lbq1 = lbq.clone();
    let lbq_cnt = Arc::new(AtomicUsize::new(0));
    let lbq_cnt_1 = lbq_cnt.clone();
    thread::spawn(move || loop {
        lbq.enqueue(1);
        lbq_cnt_1.fetch_add(1, Ordering::Release);
    });
    thread::spawn(move || loop {
        lbq1.dequeue();
    });

    let lfqn = Arc::new(lfQueue2::new());
    let lfqn1 = lfqn.clone();
    let lfqn_cnt = Arc::new(AtomicUsize::new(0));
    let lfqn_cnt_1 = lfqn_cnt.clone();
    thread::spawn(move || loop {
        lfqn.enqueue(1);
        lfqn_cnt_1.fetch_add(1, Ordering::Release);
    });
    thread::spawn(move || loop {
        lfqn1.dequeue();
    });

    println!("lock_free_queue\tlock_free_queue_no_aba\tlock_based_queue");

    let end = Instant::now() + Duration::from_secs(30);
    while Instant::now() < end {
        thread::sleep(Duration::from_secs(1));
        println!(
            "{}\t{}\t{}",
            lfq_cnt.load(Ordering::Acquire),
            lfqn_cnt.load(Ordering::Acquire),
            lbq_cnt.load(Ordering::Acquire),
        );
        lfq_cnt.store(0, Ordering::Release);
        lfqn_cnt.store(0, Ordering::Release);
        lbq_cnt.store(0, Ordering::Release);
    }
}
