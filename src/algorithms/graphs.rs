use std::collections::{HashMap, HashSet};

/// Returns the number of connected components in an undirected graph.
pub fn connected_components(G: &HashMap<i32, Vec<i32>>) -> usize {
    let mut seen = HashSet::new();
    let mut count = 0;

    // Recursive helper: mark everything reachable from `node`.
    fn explore(
        node: i32,
        G: &HashMap<i32, Vec<i32>>,
        seen: &mut HashSet<i32>,
    ) {
        // If we just insert, we know `node` was unvisited.
        if seen.insert(node) {
            if let Some(neighbors) = G.get(&node) {
                for &neighbor in neighbors {
                    // Recurse only on unvisited neighbors
                    if !seen.contains(&neighbor) {
                        explore(neighbor, G, seen);
                    }
                }
            }
        }
    }

    // For each node, if still unvisited, it's a new component.
    for &node in G.keys() {
        if !seen.contains(&node) {
            count += 1;
            explore(node, G, &mut seen);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_empty_graph() {
        let g: HashMap<i32, Vec<i32>> = HashMap::new();
        assert_eq!(connected_components(&g), 0);
    }

    #[test]
    fn test_single_node() {
        let mut g = HashMap::new();
        g.insert(1, Vec::new());
        assert_eq!(connected_components(&g), 1);
    }

    #[test]
    fn test_two_disconnected_nodes() {
        let mut g = HashMap::new();
        g.insert(1, Vec::new());
        g.insert(2, Vec::new());
        assert_eq!(connected_components(&g), 2);
    }

    #[test]
    fn test_connected_graph() {
        let mut g = HashMap::new();
        g.insert(1, vec![2]);
        g.insert(2, vec![1, 3]);
        g.insert(3, vec![2]);
        assert_eq!(connected_components(&g), 1);
    }

    #[test]
    fn test_mixed_graph() {
        let mut g = HashMap::new();
        // component 1: 1-2-3
        g.insert(1, vec![2]);
        g.insert(2, vec![1, 3]);
        g.insert(3, vec![2]);
        // isolated nodes: 4, 5
        g.insert(4, Vec::new());
        g.insert(5, Vec::new());
        // component 2: 6-7
        g.insert(6, vec![7]);
        g.insert(7, vec![6]);

        assert_eq!(connected_components(&g), 4);
    }
}