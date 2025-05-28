use std::collections::HashSet;

#[derive(Debug)]
pub struct Node {
    id: String,
    predecessors: HashSet<String>,
    successors: HashSet<String>,
}

impl Node {
    pub fn new(id: &str) -> Self {
        Node {
            id: id.to_string(),
            predecessors: HashSet::new(),
            successors: HashSet::new(),
        }
    }
}

use crate::graph::core::NodeTrait;

impl NodeTrait for Node {
    fn id(&self) -> &str {
        &self.id
    }

    fn predecessors(&self) -> &HashSet<String> {
        &self.predecessors
    }

    fn successors(&self) -> &HashSet<String> {
        &self.successors
    }

    fn add_predecessor(&mut self, id: &str) {
        self.predecessors.insert(id.to_string());
    }

    fn add_successor(&mut self, id: &str) {
        self.successors.insert(id.to_string());
    }

    fn remove_predecessor(&mut self, id: &str) {
        self.predecessors.remove(id);
    }

    fn remove_successor(&mut self, id: &str) {
        self.successors.remove(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new("A");
        assert_eq!(node.id(), "A");
        assert!(node.predecessors().is_empty());
        assert!(node.successors().is_empty());
    }

    #[test]
    fn test_add_predecessor() {
        let mut node = Node::new("A");
        node.add_predecessor("B");
        assert!(node.predecessors().contains("B"));
    }

    #[test]
    fn test_add_successor() {
        let mut node = Node::new("A");
        node.add_successor("B");
        assert!(node.successors().contains("B"));
    }

    #[test]
    fn test_remove_predecessor() {
        let mut node = Node::new("A");
        node.add_predecessor("B");
        node.remove_predecessor("B");
        assert!(!node.predecessors().contains("B"));
    }

    #[test]
    fn test_remove_successor() {
        let mut node = Node::new("A");
        node.add_successor("B");
        node.remove_successor("B");
        assert!(!node.successors().contains("B"));
    }
}
