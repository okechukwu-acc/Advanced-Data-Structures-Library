#[cfg(test)]
mod tests {
    use super::super::heaps::binary_heap::BinaryHeap;
    use super::super::heaps::fibonacci_heap::FibonacciHeap;
    use super::super::heaps::binomial_heap::BinomialHeap;

    #[test]
    fn test_binary_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);
        assert_eq!(heap.pop(), Some(20));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_fibonacci_heap() {
        let mut heap = FibonacciHeap::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(20));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_binomial_heap() {
        let mut heap = BinomialHeap::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(20));
        assert_eq!(heap.pop(), None);
    }
}
