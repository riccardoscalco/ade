use ade_traits::{EdgeTrait, GraphViewTrait, NodeTrait};
use std::collections::HashMap;

/// Finds all strongly connected components (SCCs) in a directed graph.
///
/// This function implements D.J. Pearce's algorithm for finding strongly connected components,
/// as described in "An Improved Algorithm for Finding the Strongly Connected Components of a
/// Directed Graph" (Information Processing Letters 116, 2016, 47-52). This is the recursive
/// implementation of the algorithm.
///
/// A strongly connected component is a maximal set of vertices where every vertex is reachable
/// from every other vertex in the set. This implementation uses a depth-first search approach
/// with optimizations that make it efficient for large graphs.
///
/// # Type Parameters
///
/// * `N` - The node type, which must implement [`NodeTrait`]
/// * `E` - The edge type, which must implement [`EdgeTrait`]
///
/// # Parameters
///
/// * `graph` - A reference to any graph structure implementing [`GraphViewTrait`]
///
/// # Returns
///
/// A vector of strongly connected components, where each component is represented as a vector
/// of node keys (`u32`). The order of components and the order of nodes within each component
/// is not specified.
///
/// Returns an empty vector if the graph is empty.
///
/// # Examples
///
/// ```
/// use ade_strongly_connected_components::pearce_recursive::scc;
/// use ade_graph::implementations::{Node, Edge};
/// use ade_graph::utils::build::build_graph;
///
/// // Graph with one SCC: 1 -> 2 -> 3 -> 1, and a separate node 4
/// let graph = build_graph::<Node, Edge>(
///     vec![1, 2, 3, 4],
///     vec![(1, 2), (2, 3), (3, 1), (3, 4)],
/// );
///
/// let components = scc(&graph);
/// assert_eq!(components.len(), 2);
///
/// // One component contains nodes 1, 2, 3 (a cycle)
/// // Another component contains only node 4
/// assert!(components.iter().any(|c| c.len() == 3 && c.contains(&1) && c.contains(&2) && c.contains(&3)));
/// assert!(components.iter().any(|c| c.len() == 1 && c.contains(&4)));
/// ```
///
/// ```
/// use ade_strongly_connected_components::pearce_recursive::scc;
/// use ade_graph::implementations::{Node, Edge};
/// use ade_graph::utils::build::build_graph;
///
/// // Linear graph: 1 -> 2 -> 3 -> 4 (no cycles)
/// let graph = build_graph::<Node, Edge>(
///     vec![1, 2, 3, 4],
///     vec![(1, 2), (2, 3), (3, 4)],
/// );
///
/// let components = scc(&graph);
/// // Each node is its own SCC
/// assert_eq!(components.len(), 4);
/// ```
///
/// ```
/// use ade_strongly_connected_components::pearce_recursive::scc;
/// use ade_graph::implementations::{Node, Edge};
/// use ade_graph::utils::build::build_graph;
///
/// // Fully connected cycle: 1 -> 2 -> 3 -> 4 -> 1
/// let graph = build_graph::<Node, Edge>(
///     vec![1, 2, 3, 4],
///     vec![(1, 2), (2, 3), (3, 4), (4, 1)],
/// );
///
/// let components = scc(&graph);
/// // All nodes form a single SCC
/// assert_eq!(components.len(), 1);
/// assert_eq!(components[0].len(), 4);
/// ```
pub fn scc<N: NodeTrait, E: EdgeTrait>(graph: &impl GraphViewTrait<N, E>) -> Vec<Vec<u32>> {
    if graph.is_empty() {
        return Vec::new();
    }

    let mut rindex: HashMap<u32, usize> = graph.get_nodes().map(|node| (node.key(), 0)).collect();
    let mut stack: Vec<u32> = Vec::new();
    let mut index: usize = 1;
    let mut c: usize = graph.get_nodes().count() - 1;

    for node in graph.get_nodes() {
        if rindex[&node.key()] == 0 {
            visit(node, graph, &mut rindex, &mut index, &mut stack, &mut c);
        }
    }

    fn visit<N: NodeTrait, E: EdgeTrait>(
        node: &N,
        graph: &impl GraphViewTrait<N, E>,
        rindex: &mut HashMap<u32, usize>,
        index: &mut usize,
        stack: &mut Vec<u32>,
        c: &mut usize,
    ) {
        let mut root: bool = true;
        let v = node.key();
        rindex.insert(v, *index);
        *index += 1;

        for w in node.successors() {
            if rindex[w] == 0 {
                visit(graph.get_node(*w), graph, rindex, index, stack, c);
            }
            if rindex[w] < rindex[&v] {
                rindex.insert(v, rindex[w]);
                root = false;
            }
        }

        if root {
            *index -= 1;

            while !stack.is_empty() && rindex[&v] <= rindex[stack.last().unwrap()] {
                let w = stack.pop().unwrap();
                rindex.insert(w, *c);
                *index -= 1;
            }

            rindex.insert(v, *c);
            if *c == 0 {
                return;
            }
            *c -= 1;
        } else {
            stack.push(v);
        }
    }

    components(&rindex)
}

// Utility function to extract components from the rindex
fn components(rindex: &HashMap<u32, usize>) -> Vec<Vec<u32>> {
    let mut components_map: HashMap<usize, Vec<u32>> = HashMap::new();

    for (node_key, component_id) in rindex {
        components_map
            .entry(*component_id)
            .or_default()
            .push(*node_key);
    }

    components_map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ade_graph::{implementations::{Edge, Node}, utils::build::build_graph};
    //use graph_generators::generate_random_graph_data;

    fn sort_components(components: &mut Vec<Vec<u32>>) {
        for component in components.iter_mut() {
            component.sort_unstable();
        }
        components.sort_unstable_by_key(|g| g[0]);
    }

    #[test]
    fn test_scc_1() {
        let graph = build_graph::<Node, Edge>(vec![1, 2, 3, 4], vec![(1, 2), (2, 3), (3, 1), (3, 4)]);
        let components = scc(&graph);
        assert_eq!(components.len(), 2);
        // Test that there are components [1, 2, 3] and [4]
        assert!(components
            .iter()
            .any(|c| c.len() == 3 && c.contains(&1) && c.contains(&2) && c.contains(&3)));
        assert!(components.iter().any(|c| c.len() == 1 && c.contains(&4)));
    }

    #[test]
    fn test_scc_2() {
        let graph = build_graph::<Node, Edge>(vec![1, 2, 3, 4], vec![(1, 2), (2, 3), (3, 4)]);
        let components = scc(&graph);
        assert_eq!(components.len(), 4);
    }

    #[test]
    fn test_scc_3() {
        let graph = build_graph::<Node, Edge>(vec![1, 2, 3, 4], vec![(1, 2), (2, 3), (3, 4), (4, 1)]);
        let components = scc(&graph);
        assert_eq!(components.len(), 1);
    }

    #[test]
    fn test_scc_4() {
        let graph = build_graph::<Node, Edge>(vec![1, 2, 3, 4], vec![(1, 2), (2, 3), (3, 4), (4, 3)]);
        let components = scc(&graph);
        assert_eq!(components.len(), 3);
        // Test that there are components [1], [2] and [3, 4]
        assert!(components.iter().any(|c| c.len() == 1 && c.contains(&1)));
        assert!(components.iter().any(|c| c.len() == 1 && c.contains(&2)));
        assert!(components
            .iter()
            .any(|c| c.len() == 2 && c.contains(&3) && c.contains(&4)));
    }

    #[test]
    fn test_scc_5() {
        let graph = build_graph::<Node, Edge>(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![
                (0, 1),
                (0, 4),
                (1, 2),
                (2, 3),
                (4, 7),
                (3, 1),
                (4, 0),
                (4, 5),
                (5, 6),
                (6, 4),
                (8, 9),
                (9, 8),
            ],
        );
        let mut components = scc(&graph);
        sort_components(&mut components);
        assert_eq!(
            components,
            vec![vec![0, 4, 5, 6], vec![1, 2, 3], vec![7], vec![8, 9]]
        );
    }
}
