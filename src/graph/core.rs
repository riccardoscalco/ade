use std::collections::HashSet;

pub trait NodeTrait {
    fn id(&self) -> &str;
    fn predecessors(&self) -> &HashSet<String>;
    fn successors(&self) -> &HashSet<String>;
    fn add_predecessor(&mut self, id: &str);
    fn add_successor(&mut self, id: &str);
    fn remove_predecessor(&mut self, id: &str);
    fn remove_successor(&mut self, id: &str);
}

pub trait EdgeTrait {
    fn id(&self) -> &str;
    fn source(&self) -> &str;
    fn target(&self) -> &str;
    fn make_edge_id(source: &str, target: &str) -> String;
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph<N, E> {
    nodes: HashMap<String, N>,
    edges: HashMap<String, E>,
}

impl<N: NodeTrait, E: EdgeTrait> Graph<N, E> {
    pub fn new(nodes: Vec<N>, edges: Vec<E>) -> Self {
        let mut graph = Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        };

        for node in nodes {
            graph.add_node(node);
        }

        for edge in edges {
            graph.add_edge(edge);
        }

        graph
    }

    pub fn add_node(&mut self, node: N) -> bool {
        self.nodes.insert(node.id().to_string(), node).is_none()
    }

    pub fn remove_node(&mut self, id: &str) {
        if let Some(node) = self.nodes.get(id) {
            let predecessors = node.predecessors().clone();
            let successors = node.successors().clone();

            for source in predecessors {
                self.remove_edge(&source, id);
            }

            for target in successors {
                self.remove_edge(id, &target);
            }
        }

        self.nodes.remove(id);
    }

    pub fn get_node(&self, id: &str) -> Option<&N> {
        self.nodes.get(id)
    }

    pub fn get_nodes(&self) -> Vec<&N> {
        self.nodes.values().collect()
    }

    pub fn add_edge(&mut self, edge: E) -> bool {
        if !self.nodes.contains_key(edge.source()) || !self.nodes.contains_key(edge.target()) {
            return false;
        }

        // Update successors and predecessors
        if let Some(source_node) = self.nodes.get_mut(edge.source()) {
            source_node.add_successor(edge.target());
        }
        if let Some(target_node) = self.nodes.get_mut(edge.target()) {
            target_node.add_predecessor(edge.source());
        }

        // Insert the edge
        self.edges.insert(edge.id().to_string(), edge).is_none()
    }

    pub fn remove_edge(&mut self, source: &str, target: &str) {
        let edge_id = E::make_edge_id(source, target);

        if self.edges.remove(&edge_id).is_some() {
            self.nodes.get_mut(source).map(|node| {
                node.remove_successor(target);
            });
            self.nodes.get_mut(target).map(|node| {
                node.remove_predecessor(source);
            });
        }
    }

    pub fn get_edge(&self, source: &str, target: &str) -> Option<&E> {
        self.edges.get(&E::make_edge_id(source, target))
    }

    pub fn get_edges(&self) -> Vec<&E> {
        self.edges.values().collect()
    }
}

use std::fmt;

impl<N: NodeTrait, E: EdgeTrait> fmt::Display for Graph<N, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Nodes:")?;
        for node in self.get_nodes() {
            writeln!(
                f,
                "- {} (pred: {:?}, succ: {:?})",
                node.id(),
                node.predecessors(),
                node.successors()
            )?;
        }

        writeln!(f, "\nEdges:")?;
        for edge in self.get_edges() {
            writeln!(f, "- {} -> {}", edge.source(), edge.target())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    #[test]
    fn test_add_node() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        assert!(graph.add_node(Node::new("A"))); // Adding a new node should return true
        assert!(!graph.add_node(Node::new("A"))); // Adding the same node again should return false
    }

    #[test]
    fn test_get_node() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));

        let node = graph.get_node("A");
        assert!(node.is_some());
        assert_eq!(node.unwrap().id(), "A");
        assert_eq!(node.unwrap().predecessors().len(), 0);
        assert_eq!(node.unwrap().successors().len(), 0);

        let missing = graph.get_node("B");
        assert!(missing.is_none());
    }

    #[test]
    fn test_get_nodes() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));

        let nodes = graph.get_nodes();
        assert_eq!(nodes.len(), 2);

        // assert it containes both nodes
        assert!(nodes.iter().any(|n| n.id() == "A"));
        assert!(nodes.iter().any(|n| n.id() == "B"));
    }

    #[test]
    fn test_add_predecessor() {
        let mut graph = Graph::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));
        graph.add_edge(Edge::new("A", "B"));

        assert!(graph.get_node("B").unwrap().predecessors().contains("A"));
    }

    #[test]
    fn test_add_successor() {
        let mut graph = Graph::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));
        graph.add_edge(Edge::new("A", "B"));

        assert!(graph.get_node("A").unwrap().successors().contains("B"));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));

        assert!(graph.add_edge(Edge::new("A", "B"))); // Adding a new edge should return true
        assert!(!graph.add_edge(Edge::new("A", "B"))); // Adding the same edge again should return false

        let edge_id = Edge::make_edge_id("A", "B");
        assert!(graph.edges.contains_key(&edge_id));

        // Check predecessors and successors

        assert!(graph.get_node("A").unwrap().successors().contains("B"));
        assert!(graph.get_node("B").unwrap().predecessors().contains("A"));
    }

    #[test]
    fn test_get_edge() {
        let mut graph = Graph::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));

        graph.add_edge(Edge::new("A", "B"));

        let edge = graph.get_edge("A", "B");
        assert!(edge.is_some());
        assert_eq!(edge.unwrap().source(), "A");
        assert_eq!(edge.unwrap().target(), "B");

        let missing_edge = graph.get_edge("B", "A");
        assert!(missing_edge.is_none());
    }

    #[test]
    fn test_predecessors() {
        let mut graph = Graph::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));
        graph.add_node(Node::new("C"));

        graph.add_edge(Edge::new("A", "B"));
        graph.add_edge(Edge::new("C", "B"));

        let predecessors = graph.get_node("B").unwrap().predecessors();
        assert_eq!(predecessors.len(), 2);
        assert!(predecessors.contains("A"));
        assert!(predecessors.contains("C"));
    }

    #[test]
    fn test_successors() {
        let mut graph = Graph::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));
        graph.add_node(Node::new("C"));

        graph.add_edge(Edge::new("A", "B"));
        graph.add_edge(Edge::new("A", "C"));

        let successors = graph.get_node("A").unwrap().successors();
        assert_eq!(successors.len(), 2);
        assert!(successors.contains("B"));
        assert!(successors.contains("C"));
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = Graph::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));

        graph.add_edge(Edge::new("A", "B"));
        assert!(graph.get_edge("A", "B").is_some());

        graph.remove_edge("A", "B");
        assert!(graph.get_edge("A", "B").is_none());

        // Check that predecessors and successors are also removed
        assert!(!graph.get_node("A").unwrap().successors().contains("B"));
        assert!(!graph.get_node("B").unwrap().predecessors().contains("A"));
    }

    #[test]
    fn test_remove_node() {
        let mut graph = Graph::new(Vec::new(), Vec::new());

        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));
        graph.add_node(Node::new("C"));

        graph.add_edge(Edge::new("A", "B"));
        assert!(graph.get_edge("A", "B").is_some());

        graph.add_edge(Edge::new("B", "C"));
        assert!(graph.get_edge("B", "C").is_some());

        graph.add_edge(Edge::new("C", "A"));
        assert!(graph.get_edge("C", "A").is_some());

        graph.remove_node("A");
        assert!(graph.get_node("A").is_none());
        assert!(graph.get_edge("A", "B").is_none());
        assert!(graph.get_edge("C", "A").is_none());
        assert!(graph.get_edge("B", "C").is_some());

        let node_b = graph.get_node("B").unwrap();
        assert!(!node_b.predecessors().contains("A"));
        assert!(node_b.successors().contains("C"));

        let node_c = graph.get_node("C").unwrap();
        assert!(node_c.predecessors().contains("B"));
        assert!(!node_c.successors().contains("A"));
    }
}
