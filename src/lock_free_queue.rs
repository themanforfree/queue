use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

struct Node<T> {
    value: T,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: AtomicPtr::from(ptr::null_mut()),
        }
    }
}

pub struct Queue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> Queue<T>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        let dummy_node = Box::into_raw(Box::new(Node::new(T::default())));
        let head = AtomicPtr::from(dummy_node);
        let tail = AtomicPtr::new(head.load(Ordering::SeqCst));
        Queue { head, tail }
    }

    pub fn enqueue(&mut self, x: T) {
        let q = Box::into_raw(Box::new(Node::new(x)));
        let mut p = self.tail.load(Ordering::Acquire);
        let old_p = p;

        let next_node = unsafe { &(*p).next };
        while next_node
            .compare_exchange(ptr::null_mut(), q, Ordering::Release, Ordering::Relaxed)
            .is_err()
        {
            unsafe {
                while !(*p).next.load(Ordering::Acquire).is_null() {
                    p = (*p).next.load(Ordering::Acquire);
                }
            }
        }
        let _ = self
            .tail
            .compare_exchange(old_p, q, Ordering::Release, Ordering::Relaxed);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        loop {
            let h = self.head.load(Ordering::Acquire);
            let h_next = unsafe { (*h).next.load(Ordering::Acquire) };
            if h_next.is_null() {
                return None;
            }

            if self
                .head
                .compare_exchange(h, h_next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                return Some(unsafe { (*h_next).value });
            }
        }
    }
}
