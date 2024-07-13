use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct RedBlackTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

#[derive(Debug, Clone)]
struct TreeNode<T> {
    value: T,
    color: Color,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Black,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_node(self.root.take(), value);
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        let mut node = match node {
            Some(node) => node,
            None => return Some(Box::new(TreeNode {
                value,
                color: Color::Red,
                left: None,
                right: None,
            })),
        };

        match value.cmp(&node.value) {
            Ordering::Less => node.left = Self::insert_node(node.left.take(), value),
            Ordering::Greater => node.right = Self::insert_node(node.right.take(), value),
            Ordering::Equal => return Some(node),
        }

        // Fix the red-black tree properties
        if Self::is_red(&node.right) && !Self::is_red(&node.left) {
            node = Self::rotate_left(node);
        }
        if Self::is_red(&node.left) && Self::is_red(&node.left.as_ref().unwrap().left) {
            node = Self::rotate_right(node);
        }
        if Self::is_red(&node.left) && Self::is_red(&node.right) {
            Self::flip_colors(&mut node);
        }

        Some(node)
    }

    fn is_red(node: &Option<Box<TreeNode<T>>>) -> bool {
        match node {
            Some(node) => node.color == Color::Red,
            None => false,
        }
    }

    fn rotate_left(mut node: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let mut x = node.right.take().unwrap();
        node.right = x.left.take();
        x.left = Some(node);
        x.color = x.left.as_ref().unwrap().color.clone();
        x.left.as_mut().unwrap().color = Color::Red;
        x
    }

    fn rotate_right(mut node: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let mut x = node.left.take().unwrap();
        node.left = x.right.take();
        x.right = Some(node);
        x.color = x.right.as_ref().unwrap().color.clone();
        x.right.as_mut().unwrap().color = Color::Red;
        x
    }

    fn flip_colors(node: &mut Box<TreeNode<T>>) {
        node.color = match node.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
        if let Some(ref mut left) = node.left {
            left.color = match left.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }
        if let Some(ref mut right) = node.right {
            right.color = match right.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        Self::contains_node(&self.root, value)
    }

    fn contains_node(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        match node {
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => Self::contains_node(&node.left, value),
                Ordering::Greater => Self::contains_node(&node.right, value),
                Ordering::Equal => true,
            },
            None => false,
        }
    }

    pub fn in_order_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::in_order(&self.root, &mut result);
        result
    }

    fn in_order<'a>(node: &'a Option<Box<TreeNode<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node) = node {
            Self::in_order(&node.left, result);
            result.push(&node.value);
            Self::in_order(&node.right, result);
        }
    }
}
