pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.items.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn top(&mut self) -> Option<&T> {
        self.items.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.top(), Some(&2));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.top(), Some(&1));
    }

    #[test]
    fn test_size() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.size(), 0);
        stack.push(1);
        assert_eq!(stack.size(), 1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_top() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        assert_eq!(stack.top(), Some(&1));
        stack.push(2);
        assert_eq!(stack.top(), Some(&2));
        stack.pop();
        assert_eq!(stack.top(), Some(&1));
    }
}
