pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.len() == 0 {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.size() == 0;
    }

    pub fn front(&self) -> Option<&T> {
        self.items.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut q = Queue::<i32>::new();
        q.enqueue(1);
        q.enqueue(2);
        assert_eq!(q.size(), 2);
        assert_eq!(q.front(), Some(&1));
    }

    #[test]
    fn test_dequeue() {
        let mut q = Queue::<i32>::new();
        q.enqueue(1);
        q.enqueue(2);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.size(), 1);
        assert_eq!(q.front(), Some(&2));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.size(), 0);
        assert_eq!(q.front(), (None));
    }
}
