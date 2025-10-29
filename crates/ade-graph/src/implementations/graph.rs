use crate::implementations::FilteredGraph;
use ade_traits::{EdgeTrait, GraphViewTrait, NodeTrait};
use std::collections::HashMap;
use std::fmt::Debug;

/// A directed graph data structure with nodes and edges.
///
/// A `Graph` stores nodes of type `N` and directed edges of type `E`, where each node
/// is identified by a unique `u32` key and each edge connects a source node to a target node.
///
/// # Features
///
/// - **O(1) average-case** operations for adding, removing, and querying nodes and edges
/// - Support for **self-loops** (edges from a node to itself)
/// - Automatic **edge cleanup** when removing nodes
/// - **Filtering** capabilities to create subgraphs
///
/// # Type Parameters
///
/// * `N` - Node type implementing [`NodeTrait`]
/// * `E` - Edge type implementing [`EdgeTrait`]
///
/// # Examples
///
/// Creating a graph from vectors:
///
/// ```
/// use ade_graph::implementations::{Graph, Node, Edge};
/// use ade_graph::GraphViewTrait;
///
/// let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
/// let edges = vec![Edge::new(1, 2), Edge::new(2, 3)];
/// let graph = Graph::new(nodes, edges);
///
/// assert_eq!(graph.get_nodes().count(), 3);
/// assert!(graph.has_edge(1, 2));
/// assert!(graph.has_edge(2, 3));
/// ```
///
/// Building a graph incrementally:
///
/// ```
/// use ade_graph::implementations::{Graph, Node, Edge};
/// use ade_graph::GraphViewTrait;
///
/// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
///
/// // Add nodes
/// graph.add_node(Node::new(1));
/// graph.add_node(Node::new(2));
/// graph.add_node(Node::new(3));
///
/// // Add edges
/// graph.add_edge(Edge::new(1, 2));
/// graph.add_edge(Edge::new(2, 3));
/// graph.add_edge(Edge::new(3, 1)); // Creates a cycle
///
/// assert_eq!(graph.get_nodes().count(), 3);
/// assert!(graph.has_edge(3, 1));
/// ```
///
/// Removing nodes automatically removes connected edges:
///
/// ```
/// use ade_graph::implementations::{Graph, Node, Edge};
/// use ade_graph::GraphViewTrait;
///
/// let mut graph = Graph::<Node, Edge>::new(
///     vec![Node::new(1), Node::new(2), Node::new(3)],
///     vec![Edge::new(1, 2), Edge::new(2, 3)],
/// );
///
/// // Removing node 2 also removes edges (1,2) and (2,3)
/// graph.remove_node(2);
///
/// assert!(!graph.has_node(2));
/// assert!(!graph.has_edge(1, 2));
/// assert!(!graph.has_edge(2, 3));
/// ```
#[derive(Debug)]
pub struct Graph<N, E> {
    nodes: HashMap<u32, N>,
    edges: HashMap<(u32, u32), E>,
}

impl<N: NodeTrait, E: EdgeTrait> Graph<N, E> {
    /// Creates a new graph from vectors of nodes and edges.
    ///
    /// The nodes and edges are added to the graph in the order they appear in the vectors.
    /// If duplicate nodes (same key) are provided, later nodes will replace earlier ones.
    /// If duplicate edges (same source-target pair) are provided, later edges will replace earlier ones.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A vector of nodes to add to the graph
    /// * `edges` - A vector of edges to add to the graph
    ///
    /// # Returns
    ///
    /// A new `Graph` instance containing the provided nodes and edges.
    ///
    /// # Panics
    ///
    /// Panics if any edge references a node that is not in the nodes vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_graph::GraphViewTrait;
    ///
    /// // Create an empty graph
    /// let graph = Graph::<Node, Edge>::new(vec![], vec![]);
    /// assert!(graph.is_empty());
    /// ```
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_graph::GraphViewTrait;
    ///
    /// // Create a graph with nodes and edges
    /// let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
    /// let edges = vec![Edge::new(1, 2), Edge::new(2, 3)];
    /// let graph = Graph::new(nodes, edges);
    ///
    /// assert_eq!(graph.get_nodes().count(), 3);
    /// assert_eq!(graph.get_edges().count(), 2);
    /// assert!(graph.has_edge(1, 2));
    /// assert!(graph.has_edge(2, 3));
    /// ```
    pub fn new(nodes: Vec<N>, edges: Vec<E>) -> Self {
        let mut graph = Graph {
            nodes: HashMap::with_capacity(nodes.len()),
            edges: HashMap::with_capacity(edges.len()),
        };

        for node in nodes {
            graph.add_node(node);
        }

        for edge in edges {
            graph.add_edge(edge);
        }

        graph
    }

    /// Adds a node to the graph.
    ///
    /// If a node with the same key already exists in the graph, it will be replaced
    /// and the old node will be returned.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to add to the graph
    ///
    /// # Returns
    ///
    /// * `Some(old_node)` - If a node with the same key already existed, returns the replaced node
    /// * `None` - If no node with this key existed previously
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    ///
    /// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
    ///
    /// // Adding a new node returns None
    /// assert!(graph.add_node(Node::new(1)).is_none());
    ///
    /// // Adding a node with the same key returns the old node
    /// assert!(graph.add_node(Node::new(1)).is_some());
    /// ```
    pub fn add_node(&mut self, node: N) ->  Option<N> {
        self.nodes.insert(node.key(), node)
    }

    /// Removes a node from the graph.
    ///
    /// This method also removes all edges connected to the node (both incoming and outgoing).
    /// The predecessors and successors of adjacent nodes are updated accordingly.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the node to remove
    ///
    /// # Returns
    ///
    /// * `Some(node)` - If the node existed, returns the removed node
    /// * `None` - If no node with this key existed
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_graph::GraphViewTrait;
    ///
    /// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
    /// graph.add_node(Node::new(1));
    /// graph.add_node(Node::new(2));
    /// graph.add_edge(Edge::new(1, 2));
    ///
    /// // Remove the node and verify edges are also removed
    /// assert!(graph.remove_node(1).is_some());
    /// assert!(!graph.has_node(1));
    /// assert!(!graph.has_edge(1, 2));
    ///
    /// // Trying to remove it again returns None
    /// assert!(graph.remove_node(1).is_none());
    /// ```
    pub fn remove_node(&mut self, key: u32) -> Option<N> {
        if let Some(node) = self.nodes.get(&key) {
            let mut edges_to_remove = Vec::new();
    
            for &predecessor in node.predecessors() {
                edges_to_remove.push((predecessor, key));
            }
    
            for &successor in node.successors() {
                edges_to_remove.push((key, successor));
            }
    
            for (source, target) in edges_to_remove {
                self.remove_edge(source, target);
            }

            return self.nodes.remove(&key)
        }
        None
    }

    /// Adds an edge to the graph.
    ///
    /// This method creates a directed edge from source to target and updates the
    /// successors and predecessors of the connected nodes. Self-loops (edges where
    /// source equals target) are supported.
    ///
    /// If an edge with the same key (source, target) already exists, it will be replaced
    /// and the old edge will be returned.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge to add to the graph
    ///
    /// # Returns
    ///
    /// * `Some(old_edge)` - If an edge with the same key already existed, returns the replaced edge
    /// * `None` - If no edge with this key existed previously
    ///
    /// # Panics
    ///
    /// Panics if either the source or target node does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_graph::GraphViewTrait;
    ///
    /// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
    /// graph.add_node(Node::new(1));
    /// graph.add_node(Node::new(2));
    ///
    /// // Adding a new edge returns None
    /// assert!(graph.add_edge(Edge::new(1, 2)).is_none());
    /// assert!(graph.has_edge(1, 2));
    ///
    /// // Adding the same edge again returns the old edge
    /// assert!(graph.add_edge(Edge::new(1, 2)).is_some());
    /// ```
    pub fn add_edge(&mut self, edge: E) -> Option<E> {
        let (source, target) = edge.key();
        
        if source == target {
            match self.nodes.get_mut(&source) {
                Some(node) => {
                    node.add_successor(target);
                    node.add_predecessor(source);
                    return self.edges.insert((source, target), edge)
                }
                None => panic!("Node {} not found", source)
            }
        }
        
        match self.nodes.get_disjoint_mut([&source, &target]) {
            [Some(source_node), Some(target_node)] => {
                source_node.add_successor(target);
                target_node.add_predecessor(source);
                self.edges.insert((source, target), edge)
            }
            _ => panic!("Node {} or {} not found", source, target)
        }
    }

    /// Removes an edge from the graph.
    ///
    /// This method removes the directed edge from source to target and updates the
    /// successors and predecessors of the connected nodes accordingly.
    ///
    /// # Arguments
    ///
    /// * `source` - The key of the source node
    /// * `target` - The key of the target node
    ///
    /// # Returns
    ///
    /// * `Some(edge)` - If the edge existed, returns the removed edge
    /// * `None` - If no edge with this key existed
    ///
    /// # Panics
    ///
    /// Panics if either the source or target node does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_graph::GraphViewTrait;
    ///
    /// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
    /// graph.add_node(Node::new(1));
    /// graph.add_node(Node::new(2));
    /// graph.add_edge(Edge::new(1, 2));
    ///
    /// // Remove the edge
    /// assert!(graph.remove_edge(1, 2).is_some());
    /// assert!(!graph.has_edge(1, 2));
    ///
    /// // Trying to remove it again returns None
    /// assert!(graph.remove_edge(1, 2).is_none());
    /// ```
    pub fn remove_edge(&mut self, source:u32, target:u32) -> Option<E> {
        if !self.nodes.contains_key(&source) || !self.nodes.contains_key(&target) {
            panic!("Node {} or {} not found", source, target);
        }

        match self.edges.remove(&(source, target)) {
            Some(edge) => {
                if let Some(source_node) = self.nodes.get_mut(&source) {
                    source_node.remove_successor(target);
                }
                if let Some(target_node) = self.nodes.get_mut(&target) {
                    target_node.remove_predecessor(source);
                }
                Some(edge)
            }
            None => None,
        }
    }
}

impl<N: NodeTrait, E: EdgeTrait> GraphViewTrait<N, E> for Graph<N, E> {
    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn has_sequential_keys(&self) -> bool {
        let size = self.nodes.len();
        if size == 0 {
            return true;
        }

        // Quick checks first
        if !self.nodes.contains_key(&0) || !self.nodes.contains_key(&(size as u32 - 1)) {
            return false;
        }

        // Check if all keys are sequential
        (0..size as u32).all(|i| self.nodes.contains_key(&i))
    }

    fn get_node(&self, key: u32) -> &N {
        self.nodes
            .get(&key)
            .unwrap_or_else(|| panic!("Node {} not found", key))
    }

    fn has_node(&self, key: u32) -> bool {
        self.nodes.contains_key(&key)
    }

    fn get_edge(&self, source: u32, target: u32) -> &E {
        let edge_key = (source, target);
        self.edges
            .get(&edge_key)
            .unwrap_or_else(|| panic!("Edge {}→{} not found", source, target))
    }

    fn has_edge(&self, source: u32, target: u32) -> bool {
        let edge_key = (source, target);
        self.edges.contains_key(&edge_key)
    }

    fn get_nodes<'a>(&'a self) -> impl Iterator<Item = &'a N>
    where
        N: 'a,
    {
        self.nodes.values()
    }

    fn get_node_keys(&self) -> impl Iterator<Item = u32> {
        self.nodes.keys().copied()
    }

    fn get_edges<'a>(&'a self) -> impl Iterator<Item = &'a E>
    where
        E: 'a,
    {
        self.edges.values()
    }

    fn get_predecessors<'a>(&'a self, node_key: u32) -> impl Iterator<Item = &'a N>
    where
        N: 'a,
    {
        self.get_node(node_key)
            .predecessors()
            .iter()
            .map(|pred_key| self.get_node(*pred_key))
    }

    fn get_predecessors_keys(&self, node_key: u32) -> impl Iterator<Item = u32> {
        self.get_node(node_key).predecessors().iter().copied()
    }

    fn get_successors<'a>(&'a self, node_key: u32) -> impl Iterator<Item = &'a N>
    where
        N: 'a,
    {
        self.get_node(node_key)
            .successors()
            .iter()
            .map(|succ_key| self.get_node(*succ_key))
    }

    fn get_successors_keys(&self, node_key: u32) -> impl Iterator<Item = u32> {
        self.get_node(node_key).successors().iter().copied()
    }

    fn filter(&self, node_keys: &[u32]) -> impl GraphViewTrait<N, E> {
        FilteredGraph::new(self, node_keys.iter().copied())
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
                node.key(),
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
    use crate::implementations::edge::Edge;
    use crate::implementations::node::Node;

    #[test]
    fn test_is_empty() {
        let graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        assert!(graph.is_empty());
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        assert!(graph.add_node(Node::new(1)).is_none()); // Adding a new node should return None
        assert!(graph.add_node(Node::new(1)).is_some()); // Adding the same node again should return Some(Node)
    }

    #[test]
    fn test_get_node() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));

        let node = graph.get_node(1);
        assert_eq!(node.key(), 1);
        assert_eq!(node.predecessors().len(), 0);
        assert_eq!(node.successors().len(), 0);
    }

    #[test]
    #[should_panic(expected = "Node 2 not found")]
    fn test_get_node_panic() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        graph.add_node(Node::new(1));
        graph.get_node(2);
    }

    #[test]
    fn test_get_nodes() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));

        assert_eq!(graph.get_nodes().count(), 2);

        // assert it contains both nodes
        assert!(graph.get_nodes().any(|n| n.key() == 1));
        assert!(graph.get_nodes().any(|n| n.key() == 2));
    }

    #[test]
    fn test_get_node_keys() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));

        assert_eq!(graph.get_node_keys().count(), 2);

        let node_keys = graph.get_node_keys().collect::<Vec<_>>();
        // assert it contains both keys
        assert!(node_keys.contains(&1));
        assert!(node_keys.contains(&2));
    }

    #[test]
    fn test_add_predecessor() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_edge(<Edge as EdgeTrait>::new(1, 2));

        assert!(graph.get_node(2).predecessors().contains(&1));
    }

    #[test]
    fn test_add_successor() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_edge(Edge::new(1, 2));

        assert!(graph.get_node(1).successors().contains(&2));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));

        assert!(graph.add_edge(Edge::new(1, 2)).is_none()); // Adding a new edge should return None
        assert!(graph.add_edge(Edge::new(1, 2)).is_some()); // Adding the same edge again should return Some(Edge)

        assert!(graph.has_edge(1, 2));

        // Check predecessors and successors
        assert!(graph.get_node(1).successors().contains(&2));
        assert!(graph.get_node(2).predecessors().contains(&1));
    }

    #[test]
    fn test_add_edge_loop() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        graph.add_node(Node::new(1));

        assert!(graph.add_edge(Edge::new(1, 1)).is_none()); // Adding a new edge should return None
        assert!(graph.add_edge(Edge::new(1, 1)).is_some()); // Adding the same edge again should return Some(Edge)
        
        assert!(graph.has_edge(1, 1));

        // Check predecessors and successors
        assert!(graph.get_node(1).successors().contains(&1));
        assert!(graph.get_node(1).predecessors().contains(&1));
    }

    #[test]
    #[should_panic(expected = "Node 2 or 1 not found")]
    fn test_add_edge_panic() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        graph.add_node(Node::new(1));
        graph.add_edge(Edge::new(2, 1));
    }

    #[test]
    #[should_panic(expected = "Node 2 not found")]
    fn test_add_edge_loop_panic() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        graph.add_node(Node::new(1));
        graph.add_edge(Edge::new(2, 2));
    }

    #[test]
    fn test_get_edge() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));

        graph.add_edge(Edge::new(1, 2));

        let edge = graph.get_edge(1, 2);
        assert_eq!(edge.source(), 1);
        assert_eq!(edge.target(), 2);
    }

    #[test]
    #[should_panic(expected = "Edge 2→1 not found")]
    fn test_get_edge_panic() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_edge(Edge::new(1, 2));
        graph.get_edge(2, 1);
    }

    #[test]
    fn test_predecessors() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));

        graph.add_edge(Edge::new(1, 2));
        graph.add_edge(Edge::new(3, 2));

        let predecessors = graph.get_node(2).predecessors();
        assert_eq!(predecessors.len(), 2);
        assert!(predecessors.contains(&1));
        assert!(predecessors.contains(&3));
    }

    #[test]
    fn test_successors() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));

        graph.add_edge(Edge::new(1, 2));
        graph.add_edge(Edge::new(1, 3));

        let successors = graph.get_node(1).successors();
        assert_eq!(successors.len(), 2);
        assert!(successors.contains(&2));
        assert!(successors.contains(&3));
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));

        graph.add_edge(Edge::new(1, 2));
        assert!(graph.has_edge(1, 2));

        let removed_edge = graph.remove_edge(1, 2);
        assert!(removed_edge.is_some());
        assert_eq!(removed_edge.unwrap().source(), 1);
        assert!(!graph.has_edge(1, 2));

        let removed_edge = graph.remove_edge(1, 2);
        assert!(removed_edge.is_none());

        // Check that predecessors and successors are also removed
        assert!(!graph.get_node(1).successors().contains(&2));
        assert!(!graph.get_node(2).predecessors().contains(&1));
    }

    #[test]
    #[should_panic(expected = "Node 1 or 3 not found")]
    fn test_remove_edge_panic() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_edge(Edge::new(1, 2));
        graph.remove_edge(1, 3);
    }

    #[test]
    fn test_remove_node() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));

        graph.add_edge(Edge::new(1, 2));
        assert!(graph.has_edge(1, 2));

        graph.add_edge(Edge::new(2, 3));
        assert!(graph.has_edge(2, 3));

        graph.add_edge(Edge::new(3, 1));
        assert!(graph.has_edge(3, 1));

        let removed_node = graph.remove_node(1);
        assert!(removed_node.is_some());
        assert_eq!(removed_node.unwrap().key(), 1);

        let removed_node = graph.remove_node(1);
        assert!(removed_node.is_none());

        // Check that the node and edges are removed
        assert!(!graph.has_node(1));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(3, 1));
        assert!(graph.has_edge(2, 3));

        let node_b = graph.get_node(2);
        assert!(!node_b.predecessors().contains(&1));
        assert!(node_b.successors().contains(&3));

        let node_c = graph.get_node(3);
        assert!(node_c.predecessors().contains(&2));
        assert!(!node_c.successors().contains(&1));


    }

    #[test]
    fn test_get_predecessors() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));

        graph.add_edge(Edge::new(1, 2));
        graph.add_edge(Edge::new(3, 2));

        assert_eq!(graph.get_predecessors(2).count(), 2);
        assert!(graph.get_predecessors(2).any(|n| n.key() == 1));
        assert!(graph.get_predecessors(2).any(|n| n.key() == 3));
    }

    #[test]
    fn test_get_successors() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));

        graph.add_edge(Edge::new(1, 2));
        graph.add_edge(Edge::new(1, 3));

        assert_eq!(graph.get_successors(1).count(), 2);
        assert!(graph.get_successors(1).any(|n| n.key() == 2));
        assert!(graph.get_successors(1).any(|n| n.key() == 3));
    }

    #[test]
    fn test_get_successors_keys() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));
        graph.add_edge(Edge::new(1, 2));
        graph.add_edge(Edge::new(1, 3));

        let successors_keys: Vec<u32> = graph.get_successors_keys(1).collect();
        assert_eq!(successors_keys.len(), 2);
        assert!(successors_keys.contains(&2));
        assert!(successors_keys.contains(&3));
    }

    #[test]
    fn test_filter() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));

        graph.add_edge(Edge::new(1, 2));
        graph.add_edge(Edge::new(1, 3));

        // 2 nodes and 1 edge
        let subgraph1 = graph.filter(&[1, 2]);
        assert_eq!(subgraph1.get_nodes().count(), 2);
        assert!(subgraph1.has_node(1));
        assert!(subgraph1.has_node(2));
        assert!(!subgraph1.has_node(3));
        assert!(subgraph1.has_edge(1, 2));
        assert!(!subgraph1.has_edge(1, 3));

        // 1 node and 0 edges
        let subgraph2 = graph.filter(&[1]);
        assert_eq!(subgraph2.get_nodes().count(), 1);
        assert!(subgraph2.has_node(1));
        assert!(!subgraph2.has_node(2));
        assert!(!subgraph2.has_node(3));
        assert!(!subgraph2.has_edge(1, 2));
        assert!(!subgraph2.has_edge(1, 3));

        // 0 nodes and 0 edges (node 4 doesn't exist)
        let subgraph3 = graph.filter(&[4]);
        assert_eq!(subgraph3.get_nodes().count(), 0);
        assert!(!subgraph3.has_node(1));
        assert!(!subgraph3.has_node(2));
        assert!(!subgraph3.has_node(3));
        assert!(!subgraph3.has_edge(1, 2));
        assert!(!subgraph3.has_edge(1, 3));
    }

    #[test]
    fn test_has_node() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        assert!(graph.has_node(1));
        assert!(!graph.has_node(2));
    }

    #[test]
    fn test_has_edge() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));

        assert!(!graph.has_edge(1, 2));
        graph.add_edge(Edge::new(1, 2));
        assert!(graph.has_edge(1, 2));
        assert!(!graph.has_edge(2, 1)); // Directed graph
    }

    #[test]
    fn test_get_edges() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));

        graph.add_edge(Edge::new(1, 2));
        graph.add_edge(Edge::new(2, 3));

        assert_eq!(graph.get_edges().count(), 2);
        assert!(graph
            .get_edges()
            .any(|e| e.source() == 1 && e.target() == 2));
        assert!(graph
            .get_edges()
            .any(|e| e.source() == 2 && e.target() == 3));
    }

    #[test]
    fn test_has_sequential_keys() {
        let mut graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        graph.add_node(Node::new(0));
        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));

        assert!(graph.has_sequential_keys());
    }
}
