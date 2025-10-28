use std::collections::HashSet;
use std::fmt::Debug;

/// A trait for graph nodes that track their connections.
///
/// A node in a directed graph is identified by a unique `u32` key and maintains
/// sets of predecessor and successor keys to track incoming and outgoing edges.
///
/// # Required Supertrait Bounds
///
/// - [`Debug`] - For debugging and error messages
/// - [`Clone`] - To allow copying nodes when needed
///
/// # Examples
///
/// Using the trait with a concrete implementation:
///
/// ```
/// use ade_graph::implementations::Node;
/// use ade_traits::NodeTrait;
///
/// let mut node = Node::new(1);
/// assert_eq!(node.key(), 1);
///
/// // Add connections
/// node.add_successor(2);
/// node.add_predecessor(0);
///
/// assert!(node.successors().contains(&2));
/// assert!(node.predecessors().contains(&0));
/// ```
pub trait NodeTrait: Debug + Clone {
    /// Creates a new node with the given key.
    ///
    /// The node is initialized with empty predecessor and successor sets.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let node = Node::new(42);
    /// assert_eq!(node.key(), 42);
    /// assert!(node.predecessors().is_empty());
    /// assert!(node.successors().is_empty());
    /// ```
    fn new(key: u32) -> Self;

    /// Returns the unique key identifying this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let node = Node::new(42);
    /// assert_eq!(node.key(), 42);
    /// ```
    fn key(&self) -> u32;

    /// Returns a reference to the set of predecessor node keys.
    ///
    /// Predecessors are nodes that have outgoing edges pointing to this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let mut node = Node::new(2);
    /// node.add_predecessor(1);
    /// assert!(node.predecessors().contains(&1));
    /// ```
    fn predecessors(&self) -> &HashSet<u32>;

    /// Returns a reference to the set of successor node keys.
    ///
    /// Successors are nodes that this node has outgoing edges pointing to.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let mut node = Node::new(1);
    /// node.add_successor(2);
    /// assert!(node.successors().contains(&2));
    /// ```
    fn successors(&self) -> &HashSet<u32>;

    /// Adds a predecessor to this node.
    ///
    /// This method is typically called when an edge is added from another node to this node.
    /// Adding the same predecessor multiple times has no effect (set semantics).
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let mut node = Node::new(2);
    /// node.add_predecessor(1);
    /// assert!(node.predecessors().contains(&1));
    /// 
    /// // Adding again has no effect
    /// node.add_predecessor(1);
    /// assert_eq!(node.predecessors().len(), 1);
    /// ```
    fn add_predecessor(&mut self, key: u32);

    /// Adds a successor to this node.
    ///
    /// This method is typically called when an edge is added from this node to another node.
    /// Adding the same successor multiple times has no effect (set semantics).
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let mut node = Node::new(1);
    /// node.add_successor(2);
    /// assert!(node.successors().contains(&2));
    /// 
    /// // Adding again has no effect
    /// node.add_successor(2);
    /// assert_eq!(node.successors().len(), 1);
    /// ```
    fn add_successor(&mut self, key: u32);

    /// Removes a predecessor from this node.
    ///
    /// This method is typically called when an edge from another node to this node is removed.
    /// Removing a non-existent predecessor has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let mut node = Node::new(2);
    /// node.add_predecessor(1);
    /// node.remove_predecessor(1);
    /// assert!(!node.predecessors().contains(&1));
    /// 
    /// // Removing again has no effect
    /// node.remove_predecessor(1);
    /// assert_eq!(node.predecessors().len(), 0);
    /// ```
    fn remove_predecessor(&mut self, key: u32);

    /// Removes a successor from this node.
    ///
    /// This method is typically called when an edge from this node to another node is removed.
    /// Removing a non-existent successor has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Node;
    /// use ade_traits::NodeTrait;
    ///
    /// let mut node = Node::new(1);
    /// node.add_successor(2);
    /// node.remove_successor(2);
    /// assert!(!node.successors().contains(&2));
    /// 
    /// // Removing again has no effect
    /// node.remove_successor(2);
    /// assert_eq!(node.successors().len(), 0);
    /// ```
    fn remove_successor(&mut self, key: u32);
}
