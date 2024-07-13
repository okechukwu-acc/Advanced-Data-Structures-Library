use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct AVLTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

#[derive(Debug, Clone)]
struct TreeNode<T> {
    value: T,
    height: i32,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        let mut node = match node {
            Some(node) => node,
            None => return Some(Box::new(TreeNode {
                value,
                height: 1,
                left: None,
                right: None,
            })),
        };

        match value.cmp(&node.value) {
            Ordering::Less => {
                node.left = Self::insert_node(node.left.take(), value);
            }
            Ordering::Greater => {
                node.right = Self::insert_node(node.right.take(), value);
            }
            Ordering::Equal => return Some(node),
        }

        node.height = 1 + i32::max(Self::height(&node.left), Self::height(&node.right));
        Some(Self::balance(node))
    }

    fn height(node: &Option<Box<TreeNode<T>>>) -> i32 {
        node.as_ref().map_or(0, |node| node.height)
    }

    fn balance_factor(node: &Option<Box<TreeNode<T>>>) -> i32 {
        Self::height(&node.as_ref().unwrap().left) - Self::height(&node.as_ref().unwrap().right)
    }

    fn balance(mut node: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let balance_factor = Self::balance_factor(&Some(node.clone()));

        if balance_factor > 1 {
            if Self::balance_factor(&node.left) < 0 {
                node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            }
            return Self::rotate_right(node);
        }

        if balance_factor < -1 {
            if Self::balance_factor(&node.right) > 0 {
                node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            }
            return Self::rotate_left(node);
        }

        node
    }

    fn rotate_right(mut y: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();
        x.right = Some(y);

        x.right.as_mut().unwrap().height = 1 + i32::max(Self::height(&x.right.as_ref().unwrap().left), Self::height(&x.right.as_ref().unwrap().right));
        x.height = 1 + i32::max(Self::height(&x.left), Self::height(&x.right));

        x
    }

    fn rotate_left(mut x: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();
        y.left = Some(x);

        y.left.as_mut().unwrap().height = 1 + i32::max(Self::height(&y.left.as_ref().unwrap().left), Self::height(&y.left.as_ref().unwrap().right));
        y.height = 1 + i32::max(Self::height(&y.left), Self::height(&y.right));

        y
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
