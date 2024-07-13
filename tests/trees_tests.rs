#[cfg(test)]
mod tests {
    use super::super::trees::binary_search_tree::BinarySearchTree;
    use super::super::trees::avl_tree::AVLTree;
    use super::super::trees::red_black_tree::RedBlackTree;
    use super::super::trees::b_tree::BTree;

    #[test]
    fn test_binary_search_tree_insert() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(20);
        bst.insert(5);
        assert!(bst.contains(&10));
        assert!(bst.contains(&20));
        assert!(bst.contains(&5));
        assert!(!bst.contains(&15));
    }

    #[test]
    fn test_avl_tree_insert() {
        let mut avl = AVLTree::new();
        avl.insert(10);
        avl.insert(20);
        avl.insert(5);
        assert!(avl.contains(&10));
        assert!(avl.contains(&20));
        assert!(avl.contains(&5));
        assert!(!avl.contains(&15));
    }

    #[test]
    fn test_red_black_tree_insert() {
        let mut rbt = RedBlackTree::new();
        rbt.insert(10);
        rbt.insert(20);
        rbt.insert(5);
        assert!(rbt.contains(&10));
        assert!(rbt.contains(&20));
        assert!(rbt.contains(&5));
        assert!(!rbt.contains(&15));
    }

    #[test]
    fn test_b_tree_insert() {
        let mut btree = BTree::new(2);
        btree.insert(10);
        btree.insert(20);
        btree.insert(5);
        assert!(btree.contains(&10));
        assert!(btree.contains(&20));
        assert!(btree.contains(&5));
        assert!(!btree.contains(&15));
    }
}
