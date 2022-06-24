#![feature(test)]
pub mod lock_based_queue;
pub mod lock_free_queue;

extern crate test;

#[cfg(test)]
mod tests {
    use crate::lock_based_queue::Queue as lbQueue;
    use crate::lock_free_queue::Queue as lfQueue;
    use test::{black_box, Bencher};

    #[test]
    fn test_lock_free_queue() {
        let mut queue = lfQueue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_lock_based_queue() {
        let mut queue = lbQueue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[bench]
    fn lock_free_benchmark(c: &mut Bencher) {
        let mut q = lfQueue::new();
        c.iter(|| {
            for i in 0..black_box(100) {
                q.enqueue(i);
            }
        })
    }

    #[bench]
    fn lock_based_benchmark(c: &mut Bencher) {
        let mut q = lbQueue::new();
        c.iter(|| {
            for i in 0..black_box(100) {
                q.enqueue(i);
            }
        })
    }
}
