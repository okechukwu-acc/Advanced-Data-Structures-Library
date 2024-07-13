use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct FibonacciHeap<T> {
    min: Link<T>,
    nodes: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    degree: usize,
    mark: bool,
    parent: Option<Weak<RefCell<Node<T>>>>,
    child: Link<T>,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord> FibonacciHeap<T> {
    pub fn new() -> Self {
        FibonacciHeap { min: None, nodes: 0 }
    }

    pub fn push(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node {
            value,
            degree: 0,
            mark: false,
            parent: None,
            child: None,
            left: None,
            right: None,
        }));

        if let Some(ref min) = self.min {
            node.borrow_mut().right = Some(Rc::clone(min));
            node.borrow_mut().left = min.borrow().left.clone();

            if let Some(ref left) = min.borrow().left {
                left.borrow_mut().right = Some(Rc::clone(&node));
            }

            min.borrow_mut().left = Some(Rc::clone(&node));

            if node.borrow().value < min.borrow().value {
                self.min = Some(Rc::clone(&node));
            }
        } else {
            self.min = Some(Rc::clone(&node));
            node.borrow_mut().left = Some(Rc::clone(&node));
            node.borrow_mut().right = Some(Rc::clone(&node));
        }

        self.nodes += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.min.is_none() {
            return None;
        }

        let min = self.min.take().unwrap();

        while let Some(child) = min.borrow_mut().child.take() {
            child.borrow_mut().parent = None;
            self.insert_into_root_list(child);
        }

        if Rc::ptr_eq(&min, min.borrow().right.as_ref().unwrap()) {
            self.min = None;
        } else {
            self.min = min.borrow().right.clone();
            self.remove_from_root_list(Rc::clone(&min));
            self.consolidate();
        }

        self.nodes -= 1;
        Some(Rc::try_unwrap(min).ok().unwrap().into_inner().value)
    }

    fn insert_into_root_list(&mut self, node: Rc<RefCell<Node<T>>>) {
        if let Some(ref min) = self.min {
            node.borrow_mut().right = Some(Rc::clone(min));
            node.borrow_mut().left = min.borrow().left.clone();

            if let Some(ref left) = min.borrow().left {
                left.borrow_mut().right = Some(Rc::clone(&node));
            }

            min.borrow_mut().left = Some(Rc::clone(&node));

            if node.borrow().value < min.borrow().value {
                self.min = Some(Rc::clone(&node));
            }
        } else {
            self.min = Some(Rc::clone(&node));
            node.borrow_mut().left = Some(Rc::clone(&node));
            node.borrow_mut().right = Some(Rc::clone(&node));
        }
    }

    fn remove_from_root_list(&mut self, node: Rc<RefCell<Node<T>>>) {
        let left = node.borrow().left.clone().unwrap();
        let right = node.borrow().right.clone().unwrap();

        left.borrow_mut().right = Some(right.clone());
        right.borrow_mut().left = Some(left.clone());

        if Rc::ptr_eq(&node, &self.min.clone().unwrap()) {
            self.min = Some(right);
        }
    }

    fn consolidate(&mut self) {
        let mut degrees = vec![None; self.nodes];
        let mut nodes = Vec::new();
        let start = self.min.clone().unwrap();
        nodes.push(Rc::clone(&start));

        let mut current = start.borrow().right.clone().unwrap();
        while !Rc::ptr_eq(&current, &start) {
            nodes.push(Rc::clone(&current));
            current = current.borrow().right.clone().unwrap();
        }

        for node in nodes {
            let mut x = Rc::clone(&node);
            let mut degree = x.borrow().degree;

            while let Some(Some(mut y)) = degrees.get_mut(degree) {
                if x.borrow().value > y.borrow().value {
                    std::mem::swap(&mut x, &mut y);
                }

                self.link(y, Rc::clone(&x));
                *y.borrow_mut() = None;
                degree += 1;
            }

            if degree >= degrees.len() {
                degrees.resize(degree + 1, None);
            }

            degrees[degree] = Some(Rc::clone(&x));
        }

        self.min = None;

        for node in degrees.into_iter().flatten() {
            self.insert_into_root_list(node);
        }
    }

    fn link(&mut self, y: Rc<RefCell<Node<T>>>, x: Rc<RefCell<Node<T>>>) {
        self.remove_from_root_list(Rc::clone(&y));

        y.borrow_mut().left = None;
        y.borrow_mut().right = None;
        y.borrow_mut().parent = Some(Rc::downgrade(&x));

        if x.borrow().child.is_some() {
            let child = x.borrow().child.as_ref().unwrap();
            y.borrow_mut().right = Some(Rc::clone(child));
            y.borrow_mut().left = child.borrow().left.clone();

            if let Some(ref left) = child.borrow().left {
                left.borrow_mut().right = Some(Rc::clone(&y));
            }

            child.borrow_mut().left = Some(Rc::clone(&y));
        } else {
            x.borrow_mut().child = Some(Rc::clone(&y));
            y.borrow_mut().left = Some(Rc::clone(&y));
            y.borrow_mut().right = Some(Rc::clone(&y));
        }

        x.borrow_mut().degree += 1;
        y.borrow_mut().mark = false;
    }
}
