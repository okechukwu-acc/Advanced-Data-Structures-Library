use super::directed_graph::DirectedGraph;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node<T> {
    cost: usize,
    position: T,
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl<T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + std::hash::Hash + Clone> DirectedGraph<T> {
    pub fn dijkstra(&self, start: T) -> HashMap<T, usize> {
        let mut dist: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start.clone(), 0);
        heap.push(Node { cost: 0, position: start.clone() });

        while let Some(Node { cost, position }) = heap.pop() {
            if cost > *dist.entry(position.clone()).or_insert(usize::MAX) {
                continue;
            }

            if let Some(neighbors) = self.neighbors(&position) {
                for neighbor in neighbors {
                    let next = Node { cost: cost + 1, position: neighbor.clone() };

                    if next.cost < *dist.entry(neighbor.clone()).or_insert(usize::MAX) {
                        heap.push(next);
                        dist.insert(neighbor.clone(), next.cost);
                    }
                }
            }
        }

        dist
    }
}
