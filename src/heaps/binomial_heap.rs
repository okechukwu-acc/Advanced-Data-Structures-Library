use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct BinomialHeap<T> {
    roots: Vec<Node<T>>,
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
}

impl<T: Ord> BinomialHeap<T> {
    pub fn new() -> Self {
        BinomialHeap { roots: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        let mut node = Node {
            value,
            children: Vec::new(),
        };

        self.roots.push(node);

        let mut i = self.roots.len() - 1;
        while i > 0 && self.roots[i].value < self.roots[i - 1].value {
            self.roots.swap(i, i - 1);
            i -= 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.roots.is_empty() {
            return None;
        }

        let min = self.roots.remove(0);

        for child in min.children.into_iter().rev() {
            self.roots.push(child);
        }

        self.consolidate();
        Some(min.value)
    }

    fn consolidate(&mut self) {
        let mut i = 0;

        while i < self.roots.len() - 1 {
            if self.roots[i].children.len() == self.roots[i + 1].children.len() {
                let child = self.roots.remove(i + 1);
                self.roots[i].children.push(child);
            } else {
                i += 1;
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.roots.first().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }
}
