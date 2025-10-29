use ade_traits::NodeTrait;
use std::collections::HashSet;

/// A node in a directed graph that tracks its connections to other nodes.
///
/// `Node` is a concrete implementation of the [`NodeTrait`] that maintains a unique key
/// identifier and keeps track of all incoming connections (predecessors) and outgoing
/// connections (successors) using hash sets for efficient lookup and modification.
///
/// # Structure
///
/// - **key**: A unique `u32` identifier for the node within the graph
/// - **predecessors**: A [`HashSet`] of keys representing nodes that have edges pointing to this node
/// - **successors**: A [`HashSet`] of keys representing nodes that this node has edges pointing to
///
/// # Memory Characteristics
///
/// The memory footprint of a node grows with the number of connections. Each connection
/// stores only a `u32` key, making the representation space-efficient even for nodes with
/// many connections.
///
/// # Examples
///
/// Creating and connecting nodes:
///
/// ```
/// use ade_graph::implementations::Node;
/// use ade_traits::NodeTrait;
///
/// // Create a new node with key 1
/// let mut node = Node::new(1);
/// assert_eq!(node.key(), 1);
/// assert!(node.predecessors().is_empty());
/// assert!(node.successors().is_empty());
///
/// // Add connections
/// node.add_predecessor(2); // Node 2 points to this node
/// node.add_successor(3);   // This node points to node 3
///
/// assert!(node.predecessors().contains(&2));
/// assert!(node.successors().contains(&3));
/// ```
///
/// Working with multiple connections:
///
/// ```
/// use ade_graph::implementations::Node;
/// use ade_traits::NodeTrait;
///
/// let mut hub_node = Node::new(10);
///
/// // Add multiple predecessors
/// hub_node.add_predecessor(1);
/// hub_node.add_predecessor(2);
/// hub_node.add_predecessor(3);
///
/// // Add multiple successors
/// hub_node.add_successor(20);
/// hub_node.add_successor(30);
///
/// assert_eq!(hub_node.predecessors().len(), 3);
/// assert_eq!(hub_node.successors().len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Node {
    key: u32,
    predecessors: HashSet<u32>,
    successors: HashSet<u32>,
}

impl Node {
    /// Creates a new node with the specified key and no connections.
    ///
    /// The node is initialized with empty sets of predecessors and successors. Connections
    /// to other nodes must be added explicitly using the [`NodeTrait`] methods such as
    /// [`add_predecessor`](NodeTrait::add_predecessor) and [`add_successor`](NodeTrait::add_successor).
    ///
    /// # Parameters
    ///
    /// * `key` - A unique `u32` identifier for the node. The uniqueness is typically enforced
    ///   by the graph container, not by the node itself.
    ///
    /// # Returns
    ///
    /// A new `Node` instance with the given key and no connections.
    ///
    /// # Examples
    ///
    /// Creating a standalone node:
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let node = Node::new(42);
    /// assert_eq!(node.key(), 42);
    /// assert_eq!(node.predecessors().len(), 0);
    /// assert_eq!(node.successors().len(), 0);
    /// ```
    ///
    /// Creating multiple nodes:
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let node1 = Node::new(1);
    /// let node2 = Node::new(2);
    /// let node3 = Node::new(3);
    ///
    /// assert_ne!(node1.key(), node2.key());
    /// assert_ne!(node2.key(), node3.key());
    /// ```
    ///
    /// # Note
    ///
    /// This constructor does not establish any connections with other nodes. To build a graph,
    /// nodes must be added to a graph container which manages the connections between them.
    pub fn new(key: u32) -> Self {
        Node {
            key,
            predecessors: HashSet::new(),
            successors: HashSet::new(),
        }
    }
}

impl NodeTrait for Node {
    fn new(key: u32) -> Self {
        Node::new(key)
    }

    fn key(&self) -> u32 {
        self.key
    }

    fn predecessors(&self) -> &HashSet<u32> {
        &self.predecessors
    }

    fn successors(&self) -> &HashSet<u32> {
        &self.successors
    }

    fn add_predecessor(&mut self, key: u32) {
        self.predecessors.insert(key);
    }

    fn add_successor(&mut self, key: u32) {
        self.successors.insert(key);
    }

    fn remove_predecessor(&mut self, key: u32) {
        self.predecessors.remove(&key);
    }

    fn remove_successor(&mut self, key: u32) {
        self.successors.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(1);
        assert_eq!(node.key(), 1);
        assert!(node.predecessors().is_empty());
        assert!(node.successors().is_empty());
    }

    #[test]
    fn test_add_predecessor() {
        let mut node = Node::new(1);
        node.add_predecessor(2);
        assert!(node.predecessors().contains(&2));
    }

    #[test]
    fn test_add_successor() {
        let mut node = Node::new(1);
        node.add_successor(2);
        assert!(node.successors().contains(&2));
    }

    #[test]
    fn test_remove_predecessor() {
        let mut node = Node::new(1);
        node.add_predecessor(2);
        node.remove_predecessor(2);
        assert!(!node.predecessors().contains(&2));
    }

    #[test]
    fn test_remove_successor() {
        let mut node = Node::new(1);
        node.add_successor(2);
        node.remove_successor(2);
        assert!(!node.successors().contains(&2));
    }

    #[test]
    fn test_multiple_connections() {
        let mut node = Node::new(1);

        // Add multiple predecessors and successors
        node.add_predecessor(2);
        node.add_predecessor(3);
        node.add_successor(4);
        node.add_successor(5);

        assert_eq!(node.predecessors().len(), 2);
        assert_eq!(node.successors().len(), 2);
        assert!(node.predecessors().contains(&2));
        assert!(node.predecessors().contains(&3));
        assert!(node.successors().contains(&4));
        assert!(node.successors().contains(&5));

        // Remove some connections
        node.remove_predecessor(2);
        node.remove_successor(4);

        assert_eq!(node.predecessors().len(), 1);
        assert_eq!(node.successors().len(), 1);
        assert!(!node.predecessors().contains(&2));
        assert!(node.predecessors().contains(&3));
        assert!(!node.successors().contains(&4));
        assert!(node.successors().contains(&5));
    }

    #[test]
    fn test_duplicate_connections() {
        let mut node = Node::new(1);

        // Adding the same predecessor twice should not duplicate
        node.add_predecessor(2);
        node.add_predecessor(2);
        assert_eq!(node.predecessors().len(), 1);

        // Adding the same successor twice should not duplicate
        node.add_successor(3);
        node.add_successor(3);
        assert_eq!(node.successors().len(), 1);
    }
}
