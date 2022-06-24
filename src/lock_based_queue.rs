use std::{collections::LinkedList, sync::Mutex};

pub struct Queue<T>(Mutex<LinkedList<T>>);

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue(Mutex::new(LinkedList::new()))
    }

    pub fn enqueue(&mut self, x: T) {
        self.0.lock().unwrap().push_back(x);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.0.lock().unwrap().pop_front()
    }
}
