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

    // pub fn enqueue(&mut self, x: T) {
    //     // enqueue in figure 1
    //     let q = Box::into_raw(Box::new(Node::new(x))); // new node
    //     let mut p: *mut Node<T>;
    //     loop {
    //         p = self.tail.load(Ordering::Acquire);
    //         let succ = unsafe {
    //             (*p).next
    //                 .compare_exchange(ptr::null_mut(), q, Ordering::Release, Ordering::Relaxed)
    //                 .is_ok()
    //         };
    //         if succ != true {
    //             unsafe {
    //                 let _ = self.tail.compare_exchange(
    //                     p,
    //                     (*p).next.load(Ordering::Acquire),
    //                     Ordering::Release,
    //                     Ordering::Relaxed,
    //                 );
    //             };
    //         } else {
    //             break;
    //         }
    //     }
    //     let _ = self
    //         .tail
    //         .compare_exchange(p, q, Ordering::Release, Ordering::Relaxed);
    // }

    pub fn enqueue(&mut self, x: T) {
        // enqueue in figure 3
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

    pub fn dequeue(&mut self) -> Option<T> {
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
