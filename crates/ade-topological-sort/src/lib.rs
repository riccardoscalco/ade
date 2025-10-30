use std::cmp::Reverse;
use ade_common::INVALID_KEY_SEQUENCE;
use ade_traits::{EdgeTrait, GraphViewTrait, NodeTrait};
use fixedbitset::FixedBitSet;

pub const CYCLE_ERROR_MSG: &str = "Graph contains a cycle";

/// Performs a topological sort on a directed acyclic graph (DAG).
///
/// A topological sort is a linear ordering of nodes such that for every directed edge
/// from node `u` to node `v`, `u` comes before `v` in the ordering. This is only possible
/// if the graph is acyclic (contains no cycles).
///
/// When multiple valid topological orderings exist, the optional `key_fn` parameter can be
/// used to determine a consistent ordering based on a comparison key.
///
/// # Type Parameters
///
/// * `N` - The node type, which must implement [`NodeTrait`]
/// * `E` - The edge type, which must implement [`EdgeTrait`]
/// * `K` - The type of the comparison key, which must implement [`Ord`]
/// * `F` - A function that extracts a comparison key from a node
///
/// # Parameters
///
/// * `graph` - A reference to a graph that implements [`GraphViewTrait`]
/// * `key_fn` - An optional function to determine ordering when multiple valid topological
///              orderings exist. Nodes will be processed in descending order of their keys.
///
/// # Returns
///
/// Returns `Ok(Vec<u32>)` containing the node keys in topological order, or
/// `Err(String)` if the graph contains a cycle.
///
/// # Panics
///
/// Panics if the graph does not have sequential keys starting from 0.
/// Use [`has_sequential_keys`](GraphViewTrait::has_sequential_keys) to verify this requirement.
///
/// # Errors
///
/// Returns an error with message [`CYCLE_ERROR_MSG`] if the graph contains a cycle.
/// A cycle makes topological sorting impossible since there would be no valid linear ordering.
///
/// # Examples
///
/// Basic topological sort without a key function:
///
/// ```
/// use ade_topological_sort::topological_sort;
/// use ade_graph::implementations::{Graph, Node, Edge};
/// use ade_traits::NodeTrait;
///
/// // Create a simple DAG: 0 -> 1 -> 2
/// let graph = Graph::new(
///     vec![Node::new(0), Node::new(1), Node::new(2)],
///     vec![Edge::new(0, 1), Edge::new(1, 2)],
/// );
///
/// let sorted = topological_sort::<Node, Edge, u32, fn(&Node) -> u32>(&graph, None).unwrap();
/// assert_eq!(sorted, vec![0, 1, 2]);
/// ```
///
/// Using a key function to control ordering:
///
/// ```
/// use ade_topological_sort::topological_sort;
/// use ade_graph::implementations::{Graph, Node, Edge};
/// use ade_traits::NodeTrait;
///
/// // Create a graph with multiple valid orderings: 0 -> 2, 1 -> 2
/// let graph = Graph::new(
///     vec![Node::new(0), Node::new(1), Node::new(2)],
///     vec![Edge::new(0, 2), Edge::new(1, 2)],
/// );
///
/// // Sort in ascending order of node keys
/// let sort_fn = |n: &Node| n.key();
/// let sorted = topological_sort(&graph, Some(sort_fn)).unwrap();
/// assert_eq!(sorted, vec![0, 1, 2]);
///
/// // Sort in descending order of node keys
/// let reverse_sort_fn = |n: &Node| -(n.key() as i32);
/// let sorted = topological_sort(&graph, Some(reverse_sort_fn)).unwrap();
/// assert_eq!(sorted, vec![1, 0, 2]);
/// ```
///
/// Detecting cycles:
///
/// ```
/// use ade_topological_sort::{topological_sort, CYCLE_ERROR_MSG};
/// use ade_graph::implementations::{Graph, Node, Edge};
///
/// // Create a graph with a cycle: 0 -> 1 -> 0
/// let graph = Graph::new(
///     vec![Node::new(0), Node::new(1)],
///     vec![Edge::new(0, 1), Edge::new(1, 0)],
/// );
///
/// let result = topological_sort::<Node, Edge, u32, fn(&Node) -> u32>(&graph, None);
/// assert!(result.is_err());
/// assert_eq!(result.unwrap_err(), CYCLE_ERROR_MSG);
/// ```
pub fn topological_sort<N, E, K, F>(
    graph: &impl GraphViewTrait<N, E>,
    key_fn: Option<F>,
) -> Result<Vec<u32>, String>
where
    N: NodeTrait,
    E: EdgeTrait,
    K: Ord,
    F: Fn(&N) -> K,
{
    fn visit<N, E, K, F>(
        mut nodes: Vec<&N>,
        graph: &impl GraphViewTrait<N, E>,
        visiting: &mut FixedBitSet,
        visited: &mut FixedBitSet,
        result: &mut Vec<u32>,
        key_fn: &Option<F>,
    ) -> Result<(), String>
    where
        N: NodeTrait,
        E: EdgeTrait,
        K: Ord,
        F: Fn(&N) -> K,
    {
        match key_fn {
            Some(f) => {
                nodes.sort_by_key(|n| Reverse(f(n)));
            }
            None => (),
        }

        for node in nodes {
            dfs(node.key(), graph, visiting, visited, result, key_fn)?;
        }

        Ok(())
    }

    fn dfs<N, E, K, F>(
        node_key: u32,
        graph: &impl GraphViewTrait<N, E>,
        visiting: &mut FixedBitSet,
        visited: &mut FixedBitSet,
        result: &mut Vec<u32>,
        key_fn: &Option<F>,
    ) -> Result<(), String>
    where
        N: NodeTrait,
        E: EdgeTrait,
        K: Ord,
        F: Fn(&N) -> K,
    {
        let idx = node_key as usize;

        if visited[idx] {
            return Ok(());
        }

        if visiting[idx] {
            return Err(CYCLE_ERROR_MSG.into());
        }

        visiting.set(idx, true);

        visit(graph.get_successors(node_key).collect(), graph, visiting, visited, result, key_fn)?;

        visiting.set(idx, false);
        visited.set(idx, true);
        result.push(node_key);
        Ok(())
    }

    // Panic if the graph does not have sequential keys
    if !graph.has_sequential_keys() {
        panic!("{}", INVALID_KEY_SEQUENCE);
    }

    let node_count = graph.node_count();
    let mut result = Vec::with_capacity(node_count);

    // Initialize bit sets for visiting and visited nodes
    let mut visiting = FixedBitSet::with_capacity(node_count);
    let mut visited = FixedBitSet::with_capacity(node_count);

    visit(graph.get_nodes().collect(), graph, &mut visiting, &mut visited, &mut result, &key_fn)?;

    result.reverse();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ade_graph::implementations::Edge;
    use ade_graph::implementations::Graph;
    use ade_graph::implementations::Node;
    use ade_graph::utils::build::build_graph;
    use ade_graph_generators::generate_random_graph_data;

    #[test]
    fn test_topological_sort() {
        let n1 = Node::new(0);
        let n2 = Node::new(1);
        let n3 = Node::new(2);

        let e1 = Edge::new(0, 1);
        let e2 = Edge::new(1, 2);

        let graph = Graph::<Node, Edge>::new(vec![n1, n2, n3], vec![e1, e2]);
        let sorted = topological_sort::<Node, Edge, u32, fn(&Node) -> u32>(&graph, None).unwrap();

        assert_eq!(sorted, vec![0, 1, 2]);
    }

    #[test]
    fn test_topological_sort_non_sequential_keys() {
        use ade_common::assert_panics_with;

        let graph = build_graph(vec![1, 3, 5], vec![(1, 3), (3, 5), (5, 1)]);
        assert_panics_with!(
            topological_sort::<Node, Edge, u32, fn(&Node) -> u32>(&graph, None),
            ade_common::INVALID_KEY_SEQUENCE
        );
    }

    #[test]
    fn test_topological_sort_cycle() {
        let n1 = Node::new(0);
        let n2 = Node::new(1);

        let e1 = Edge::new(0, 1);
        let e2 = Edge::new(1, 0); // creates a cycle

        let graph = Graph::<Node, Edge>::new(vec![n1, n2], vec![e1, e2]);

        let result = topological_sort::<Node, Edge, u32, fn(&Node) -> u32>(&graph, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CYCLE_ERROR_MSG);
    }

    #[test]
    fn test_topological_sort_with_compare_by_key_1() {
        let graph1 = build_graph(vec![0, 1, 2], vec![(0, 1), (0, 2)]);

        let sort_fn = |n: &Node| (n.key() as u32);
        let reverse_sort_fn = |n: &Node| -(n.key() as i32);

        assert_eq!(
            topological_sort::<Node, Edge, u32, _>(
                &graph1,
                Some(sort_fn)
            )
            .unwrap(),
            vec![0, 1, 2]
        );

        assert_eq!(
            topological_sort::<Node, Edge, i32, _>
                (&graph1,
                Some(reverse_sort_fn)
            ).unwrap(),
            vec![0, 2, 1]
        );

        let graph2 = build_graph(vec![0, 1, 2], vec![(0, 2), (1, 2)]);

        assert_eq!(
            topological_sort::<Node, Edge, u32, _>(
                &graph2,
                Some(sort_fn)
            ).unwrap(),
            vec![0, 1, 2]
        );

        assert_eq!(
            topological_sort::<Node, Edge, i32, _>(
                &graph2,
                Some(reverse_sort_fn)
            ).unwrap(),
            vec![1, 0, 2]
        );

        let graph3 = build_graph(vec![0, 1, 2, 3, 4], vec![(0, 1), (0, 4), (2, 4), (2, 3)]);

        assert_eq!(
            topological_sort::<Node, Edge, u32, _>(
                &graph3,
                Some(sort_fn)
            ).unwrap(),
            vec![0, 1, 2, 3, 4]
        );

        assert_eq!(
            topological_sort::<Node, Edge, i32, _>(
                &graph3,
                Some(reverse_sort_fn)
            ).unwrap(),
            vec![2, 3, 0, 4, 1]
        );
    }

    #[test]
    fn test_topological_sort_random_graph() {
        let (nodes, edges) = generate_random_graph_data(20, 20, 3);
        let graph = build_graph(nodes, edges);

        let sort_fn = |n: &Node| (n.key() as u32);
        let reverse_sort_fn = |n: &Node| -(n.key() as i32);
        
        let sorting = topological_sort::<Node, Edge, u32, _>(
            &graph,
            Some(sort_fn)
        );
        assert!(sorting.is_ok());

        let sorting = topological_sort::<Node, Edge, i32, _>(
            &graph,
            Some(reverse_sort_fn)
        );
        assert!(sorting.is_ok());
    }
}
