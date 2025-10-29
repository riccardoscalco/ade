use ade_traits::EdgeTrait;

/// A lightweight directed edge implementation connecting two nodes in a graph.
///
/// `Edge` is a concrete implementation of the [`EdgeTrait`] that stores only the source
/// and target node keys. It represents a directed connection from a source node to a target
/// node in a directed graph.
///
/// # Memory Efficiency
///
/// This implementation uses only 8 bytes (two `u32` values), making it very memory-efficient
/// for graphs with many edges.
///
/// # Examples
///
/// Creating and using edges:
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
/// ```
///
/// Edges are directional:
///
/// ```
/// use ade_graph::implementations::Edge;
/// use ade_traits::EdgeTrait;
///
/// let edge_1_to_2 = Edge::new(1, 2);
/// let edge_2_to_1 = Edge::new(2, 1);
///
/// // Different directions means different edges
/// assert_ne!(edge_1_to_2.key(), edge_2_to_1.key());
/// ```
///
/// Self-loops are supported:
///
/// ```
/// use ade_graph::implementations::Edge;
/// use ade_traits::EdgeTrait;
///
/// let self_loop = Edge::new(5, 5);
/// assert_eq!(self_loop.source(), self_loop.target());
/// ```
#[derive(Debug, Clone)]
pub struct Edge {
    source: u32,
    target: u32,
}

impl Edge {
    /// Creates a new directed edge from a source node to a target node.
    ///
    /// # Parameters
    ///
    /// * `source` - The key of the source node where the edge originates
    /// * `target` - The key of the target node where the edge points to
    ///
    /// # Returns
    ///
    /// A new `Edge` instance representing a directed connection from `source` to `target`.
    ///
    /// # Examples
    ///
    /// Creating a simple edge:
    ///
    /// ```
    /// use ade_graph::implementations::Edge;
    ///
    /// let edge = Edge::new(1, 2);
    /// ```
    ///
    /// Creating a self-loop:
    ///
    /// ```
    /// use ade_graph::implementations::Edge;
    ///
    /// // Self-loops are valid
    /// let self_loop = Edge::new(5, 5);
    /// ```
    ///
    /// # Note
    ///
    /// This constructor does not validate whether the source and target nodes actually exist
    /// in any graph. It simply creates the edge data structure. Validation typically occurs
    /// when adding the edge to a graph.
    pub fn new(source: u32, target: u32) -> Self {
        Self { source, target }
    }
}

impl EdgeTrait for Edge {
    fn new(source: u32, target: u32) -> Self {
        Edge::new(source, target)
    }

    fn source(&self) -> u32 {
        self.source
    }

    fn target(&self) -> u32 {
        self.target
    }

    fn key(&self) -> (u32, u32) {
        (self.source, self.target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let edge = Edge::new(1, 2);
        assert_eq!(edge.source(), 1);
        assert_eq!(edge.target(), 2);
    }

    #[test]
    fn test_source() {
        let edge = Edge::new(5, 10);
        assert_eq!(edge.source(), 5);
    }

    #[test]
    fn test_target() {
        let edge = Edge::new(5, 10);
        assert_eq!(edge.target(), 10);
    }

    #[test]
    fn test_key() {
        let edge = Edge::new(1, 2);
        assert_eq!(edge.key(), (1, 2));
    }

    #[test]
    fn test_different_edges_different_keys() {
        let edge1 = Edge::new(1, 2);
        let edge2 = Edge::new(2, 1);
        let edge3 = Edge::new(1, 3);

        assert_ne!(edge1.key(), edge2.key());
        assert_ne!(edge1.key(), edge3.key());
        assert_ne!(edge2.key(), edge3.key());
    }

    #[test]
    fn test_same_edges_same_keys() {
        let edge1 = Edge::new(1, 2);
        let edge2 = Edge::new(1, 2);

        assert_eq!(edge1.key(), edge2.key());
        assert_eq!(edge1.source(), edge2.source());
        assert_eq!(edge1.target(), edge2.target());
    }

    #[test]
    fn test_edge_trait_implementation() {
        let edge = Edge::new(42, 99);

        // Test that EdgeTrait methods work correctly
        assert_eq!(edge.source(), 42);
        assert_eq!(edge.target(), 99);
    }
}
