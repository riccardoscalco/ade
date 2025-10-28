use std::fmt::Debug;

/// A trait for directed graph edges connecting two nodes.
///
/// An edge in a directed graph represents a connection from a source node to a target node.
/// Each edge is uniquely identified by the pair (source, target), allowing for multiple edges
/// between different pairs of nodes but only one edge per source-target pair.
///
/// # Required Supertrait Bounds
///
/// - [`Debug`] - For debugging and error messages
/// - [`Clone`] - To allow copying edges when needed
///
/// # Examples
///
/// Using the trait with a concrete implementation:
///
/// ```
/// use ade_graph::implementations::Edge;
/// use ade_traits::EdgeTrait;
///
/// // Create a directed edge from node 1 to node 2
/// let edge = Edge::new(1, 2);
/// assert_eq!(edge.source(), 1);
/// assert_eq!(edge.target(), 2);
/// assert_eq!(edge.key(), (1, 2));
///
/// // Self-loops are allowed
/// let self_loop = Edge::new(5, 5);
/// assert_eq!(self_loop.source(), 5);
/// assert_eq!(self_loop.target(), 5);
/// ```
pub trait EdgeTrait: Debug + Clone {
    /// Creates a new edge from source to target.
    ///
    /// The edge represents a directed connection from the source node to the target node.
    /// Self-loops (where source equals target) are allowed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Edge;
    /// use ade_traits::EdgeTrait;
    ///
    /// let edge = Edge::new(1, 2);
    /// assert_eq!(edge.source(), 1);
    /// assert_eq!(edge.target(), 2);
    ///
    /// // Self-loop
    /// let loop_edge = Edge::new(3, 3);
    /// assert_eq!(loop_edge.source(), loop_edge.target());
    /// ```
    fn new(source: u32, target: u32) -> Self;

    /// Returns the source node key of this edge.
    ///
    /// The source is the node from which this directed edge originates.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Edge;
    /// use ade_traits::EdgeTrait;
    ///
    /// let edge = Edge::new(42, 99);
    /// assert_eq!(edge.source(), 42);
    /// ```
    fn source(&self) -> u32;

    /// Returns the target node key of this edge.
    ///
    /// The target is the node to which this directed edge points.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Edge;
    /// use ade_traits::EdgeTrait;
    ///
    /// let edge = Edge::new(42, 99);
    /// assert_eq!(edge.target(), 99);
    /// ```
    fn target(&self) -> u32;

    /// Returns the unique key identifying this edge.
    ///
    /// The key is a tuple `(source, target)` that uniquely identifies the edge in a graph.
    /// Two edges with the same key represent the same connection, even if they are
    /// separate instances.
    ///
    /// # Examples
    ///
    /// ```
    /// use ade_graph::implementations::Edge;
    /// use ade_traits::EdgeTrait;
    ///
    /// let edge = Edge::new(1, 2);
    /// assert_eq!(edge.key(), (1, 2));
    ///
    /// // Direction matters: (1,2) ≠ (2,1)
    /// let reverse_edge = Edge::new(2, 1);
    /// assert_ne!(edge.key(), reverse_edge.key());
    /// ```
    fn key(&self) -> (u32, u32);
}
