#[derive(Debug)]
pub struct Edge {
    id: String,
    source: String,
    target: String,
}

impl Edge {
    pub fn new(source: &str, target: &str) -> Self {
        Edge {
            id: Self::make_edge_id(source, target),
            source: source.to_string(),
            target: target.to_string(),
        }
    }
}

use crate::graph::core::EdgeTrait;

impl EdgeTrait for Edge {
    fn id(&self) -> &str {
        &self.id
    }

    fn source(&self) -> &str {
        &self.source
    }

    fn target(&self) -> &str {
        &self.target
    }

    fn make_edge_id(source: &str, target: &str) -> String {
        format!("{}->{}", source, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let edge = Edge::new("A", "B");
        assert_eq!(edge.id(), "A->B");
    }

    #[test]
    fn test_source() {
        let edge = Edge::new("A", "B");
        assert_eq!(edge.source(), "A");
    }

    #[test]
    fn test_target() {
        let edge = Edge::new("A", "B");
        assert_eq!(edge.target(), "B");
    }

    #[test]
    fn test_make_edge_id() {
        let edge_id = Edge::make_edge_id("A", "B");
        assert_eq!(edge_id, "A->B");
    }

    #[test]
    fn test_edge_creation() {
        let edge = Edge::new("A", "B");
        assert_eq!(edge.source(), "A");
        assert_eq!(edge.target(), "B");
    }
}
