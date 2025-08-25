use graph::node::Node;
use graph::NodeTrait;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Base,
    Dummy,
    UpperBorder,
    LowerBorder,
    LeftBorder,
    RightBorder,
}

#[derive(Debug)]
pub struct HierarchyNode {
    node: Node,
    node_type: NodeType,
    layer: usize,
    rank: usize,
    x: f64,
    y: f64,
}

impl HierarchyNode {
    pub fn new(id: &str, node_type: NodeType) -> Self {
        HierarchyNode {
            node: Node::new(id),
            node_type,
            layer: 0,
            rank: 0,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn layer(&self) -> usize {
        self.layer
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.layer = layer;
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn set_rank(&mut self, rank: usize) {
        self.rank = rank;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}

impl NodeTrait for HierarchyNode {
    fn id(&self) -> &str {
        self.node.id()
    }

    fn predecessors(&self) -> &HashSet<String> {
        self.node.predecessors()
    }

    fn successors(&self) -> &HashSet<String> {
        self.node.successors()
    }

    fn add_predecessor(&mut self, id: &str) {
        self.node.add_predecessor(id);
    }

    fn add_successor(&mut self, id: &str) {
        self.node.add_successor(id);
    }

    fn remove_predecessor(&mut self, id: &str) {
        self.node.remove_predecessor(id);
    }

    fn remove_successor(&mut self, id: &str) {
        self.node.remove_successor(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_node_creation() {
        let node = HierarchyNode::new("A", NodeType::Base);
        assert_eq!(node.id(), "A");
        assert_eq!(node.node_type(), &NodeType::Base);
        assert_eq!(node.layer(), 0);
        assert_eq!(node.rank(), 0);
        assert_eq!(node.x(), 0.0);
        assert_eq!(node.y(), 0.0);
    }

    #[test]
    fn test_hierarchy_node_setters() {
        let mut node = HierarchyNode::new("A", NodeType::Base);
        node.set_layer(1);
        node.set_rank(2);
        node.set_x(3.0);
        node.set_y(4.0);

        assert_eq!(node.layer(), 1);
        assert_eq!(node.rank(), 2);
        assert_eq!(node.x(), 3.0);
        assert_eq!(node.y(), 4.0);
    }
}
