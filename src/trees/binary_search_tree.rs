use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

#[derive(Debug, Clone)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => node.left = Self::insert_node(node.left.take(), value),
                    Ordering::Greater => node.right = Self::insert_node(node.right.take(), value),
                    Ordering::Equal => {} // Value already exists, do nothing
                }
                Some(node)
            }
            None => Some(Box::new(TreeNode {
                value,
                left: None,
                right: None,
            })),
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
