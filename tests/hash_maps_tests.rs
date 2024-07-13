#[cfg(test)]
mod tests {
    use super::super::hash_maps::open_addressing::OpenAddressingHashMap;
    use super::super::hash_maps::chaining::ChainingHashMap;

    #[test]
    fn test_open_addressing_hash_map() {
        let mut map = OpenAddressingHashMap::new(16);
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
        assert_eq!(map.remove(&2), Some("two"));
        assert_eq!(map.get(&2), None);
    }

    #[test]
    fn test_chaining_hash_map() {
        let mut map = ChainingHashMap::new(16);
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
        assert_eq!(map.remove(&2), Some("two"));
        assert_eq!(map.get(&2), None);
    }
}
