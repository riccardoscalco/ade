mod graph;
mod hierarchy;

use graph::edge::Edge;
use graph::node::Node;

fn main() {
    let mut graph = graph::Graph::new(Vec::new(), Vec::new());

    graph.add_node(Node::new("A"));
    graph.add_node(Node::new("B"));
    graph.add_node(Node::new("C"));
    graph.add_edge(Edge::new("A", "B"));
    graph.add_edge(Edge::new("A", "C"));

    println!("{}", graph);
}
