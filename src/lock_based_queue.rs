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

    pub fn enqueue(&mut self, x: T) {
        self.0.lock().unwrap().push_back(x);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.0.lock().unwrap().pop_front()
    }
}
