use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct OpenAddressingHashMap<K, V> {
    data: Vec<Option<(K, V)>>,
    capacity: usize,
    size: usize,
}

impl<K: Hash + Eq + Clone, V: Clone> OpenAddressingHashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        OpenAddressingHashMap {
            data: vec![None; capacity],
            capacity,
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.size >= self.capacity / 2 {
            self.resize(self.capacity * 2);
        }

        let mut index = self.hash(&key) % self.capacity;
        while self.data[index].is_some() {
            if let Some((existing_key, _)) = &self.data[index] {
                if existing_key == &key {
                    self.data[index] = Some((key, value));
                    return;
                }
            }
            index = (index + 1) % self.capacity;
        }

        self.data[index] = Some((key, value));
        self.size += 1;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.hash(key) % self.capacity;
        while let Some((existing_key, value)) = &self.data[index] {
            if existing_key == key {
                return Some(value);
            }
            index = (index + 1) % self.capacity;
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut index = self.hash(key) % self.capacity;
        while let Some((existing_key, value)) = &self.data[index] {
            if existing_key == key {
                let value = value.clone();
                self.data[index] = None;
                self.size -= 1;
                self.rehash(index);
                return Some(value);
            }
            index = (index + 1) % self.capacity;
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

    fn resize(&mut self, new_capacity: usize) {
        let mut new_data = vec![None; new_capacity];
        let old_data = std::mem::replace(&mut self.data, new_data);
        self.capacity = new_capacity;
        self.size = 0;

        for entry in old_data.into_iter().flatten() {
            self.insert(entry.0, entry.1);
        }
    }

    fn rehash(&mut self, start_index: usize) {
        let mut index = (start_index + 1) % self.capacity;
        while let Some((key, value)) = self.data[index].take() {
            self.size -= 1;
            self.insert(key, value);
            index = (index + 1) % self.capacity;
        }
    }
}
