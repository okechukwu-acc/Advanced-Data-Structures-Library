#[cfg(test)]
mod tests {
    use super::super::graphs::directed_graph::DirectedGraph;
    use super::super::graphs::undirected_graph::UndirectedGraph;

    #[test]
    fn test_directed_graph() {
        let mut graph = DirectedGraph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_edge(1, 2);
        assert!(graph.contains(&1));
        assert!(graph.contains(&2));
        assert!(graph.neighbors(&1).unwrap().contains(&2));
    }

    #[test]
    fn test_undirected_graph() {
        let mut graph = UndirectedGraph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_edge(1, 2);
        assert!(graph.contains(&1));
        assert!(graph.contains(&2));
        assert!(graph.neighbors(&1).unwrap().contains(&2));
        assert!(graph.neighbors(&2).unwrap().contains(&1));
    }
}
