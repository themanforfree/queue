use std::sync::atomic::Ordering;

use crossbeam::epoch::{self, Atomic, Owned, Shared};

struct Node<T> {
    value: Option<T>,
    next: Atomic<Node<T>>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node {
            value: None,
            next: Atomic::null(),
        }
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: Some(value),
            next: Atomic::null(),
        }
    }
}

pub struct Queue<T> {
    head: Atomic<Node<T>>,
    tail: Atomic<Node<T>>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        let dummy_node = Owned::new(Node::default());
        let head = Atomic::from(dummy_node);
        let tail = head.clone();

        Self { head, tail }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&self, value: T) {
        let guard = epoch::pin();
        let q = Owned::new(Node::new(value)).into_shared(&guard);
        let mut p = self.tail.load(Ordering::Acquire, &guard);
        let old_p = p;

        unsafe {
            while (*p.as_raw())
                .next
                .compare_exchange(
                    Shared::null(),
                    q,
                    Ordering::Release,
                    Ordering::Relaxed,
                    &guard,
                )
                .is_err()
            {
                while !(*p.as_raw()).next.load(Ordering::Acquire, &guard).is_null() {
                    p = (*p.as_raw()).next.load(Ordering::Acquire, &guard);
                }
            }
        }

        let _ = self
            .tail
            .compare_exchange(old_p, q, Ordering::Release, Ordering::Relaxed, &guard);
    }

    pub fn dequeue(&self) -> Option<T> {
        let guard = epoch::pin();
        loop {
            let p = self.head.load(Ordering::Acquire, &guard);
            let mut p_next = unsafe { (*p.as_raw()).next.load(Ordering::Acquire, &guard) };
            if p_next.is_null() {
                return None;
            }

            if self
                .head
                .compare_exchange(p, p_next, Ordering::Release, Ordering::Relaxed, &guard)
                .is_ok()
            {
                let data: Option<T>;
                unsafe {
                    data = p_next.deref_mut().value.take();
                    guard.defer_destroy(p);
                    // let _ = Box::from_raw(p);
                }
                return data;
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        let guard = epoch::pin();
        unsafe {
            let h = self.head.load(Ordering::Acquire, &guard);
            guard.defer_destroy(h);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Queue;

    #[test]
    fn test_in_single_thread() {
        let queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }
}
