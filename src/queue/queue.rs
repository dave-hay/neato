struct Node<T> {
    item: Option<T>,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node { Some(item), next: None }
    }
}

pub struct Queue<T> {
    first: Option<Node<T>>,
    last: Option<Node<T>>,
    n: u32,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let mut dummy1 = Node::new(None);
        let mut dummy2 = Node::new(None);
        Queue {
            first: dummy1,
            last: dummy2,
            n: 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> u32 {
        self.n
    }
}
