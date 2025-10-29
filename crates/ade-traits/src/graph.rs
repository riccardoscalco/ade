use crate::{EdgeTrait, NodeTrait};

/// A trait for read-only views of directed graphs.
///
/// This trait provides a common interface for querying directed graph structures,
/// including access to nodes, edges, and their relationships. Implementations of this
/// trait should provide efficient read-only access to the graph's topology.
///
/// # Type Parameters
///
/// * `N` - The node type, which must implement [`NodeTrait`]
/// * `E` - The edge type, which must implement [`EdgeTrait`]
///
/// # Examples
///
/// ```
/// use ade_graph::implementations::{Graph, Node, Edge};
/// use ade_traits::{GraphViewTrait, NodeTrait};
///
/// let graph = Graph::new(
///     vec![Node::new(1), Node::new(2), Node::new(3)],
///     vec![Edge::new(1, 2), Edge::new(2, 3)],
/// );
///
/// // Query nodes and edges
/// assert_eq!(graph.get_nodes().count(), 3);
/// assert_eq!(graph.get_edges().count(), 2);
/// assert!(graph.has_edge(1, 2));
/// ```
pub trait GraphViewTrait<N: NodeTrait, E: EdgeTrait> {
    /// Returns `true` if the graph contains no nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::GraphViewTrait;
    ///
    /// let empty = Graph::<Node, Edge>::new(vec![], vec![]);
    /// assert!(empty.is_empty());
    ///
    /// let graph = Graph::<Node, Edge>::new(vec![Node::new(1)], vec![]);
    /// assert!(!graph.is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// Returns a reference to the node with the specified key.
    ///
    /// # Parameters
    ///
    /// * `key` - The unique identifier of the node to retrieve
    ///
    /// # Panics
    ///
    /// Panics if the node with the given key does not exist in the graph.
    /// Use [`has_node`](Self::has_node) to check for existence first.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::{GraphViewTrait, NodeTrait};
    ///
    /// let graph = Graph::<Node, Edge>::new(
    ///     vec![Node::new(1), Node::new(2)],
    ///     vec![],
    /// );
    ///
    /// let node = graph.get_node(1);
    /// assert_eq!(node.key(), 1);
    /// ```
    fn get_node(&self, key: u32) -> &N;

    /// Returns a reference to the edge from `source` to `target`.
    ///
    /// # Parameters
    ///
    /// * `source` - The key of the source node
    /// * `target` - The key of the target node
    ///
    /// # Panics
    ///
    /// Panics if the edge does not exist in the graph.
    /// Use [`has_edge`](Self::has_edge) to check for existence first.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::{GraphViewTrait, EdgeTrait};
    ///
    /// let graph = Graph::new(
    ///     vec![Node::new(1), Node::new(2)],
    ///     vec![Edge::new(1, 2)],
    /// );
    ///
    /// let edge = graph.get_edge(1, 2);
    /// assert_eq!(edge.source(), 1);
    /// assert_eq!(edge.target(), 2);
    /// ```
    fn get_edge(&self, source: u32, target: u32) -> &E;

    /// Returns `true` if the graph contains a node with the specified key.
    ///
    /// # Parameters
    ///
    /// * `key` - The unique identifier of the node to check
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::GraphViewTrait;
    ///
    /// let graph = Graph::<Node, Edge>::new(
    ///     vec![Node::new(1), Node::new(2)],
    ///     vec![],
    /// );
    ///
    /// assert!(graph.has_node(1));
    /// assert!(graph.has_node(2));
    /// assert!(!graph.has_node(3));
    /// ```
    fn has_node(&self, key: u32) -> bool;

    /// Returns `true` if the graph contains an edge from `source` to `target`.
    ///
    /// # Parameters
    ///
    /// * `source` - The key of the source node
    /// * `target` - The key of the target node
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::GraphViewTrait;
    ///
    /// let graph = Graph::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![Edge::new(1, 2)],
    /// );
    ///
    /// assert!(graph.has_edge(1, 2));
    /// assert!(!graph.has_edge(2, 1));
    /// assert!(!graph.has_edge(1, 3));
    /// ```
    fn has_edge(&self, source: u32, target: u32) -> bool;

    /// Returns an iterator over all nodes in the graph.
    ///
    /// The order of iteration is implementation-dependent.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::{GraphViewTrait, NodeTrait};
    ///
    /// let graph = Graph::<Node, Edge>::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![],
    /// );
    ///
    /// let node_keys: Vec<u32> = graph.get_nodes()
    ///     .map(|n| n.key())
    ///     .collect();
    /// assert_eq!(node_keys.len(), 3);
    /// assert!(node_keys.contains(&1));
    /// assert!(node_keys.contains(&2));
    /// assert!(node_keys.contains(&3));
    /// ```
    fn get_nodes<'a>(&'a self) -> impl Iterator<Item = &'a N>
    where
        N: 'a;

    /// Returns an iterator over all edges in the graph.
    ///
    /// The order of iteration is implementation-dependent.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::{GraphViewTrait, EdgeTrait};
    ///
    /// let graph = Graph::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![Edge::new(1, 2), Edge::new(2, 3)],
    /// );
    ///
    /// let edge_keys: Vec<(u32, u32)> = graph.get_edges()
    ///     .map(|e| e.key())
    ///     .collect();
    /// assert_eq!(edge_keys.len(), 2);
    /// assert!(edge_keys.contains(&(1, 2)));
    /// assert!(edge_keys.contains(&(2, 3)));
    /// ```
    fn get_edges<'a>(&'a self) -> impl Iterator<Item = &'a E>
    where
        E: 'a;

    /// Returns an iterator over the predecessor nodes of the specified node.
    ///
    /// Predecessors are nodes that have an edge pointing to the specified node.
    ///
    /// # Parameters
    ///
    /// * `node_key` - The key of the node whose predecessors to retrieve
    ///
    /// # Panics
    ///
    /// Panics if the node with the given key does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::{GraphViewTrait, NodeTrait};
    ///
    /// let graph = Graph::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![Edge::new(1, 2), Edge::new(3, 2)],
    /// );
    ///
    /// // Node 2 has predecessors 1 and 3
    /// let pred_keys: Vec<u32> = graph.get_predecessors(2)
    ///     .map(|n| n.key())
    ///     .collect();
    /// assert_eq!(pred_keys.len(), 2);
    /// assert!(pred_keys.contains(&1));
    /// assert!(pred_keys.contains(&3));
    /// ```
    fn get_predecessors<'a>(&'a self, node_key: u32) -> impl Iterator<Item = &'a N>
    where
        N: 'a;

    /// Returns an iterator over the successor nodes of the specified node.
    ///
    /// Successors are nodes that the specified node has an edge pointing to.
    ///
    /// # Parameters
    ///
    /// * `node_key` - The key of the node whose successors to retrieve
    ///
    /// # Panics
    ///
    /// Panics if the node with the given key does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::{GraphViewTrait, NodeTrait};
    ///
    /// let graph = Graph::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![Edge::new(1, 2), Edge::new(1, 3)],
    /// );
    ///
    /// // Node 1 has successors 2 and 3
    /// let succ_keys: Vec<u32> = graph.get_successors(1)
    ///     .map(|n| n.key())
    ///     .collect();
    /// assert_eq!(succ_keys.len(), 2);
    /// assert!(succ_keys.contains(&2));
    /// assert!(succ_keys.contains(&3));
    /// ```
    fn get_successors<'a>(&'a self, node_key: u32) -> impl Iterator<Item = &'a N>
    where
        N: 'a;

    /// Returns an iterator over all node keys in the graph.
    ///
    /// This is more efficient than iterating over nodes when only keys are needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::GraphViewTrait;
    ///
    /// let graph = Graph::<Node, Edge>::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![],
    /// );
    ///
    /// let keys: Vec<u32> = graph.get_node_keys().collect();
    /// assert_eq!(keys.len(), 3);
    /// assert!(keys.contains(&1));
    /// assert!(keys.contains(&2));
    /// assert!(keys.contains(&3));
    /// ```
    fn get_node_keys(&self) -> impl Iterator<Item = u32> + '_;

    /// Returns an iterator over the keys of predecessor nodes.
    ///
    /// This is more efficient than [`get_predecessors`](Self::get_predecessors)
    /// when only keys are needed.
    ///
    /// # Parameters
    ///
    /// * `node_key` - The key of the node whose predecessor keys to retrieve
    /// 
    /// # Panics
    ///
    /// Panics if the node with the given key does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::GraphViewTrait;
    ///
    /// let graph = Graph::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![Edge::new(1, 2), Edge::new(3, 2)],
    /// );
    ///
    /// let pred_keys: Vec<u32> = graph.get_predecessors_keys(2).collect();
    /// assert_eq!(pred_keys.len(), 2);
    /// assert!(pred_keys.contains(&1));
    /// assert!(pred_keys.contains(&3));
    /// ```
    fn get_predecessors_keys(&self, node_key: u32) -> impl Iterator<Item = u32> + '_;

    /// Returns an iterator over the keys of successor nodes.
    ///
    /// This is more efficient than [`get_successors`](Self::get_successors)
    /// when only keys are needed.
    ///
    /// # Parameters
    ///
    /// * `node_key` - The key of the node whose successor keys to retrieve
    /// 
    /// # Panics
    ///
    /// Panics if the node with the given key does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::GraphViewTrait;
    ///
    /// let graph = Graph::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![Edge::new(1, 2), Edge::new(1, 3)],
    /// );
    ///
    /// let succ_keys: Vec<u32> = graph.get_successors_keys(1).collect();
    /// assert_eq!(succ_keys.len(), 2);
    /// assert!(succ_keys.contains(&2));
    /// assert!(succ_keys.contains(&3));
    /// ```
    fn get_successors_keys(&self, node_key: u32) -> impl Iterator<Item = u32> + '_;

    /// Creates a filtered view of the graph containing only the specified nodes.
    ///
    /// The filtered view includes only the nodes whose keys are in `node_keys`,
    /// along with any edges between those nodes.
    ///
    /// # Parameters
    ///
    /// * `node_keys` - A slice of node keys to include in the filtered view
    ///
    /// # Returns
    ///
    /// A new graph view containing only the specified nodes and edges between them.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::GraphViewTrait;
    ///
    /// let graph = Graph::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3), Node::new(4)],
    ///     vec![Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 4), Edge::new(1, 4)],
    /// );
    ///
    /// // Create a filtered view with only nodes 1, 2, and 3
    /// let filtered = graph.filter(&[1, 2, 3]);
    ///
    /// assert_eq!(filtered.get_nodes().count(), 3);
    /// assert!(filtered.has_node(1));
    /// assert!(filtered.has_node(2));
    /// assert!(filtered.has_node(3));
    /// assert!(!filtered.has_node(4));
    ///
    /// // Edge (1,2) and (2,3) are included, but (3,4) and (1,4) are excluded
    /// assert!(filtered.has_edge(1, 2));
    /// assert!(filtered.has_edge(2, 3));
    /// assert!(!filtered.has_edge(3, 4));
    /// assert!(!filtered.has_edge(1, 4));
    /// ```
    fn filter(&self, node_keys: &[u32]) -> impl GraphViewTrait<N, E>;

    /// Returns `true` if node keys form a sequential sequence starting from 0.
    ///
    /// This indicates whether the graph uses a dense, array-like node key allocation
    /// (0, 1, 2, 3, ..., n-1) which can enable certain optimizations.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge};
    /// use ade_traits::GraphViewTrait;
    ///
    /// // Sequential keys: 0, 1, 2
    /// let sequential = Graph::<Node, Edge>::new(
    ///     vec![Node::new(0), Node::new(1), Node::new(2)],
    ///     vec![],
    /// );
    /// assert!(sequential.has_sequential_keys());
    ///
    /// // Non-sequential keys: 1, 2, 3
    /// let non_sequential = Graph::<Node, Edge>::new(
    ///     vec![Node::new(1), Node::new(2), Node::new(3)],
    ///     vec![],
    /// );
    /// assert!(!non_sequential.has_sequential_keys());
    ///
    /// // Non-sequential keys: 0, 2, 3 (missing 1)
    /// let gaps = Graph::<Node, Edge>::new(
    ///     vec![Node::new(0), Node::new(2), Node::new(3)],
    ///     vec![],
    /// );
    /// assert!(!gaps.has_sequential_keys());
    /// ```
    fn has_sequential_keys(&self) -> bool;
}
