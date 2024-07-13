use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BTree<T> {
    root: Option<Box<Node<T>>>,
    t: usize,
}

#[derive(Debug, Clone)]
struct Node<T> {
    keys: Vec<T>,
    children: Vec<Option<Box<Node<T>>>>,
    leaf: bool,
}

impl<T: Ord + Clone> BTree<T> {
    pub fn new(t: usize) -> Self {
        BTree {
            root: None,
            t,
        }
    }

    pub fn insert(&mut self, key: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node {
                keys: vec![key],
                children: vec![],
                leaf: true,
            }));
        } else {
            let root = self.root.as_mut().unwrap();
            if root.keys.len() == 2 * self.t - 1 {
                let mut s = Node {
                    keys: vec![],
                    children: vec![Some(Box::new(root.clone()))],
                    leaf: false,
                };
                self.split_child(&mut s, 0);
                self.insert_non_full(&mut s, key);
                self.root = Some(Box::new(s));
            } else {
                self.insert_non_full(root, key);
            }
        }
    }

    fn split_child(&self, parent: &mut Node<T>, i: usize) {
        let t = self.t;
        let child = parent.children[i].as_mut().unwrap();
        let mut z = Node {
            keys: child.keys.split_off(t),
            children: if child.leaf {
                vec![]
            } else {
                child.children.split_off(t)
            },
            leaf: child.leaf,
        };
        parent.keys.insert(i, child.keys.pop().unwrap());
        parent.children.insert(i + 1, Some(Box::new(z)));
    }

    fn insert_non_full(&self, node: &mut Node<T>, key: T) {
        if node.leaf {
            match node.keys.binary_search(&key) {
                Ok(_) => return,
                Err(pos) => node.keys.insert(pos, key),
            }
        } else {
            match node.keys.binary_search(&key) {
                Ok(_) => return,
                Err(pos) => {
                    if node.children[pos].as_mut().unwrap().keys.len() == 2 * self.t - 1 {
                        self.split_child(node, pos);
                        if key > node.keys[pos] {
                            self.insert_non_full(node.children[pos + 1].as_mut().unwrap(), key);
                        } else {
                            self.insert_non_full(node.children[pos].as_mut().unwrap(), key);
                        }
                    } else {
                        self.insert_non_full(node.children[pos].as_mut().unwrap(), key);
                    }
                },
            }
        }
    }

    pub fn contains(&self, key: &T) -> bool {
        self.search(&self.root, key).is_some()
    }

    fn search<'a>(&'a self, node: &'a Option<Box<Node<T>>>, key: &T) -> Option<&'a T> {
        if let Some(node) = node {
            match node.keys.binary_search(key) {
                Ok(pos) => Some(&node.keys[pos]),
                Err(pos) => self.search(&node.children.get(pos).unwrap(), key),
            }
        } else {
            None
        }
    }

    pub fn in_order_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        self.in_order(&self.root, &mut result);
        result
    }

    fn in_order<'a>(&'a self, node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node) = node {
            for i in 0..node.keys.len() {
                self.in_order(&node.children.get(i).unwrap(), result);
                result.push(&node.keys[i]);
            }
            self.in_order(&node.children.get(node.keys.len()).unwrap(), result);
        }
    }
}
