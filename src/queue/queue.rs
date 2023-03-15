pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.len() == 0 {
            None
        } else {
            Some(self.items.swap_remove(0))
        }
    }
}
