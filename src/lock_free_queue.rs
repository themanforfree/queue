use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node {
            value: None,
            next: AtomicPtr::from(ptr::null_mut()),
        }
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: Some(value),
            next: AtomicPtr::from(ptr::null_mut()),
        }
    }
}

pub struct Queue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        let dummy_node = Box::into_raw(Box::new(Node::default()));
        let head = AtomicPtr::new(dummy_node);
        let tail = AtomicPtr::new(dummy_node);
        Queue { head, tail }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&self, x: T) {
        let q = Box::into_raw(Box::new(Node::new(x)));
        let mut p = self.tail.load(Ordering::Acquire);
        let old_p = p;

        unsafe {
            while (*p)
                .next
                .compare_exchange(ptr::null_mut(), q, Ordering::Release, Ordering::Relaxed)
                .is_err()
            {
                while !(*p).next.load(Ordering::Acquire).is_null() {
                    p = (*p).next.load(Ordering::Acquire);
                }
            }
        }

        let _ = self
            .tail
            .compare_exchange(old_p, q, Ordering::Release, Ordering::Relaxed);
    }

    pub fn dequeue(&self) -> Option<T> {
        let data: Option<T>;
        loop {
            let p = self.head.load(Ordering::Acquire);
            let p_next = unsafe { (*p).next.load(Ordering::Acquire) };
            if p_next.is_null() {
                return None;
            }

            if self
                .head
                .compare_exchange(p, p_next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                data = unsafe { (*p_next).value.take() };
                let _ = unsafe { Box::from_raw(p) };
                return data;
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        let _ = unsafe { Box::from_raw(self.head.load(Ordering::Acquire)) };
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};

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

    #[test]
    fn test_in_multi_threads() {
        let queue = Arc::new(Queue::new());

        let q1 = queue.clone();
        let t1 = thread::spawn(move || {
            for i in 0..1000 {
                q1.enqueue(i);
            }
        });

        let q2 = queue.clone();
        let t2 = thread::spawn(move || {
            for i in 1000..2000 {
                q2.enqueue(i);
            }
        });

        let _ = t1.join();
        let _ = t2.join();
        let mut sum = 0;
        while let Some(v) = queue.dequeue() {
            sum += v;
        }

        assert_eq!(sum, (0i32..2000).sum());
    }
}
