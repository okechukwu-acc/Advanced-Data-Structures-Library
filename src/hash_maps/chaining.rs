use std::collections::hash_map::DefaultHasher;
use std::collections::LinkedList;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct ChainingHashMap<K, V> {
    data: Vec<LinkedList<(K, V)>>,
    capacity: usize,
    size: usize,
}

impl<K: Hash + Eq + Clone, V: Clone> ChainingHashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        ChainingHashMap {
            data: vec![LinkedList::new(); capacity],
            capacity,
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key) % self.capacity;
        for (existing_key, existing_value) in self.data[index].iter_mut() {
            if existing_key == &key {
                *existing_value = value;
                return;
            }
        }

        self.data[index].push_back((key, value));
        self.size += 1;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key) % self.capacity;
        for (existing_key, value) in self.data[index].iter() {
            if existing_key == key {
                return Some(value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash(key) % self.capacity;
        let bucket = &mut self.data[index];

        let pos = bucket.iter().position(|(existing_key, _)| existing_key == key);
        if let Some(pos) = pos {
            self.size -= 1;
            return bucket.remove(pos).map(|(_, value)| value);
        }
        None
    }

    fn hash<Q: ?Sized>(&self, key: &Q) -> usize
    where
        Q: Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}
