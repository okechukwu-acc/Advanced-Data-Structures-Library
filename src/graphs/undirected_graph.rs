use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct UndirectedGraph<T> {
    nodes: HashMap<T, Vec<T>>,
}

impl<T: Eq + std::hash::Hash + Clone> UndirectedGraph<T> {
    pub fn new() -> Self {
        UndirectedGraph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: T) {
        self.nodes.entry(node).or_insert_with(Vec::new);
    }

    pub fn add_edge(&mut self, a: T, b: T) {
        self.nodes.entry(a.clone()).or_insert_with(Vec::new).push(b.clone());
        self.nodes.entry(b).or_insert_with(Vec::new).push(a);
    }

    pub fn contains(&self, node: &T) -> bool {
        self.nodes.contains_key(node)
    }

    pub fn neighbors(&self, node: &T) -> Option<&Vec<T>> {
        self.nodes.get(node)
    }

    pub fn bfs(&self, start: T) -> Vec<T> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(start.clone());
        visited.insert(start.clone(), true);

        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(neighbors) = self.neighbors(&node) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor) {
                        queue.push_back(neighbor.clone());
                        visited.insert(neighbor.clone(), true);
                    }
                }
            }
        }

        result
    }
}
