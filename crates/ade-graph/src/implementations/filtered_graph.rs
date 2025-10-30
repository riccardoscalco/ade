use crate::implementations::Graph;
use ade_traits::{EdgeTrait, GraphViewTrait, NodeTrait};
use fixedbitset::FixedBitSet;
use ade_common::INVALID_KEY_SEQUENCE;

/// A filtered view of a graph that only exposes a subset of nodes and their edges.
///
/// `FilteredGraph` provides a memory-efficient way to work with subsets of a graph
/// without copying the underlying data. It maintains a bitset to track which nodes
/// are "active" (visible) in the filtered view, while borrowing the original graph data.
///
/// # Features
///
/// - **Zero-copy filtering** - references the base graph without duplicating data
/// - **O(1) membership checks** using a bitset for active nodes
/// - **Automatic edge filtering** - only edges between active nodes are visible
/// - **Composable filtering** - can create filtered views of filtered views
///
/// # Requirements
///
/// The base graph **must have sequential keys** (0, 1, 2, ..., n-1) to use filtering.
/// This requirement allows efficient bitset-based lookups. Attempting to filter a graph
/// with non-sequential keys will panic.
///
/// # Type Parameters
///
/// * `'a` - Lifetime of the borrowed base graph
/// * `N` - Node type implementing [`NodeTrait`]
/// * `E` - Edge type implementing [`EdgeTrait`]
///
/// # Examples
///
/// Creating a filtered subgraph:
///
/// ```
/// use ade_graph::implementations::{Graph, Node, Edge, FilteredGraph};
/// use ade_graph::GraphViewTrait;
///
/// // Create a graph with sequential keys (0, 1, 2)
/// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
/// graph.add_node(Node::new(0));
/// graph.add_node(Node::new(1));
/// graph.add_node(Node::new(2));
/// graph.add_edge(Edge::new(0, 1));
/// graph.add_edge(Edge::new(1, 2));
///
/// // Create a filtered view with only nodes 0 and 1
/// let filtered = FilteredGraph::new(&graph, vec![0, 1]);
///
/// assert_eq!(filtered.get_nodes().count(), 2);
/// assert!(filtered.has_node(0));
/// assert!(filtered.has_node(1));
/// assert!(!filtered.has_node(2));
/// assert!(filtered.has_edge(0, 1));
/// assert!(!filtered.has_edge(1, 2)); // Edge to inactive node is hidden
/// ```
///
/// Filtering with the `filter` method:
///
/// ```
/// use ade_graph::implementations::{Graph, Node, Edge};
/// use ade_graph::GraphViewTrait;
///
/// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
/// for i in 0..5 {
///     graph.add_node(Node::new(i));
/// }
/// graph.add_edge(Edge::new(0, 1));
/// graph.add_edge(Edge::new(1, 2));
/// graph.add_edge(Edge::new(2, 3));
/// graph.add_edge(Edge::new(3, 4));
///
/// // Create a filtered view using the filter method
/// let subgraph = graph.filter(&[1, 2, 3]);
///
/// assert_eq!(subgraph.get_nodes().count(), 3);
/// assert!(subgraph.has_edge(1, 2));
/// assert!(subgraph.has_edge(2, 3));
/// assert!(!subgraph.has_edge(0, 1)); // Edge from inactive node
/// ```
///
/// Composing filters:
///
/// ```
/// use ade_graph::implementations::{Graph, Node, Edge};
/// use ade_graph::GraphViewTrait;
///
/// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
/// for i in 0..5 {
///     graph.add_node(Node::new(i));
/// }
///
/// // First filter: nodes 0, 1, 2, 3
/// let first_filter = graph.filter(&[0, 1, 2, 3]);
///
/// // Second filter on top of the first: nodes 1, 2
/// let second_filter = first_filter.filter(&[1, 2]);
///
/// assert_eq!(second_filter.get_nodes().count(), 2);
/// assert!(second_filter.has_node(1));
/// assert!(second_filter.has_node(2));
/// assert!(!second_filter.has_node(0));
/// ```
pub struct FilteredGraph<'a, N: NodeTrait, E: EdgeTrait> {
    base: &'a Graph<N, E>,
    active: FixedBitSet,
}

impl<'a, N: NodeTrait, E: EdgeTrait> FilteredGraph<'a, N, E> {
    /// Creates a new filtered view of a graph with only the specified nodes active.
    ///
    /// This method creates a lightweight view over the base graph that only exposes
    /// the nodes specified in `active_nodes` and edges between those nodes. The base
    /// graph data is borrowed, not copied, making this operation very efficient.
    ///
    /// Node keys that don't exist in the base graph or are outside the valid range
    /// are silently ignored.
    ///
    /// # Arguments
    ///
    /// * `base` - A reference to the base graph to filter
    /// * `active_nodes` - An iterator of node keys to include in the filtered view
    ///
    /// # Returns
    ///
    /// A new `FilteredGraph` instance that provides a filtered view of the base graph.
    ///
    /// # Panics
    ///
    /// Panics if the base graph does not have sequential keys (0, 1, 2, ..., n-1).
    /// Sequential keys are required for the efficient bitset-based filtering mechanism.
    ///
    /// # Examples
    ///
    /// Basic filtering:
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge, FilteredGraph};
    /// use ade_graph::GraphViewTrait;
    ///
    /// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
    /// graph.add_node(Node::new(0));
    /// graph.add_node(Node::new(1));
    /// graph.add_node(Node::new(2));
    /// graph.add_edge(Edge::new(0, 1));
    /// graph.add_edge(Edge::new(1, 2));
    ///
    /// // Filter to only include nodes 0 and 2
    /// let filtered = FilteredGraph::new(&graph, vec![0, 2]);
    ///
    /// assert_eq!(filtered.get_nodes().count(), 2);
    /// assert!(filtered.has_node(0));
    /// assert!(!filtered.has_node(1));
    /// assert!(filtered.has_node(2));
    /// ```
    ///
    /// Invalid keys are silently ignored:
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge, FilteredGraph};
    /// use ade_graph::GraphViewTrait;
    ///
    /// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
    /// graph.add_node(Node::new(0));
    /// graph.add_node(Node::new(1));
    ///
    /// // Node 10 doesn't exist, but this doesn't panic
    /// let filtered = FilteredGraph::new(&graph, vec![0, 10]);
    ///
    /// assert_eq!(filtered.get_nodes().count(), 1);
    /// assert!(filtered.has_node(0));
    /// assert!(!filtered.has_node(10));
    /// ```
    ///
    /// Using an iterator as input:
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge, FilteredGraph};
    /// use ade_graph::GraphViewTrait;
    ///
    /// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
    /// for i in 0..10 {
    ///     graph.add_node(Node::new(i));
    /// }
    ///
    /// // Filter to even-numbered nodes using an iterator
    /// let filtered = FilteredGraph::new(&graph, (0..10).filter(|x| x % 2 == 0));
    ///
    /// assert_eq!(filtered.get_nodes().count(), 5);
    /// assert!(filtered.has_node(0));
    /// assert!(filtered.has_node(2));
    /// assert!(!filtered.has_node(1));
    /// assert!(!filtered.has_node(3));
    /// ```
    pub fn new(base: &'a Graph<N, E>, active_nodes: impl IntoIterator<Item = u32>) -> Self {
        // Panic if the graph does not have sequential keys
        if !base.has_sequential_keys() {
            panic!("{}", INVALID_KEY_SEQUENCE);
        }

        let node_count = base.get_node_keys().count();

        // Assume normalized keys: 0, 1, 2, ..., n-1
        let mut active = FixedBitSet::with_capacity(node_count);
        for key in active_nodes {
            if (key as usize) < node_count {
                active.insert(key as usize);
            }
        }

        Self { base, active }
    }

    /// Checks if a node is active (visible) in the filtered view.
    ///
    /// This is an internal helper method that performs an O(1) lookup in the bitset
    /// to determine if a node with the given key is included in the filtered view.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the node to check
    ///
    /// # Returns
    ///
    /// * `true` - If the node is active in this filtered view
    /// * `false` - If the node is not active (filtered out) or doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::{Graph, Node, Edge, FilteredGraph};
    /// use ade_graph::GraphViewTrait;
    ///
    /// let mut graph = Graph::<Node, Edge>::new(vec![], vec![]);
    /// graph.add_node(Node::new(0));
    /// graph.add_node(Node::new(1));
    /// graph.add_node(Node::new(2));
    ///
    /// let filtered = FilteredGraph::new(&graph, vec![0, 2]);
    ///
    /// // is_active is private, but you can use has_node which calls it
    /// assert!(filtered.has_node(0));  // active
    /// assert!(!filtered.has_node(1)); // not active
    /// assert!(filtered.has_node(2));  // active
    /// ```
    fn is_active(&self, key: u32) -> bool {
        self.active.contains(key as usize)
    }
}

impl<N: NodeTrait, E: EdgeTrait> GraphViewTrait<N, E> for FilteredGraph<'_, N, E> {
    fn node_count(&self) -> usize {
        self.active.count_ones(..)
    }

    fn is_empty(&self) -> bool {
        self.active.count_ones(..) == 0
    }

    fn get_node(&self, key: u32) -> &N {
        if !self.is_active(key) {
            panic!("Node {} not active in filtered graph", key);
        }
        self.base.get_node(key)
    }

    fn has_node(&self, key: u32) -> bool {
        self.is_active(key)
    }

    fn get_nodes<'b>(&'b self) -> impl Iterator<Item = &'b N>
    where
        N: 'b,
    {
        self.base
            .get_nodes()
            .filter(move |n| self.is_active(n.key()))
    }

    fn get_node_keys(&self) -> impl Iterator<Item = u32> {
        self.base
            .get_node_keys()
            .filter(move |&k| self.is_active(k))
    }

    fn get_edge(&self, source: u32, target: u32) -> &E {
        if !self.is_active(source) {
            panic!("Source node {} not active in filtered graph", source);
        }
        if !self.is_active(target) {
            panic!("Target node {} not active in filtered graph", target);
        }
        self.base.get_edge(source, target)
    }

    fn has_edge(&self, source: u32, target: u32) -> bool {
        self.is_active(source) && self.is_active(target) && self.base.has_edge(source, target)
    }

    fn get_edges<'b>(&'b self) -> impl Iterator<Item = &'b E>
    where
        E: 'b,
    {
        self.base
            .get_edges()
            .filter(move |e| self.is_active(e.source()) && self.is_active(e.target()))
    }

    fn get_predecessors<'b>(&'b self, node_key: u32) -> impl Iterator<Item = &'b N>
    where
        N: 'b,
    {
        if !self.is_active(node_key) {
            panic!("Node {} not active in filtered graph", node_key);
        }
        self.base
            .get_node(node_key)
            .predecessors()
            .iter()
            .filter(move |&&pred| self.is_active(pred))
            .map(move |&pred| self.base.get_node(pred))
    }

    fn get_successors<'b>(&'b self, node_key: u32) -> impl Iterator<Item = &'b N>
    where
        N: 'b,
    {
        if !self.is_active(node_key) {
            panic!("Node {} not active in filtered graph", node_key);
        }
        self.base
            .get_node(node_key)
            .successors()
            .iter()
            .filter(move |&&succ| self.is_active(succ))
            .map(move |&succ| self.base.get_node(succ))
    }

    fn get_successors_keys(&self, node_key: u32) -> impl Iterator<Item = u32> {
        if !self.is_active(node_key) {
            panic!("Node {} not active in filtered graph", node_key);
        }
        self.base
            .get_successors_keys(node_key)
            .filter(move |&succ| self.is_active(succ))
    }

    fn get_predecessors_keys(&self, node_key: u32) -> impl Iterator<Item = u32> {
        if !self.is_active(node_key) {
            panic!("Node {} not active in filtered graph", node_key);
        }
        self.base
            .get_predecessors_keys(node_key)
            .filter(move |&pred| self.is_active(pred))
    }

    fn filter(&self, node_keys: &[u32]) -> impl GraphViewTrait<N, E> {
        // Panic if the base graph does not have sequential keys
        if !self.base.has_sequential_keys() {
            panic!("{}", ade_common::INVALID_KEY_SEQUENCE);
        }
        
        // Intersect the requested nodes with the currently active ones
        let filtered_keys = node_keys.iter().copied().filter(|&key| self.is_active(key));

        FilteredGraph::new(self.base, filtered_keys)
    }

    fn has_sequential_keys(&self) -> bool {
        let size = self.active.count_ones(..);
        if size == 0 {
            return true;
        }

        // Quick checks first - check if we have nodes 0 and size-1
        if !self.is_active(0) || !self.is_active(size as u32 - 1) {
            return false;
        }

        // Check if all keys from 0 to size-1 are active
        (0..size as u32).all(|i| self.is_active(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::implementations::{Edge, Node};
    use ade_traits::GraphViewTrait;

    #[test]
    fn test_node_count() {
        let mut base_graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());
        for i in 0..5 {
            base_graph.add_node(Node::new(i));
        }
        let filtered = FilteredGraph::new(&base_graph, vec![0, 1, 2]);
        assert_eq!(filtered.node_count(), 3);
    }

    #[test]
    fn test_filtered_graph_has_sequential_keys() {
        let mut base_graph = Graph::<Node, Edge>::new(Vec::new(), Vec::new());

        // Add nodes 0, 1, 2, 3, 4
        for i in 0..5 {
            base_graph.add_node(Node::new(i));
        }

        // Test 1: Filter to nodes 0, 1, 2 (sequential from 0)
        let filtered = FilteredGraph::new(&base_graph, vec![0, 1, 2]);
        assert!(filtered.has_sequential_keys());

        // Test 2: Filter to nodes 1, 2, 3 (not starting from 0)
        let filtered = FilteredGraph::new(&base_graph, vec![1, 2, 3]);
        assert!(!filtered.has_sequential_keys());

        // Test 3: Filter to nodes 0, 2 (not consecutive)
        let filtered = FilteredGraph::new(&base_graph, vec![0, 2]);
        assert!(!filtered.has_sequential_keys());

        // Test 4: Empty filter
        let filtered = FilteredGraph::new(&base_graph, vec![]);
        assert!(filtered.has_sequential_keys());

        // Test 5: Single node 0
        let filtered = FilteredGraph::new(&base_graph, vec![0]);
        assert!(filtered.has_sequential_keys());

        // Test 6: Single node not 0
        let filtered = FilteredGraph::new(&base_graph, vec![2]);
        assert!(!filtered.has_sequential_keys());
    }
}
