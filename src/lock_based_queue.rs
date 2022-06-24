use std::{collections::LinkedList, sync::Mutex};

pub struct Queue<T>(Mutex<LinkedList<T>>);

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Queue(Mutex::new(LinkedList::new()))
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&self, x: T) {
        self.0.lock().unwrap().push_back(x);
    }

    pub fn dequeue(&self) -> Option<T> {
        self.0.lock().unwrap().pop_front()
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
