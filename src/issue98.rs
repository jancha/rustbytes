// Test cases: Run `cargo test` to verify. Covers basics, edges, combinations.
#[cfg(test)]
mod tests {
    use super::*;
    // Rust Bytes Challenge Issue #98 Graph Gossip️
    use std::collections::{HashMap, HashSet, VecDeque};
    type Graph = HashMap<String, Vec<String>>;
    pub fn walk_graph<'a>(graph: &'a Graph, start: &'a String, path: &VecDeque<String>, cycles: &mut Vec<Vec<String>>) {
        let mut path = path.clone();
        path.push_back(start.to_string());

        if let Some(nodes) = graph.get(start) {
            for node in nodes {
                if !path.contains(node) {
                    walk_graph(graph, node, &path, cycles);
                } else {
                    let mut found = false;
                    let mut cycle:Vec<String> = Vec::new();
                    for i in &path {
                        if i == node {
                            found = true;
                            // start
                        }
                        if found {
                            cycle.push(i.to_string());
                        }
                    }
                    cycles.push(cycle);
                }
            }
        }
    }

    pub fn topological_sort(graph: &Graph) -> Result<Vec<String>, Vec<String>> {
        let mut queue: VecDeque<&String> = VecDeque::with_capacity(graph.len());
        let mut paths: HashMap<&String, usize> = HashMap::new();
        for (person, nodes) in graph {
            if !paths.contains_key(person) {
                paths.insert(person, 0);
            }
            for node in nodes {
                paths.entry(node).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        let mut top:Vec<(&String, usize)> = paths.into_iter().collect::<Vec<(&String, usize)>>();
        top.sort_unstable_by_key(|(_node, paths)| *paths);
        let mut failed: Vec<String> = Vec::new();
        let mut order: Vec<String> = Vec::new();
        let mut seen: HashSet<&String> = HashSet::new();
        // try sort each "top node"
        for (person, _paths) in top {
            if seen.contains(person) {
                continue;
            }
            let mut cycles = Vec::new();

            walk_graph(graph, person, &VecDeque::new(), &mut cycles);

            if !cycles.is_empty() {
                //flattern
                for i in cycles {
                    for j in i {
                        if !failed.contains(&j) {
                            failed.push(j);
                        }
                    }
                }
            } else {
                queue.push_front(person);
                while let Some(person) = queue.pop_front() {
                    order.push(person.to_string());
                    seen.insert(person);
                    if let Some(nodes) = graph.get(person) && !nodes.is_empty() {
                        let mut found_cycle = true;
                        for node in nodes {
                            if !seen.contains(node) {
                                if !queue.contains(&node) {
                                    queue.push_back(node);
                                }
                                found_cycle = false;
                            }
                        }
                        if found_cycle {
                            panic!("SHould have found earlier");
                        }
                    }
                }
            }
        }
        if failed.is_empty() {
            Ok(order)
        } else {
            Err(failed)
        }
    }

    // Helper to validate topo order (checks all edges A->B have pos(A) < pos(B))
    fn is_valid_topo_order(order: &[String], graph: &HashMap<String, Vec<String>>) -> bool {
        let pos: HashMap<&String, usize> = order.iter().enumerate().map(|(i, n)| (n, i)).collect();
        for (from, neighbors) in graph {
            if let Some(&p_from) = pos.get(from) {
                for to in neighbors {
                    if let Some(&p_to) = pos.get(to) {
                        if p_from >= p_to {
                            return false;
                        }
                    } else {
                        return false;
                    } // Missing node? Invalid
                }
            }
        }
        true
    }

    #[test]
    fn test_empty_graph() {
        let graph: HashMap<String, Vec<String>> = HashMap::new();
        assert_eq!(topological_sort(&graph), Ok(vec![]));
    }

    #[test]
    fn test_single_node_no_edges() {
        let mut graph = HashMap::new();
        graph.insert("Alice".to_string(), vec![]);
        let result = topological_sort(&graph).unwrap();
        assert_eq!(result, vec!["Alice"]);
    }

    #[test]
    fn test_single_node_self_loop() {
        let mut graph = HashMap::new();
        graph.insert("Bob".to_string(), vec!["Bob".to_string()]);
        let err = topological_sort(&graph).unwrap_err();
        assert_eq!(err, vec!["Bob"]);
    }

    #[test]
    fn test_simple_dag() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["D".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec![]);

        let result = topological_sort(&graph).unwrap();
        assert_eq!(result.len(), 4);
        assert!(is_valid_topo_order(&result, &graph));
        // Possible: ["A", "B", "C", "D"] or ["A", "C", "B", "D"]
    }

    #[test]
    fn test_disconnected_components() {
        let mut graph = HashMap::new();
        graph.insert("X".to_string(), vec!["Y".to_string()]);
        graph.insert("Y".to_string(), vec![]);
        graph.insert("P".to_string(), vec!["Q".to_string()]);
        graph.insert("Q".to_string(), vec![]);

        let result = topological_sort(&graph).unwrap();
        assert_eq!(result.len(), 4);
        assert!(is_valid_topo_order(&result, &graph));
        // Possible: ["X", "P", "Y", "Q"] etc., as long as X<Y, P<Q
    }

    #[test]
    fn test_simple_cycle() {
        let mut graph = HashMap::new();
        graph.insert("E".to_string(), vec!["F".to_string()]);
        graph.insert("F".to_string(), vec!["G".to_string()]);
        graph.insert("G".to_string(), vec!["E".to_string()]);

        let err = topological_sort(&graph).unwrap_err();
        assert_eq!(err.len(), 3);
        assert!(err.contains(&"E".to_string()));
        assert!(err.contains(&"F".to_string()));
        assert!(err.contains(&"G".to_string()));
        // Order may vary (e.g., ["E", "F", "G"] or reverse)
    }

    #[test]
    fn test_multiple_cycles() {
        let mut graph = HashMap::new();
        graph.insert("C1".to_string(), vec!["C2".to_string()]);
        graph.insert("C2".to_string(), vec!["C1".to_string()]);
        graph.insert("D1".to_string(), vec!["D2".to_string()]);
        graph.insert("D2".to_string(), vec!["D1".to_string()]);

        let err = topological_sort(&graph).unwrap_err();
        assert_eq!(err.len(), 2); // Detects one cycle; impl choice
                                  // Could be C1-C2 or D1-D2
    }

    #[test]
    fn test_cycle_with_dag() {
        // Mixed: DAG part + cycle
        let mut graph = HashMap::new();
        graph.insert("Start".to_string(), vec!["CycleA".to_string()]);
        graph.insert("CycleA".to_string(), vec!["CycleB".to_string()]);
        graph.insert("CycleB".to_string(), vec!["CycleA".to_string()]);
        graph.insert("End".to_string(), vec![]); // Disconnected

        let err = topological_sort(&graph).unwrap_err();
        assert_eq!(err.len(), 2);
        assert!(err.contains(&"CycleA".to_string()));
        assert!(err.contains(&"CycleB".to_string()));
    }

    #[test]
    fn test_complex_dag_multiple_paths() {
        let mut graph = HashMap::new();
        graph.insert("T1".to_string(), vec!["T2".to_string(), "T3".to_string()]);
        graph.insert("T2".to_string(), vec!["T4".to_string()]);
        graph.insert("T3".to_string(), vec!["T4".to_string(), "T5".to_string()]);
        graph.insert("T4".to_string(), vec!["T6".to_string()]);
        graph.insert("T5".to_string(), vec!["T6".to_string()]);
        graph.insert("T6".to_string(), vec![]);

        let result = topological_sort(&graph).unwrap();
        assert_eq!(result.len(), 6);
        assert!(is_valid_topo_order(&result, &graph));
        // Ensures T1 < T2/T3, T2/T3 < T4/T5, T4/T5 < T6
    }

    #[test]
    fn test_no_duplicate_edges() {
        // Graph with potential dups (but spec says no dups, test impl handles)
        let mut graph = HashMap::new();
        graph.insert(
            "DupA".to_string(),
            vec!["DupB".to_string(), "DupB".to_string()],
        ); // Dup edge
        graph.insert("DupB".to_string(), vec![]);

        let result = topological_sort(&graph).unwrap();
        assert_eq!(result.len(), 2);
        assert!(is_valid_topo_order(&result, &graph)); // Treat as single edge
    }

    #[test]
    fn test_long_chain() {
        // Deep recursion test (chain of 10 nodes)
        let mut graph = HashMap::new();
        let nodes = (0..10).map(|i| format!("N{}", i)).collect::<Vec<_>>();
        for i in 0..9 {
            graph.insert(nodes[i].clone(), vec![nodes[i + 1].clone()]);
        }
        graph.insert(nodes[9].clone(), vec![]);

        let result = topological_sort(&graph).unwrap();
        assert_eq!(result.len(), 10);
        for i in 0..9 {
            let pos_i = result.iter().position(|n| n == &nodes[i]).unwrap();
            let pos_next = result.iter().position(|n| n == &nodes[i + 1]).unwrap();
            assert!(pos_i < pos_next);
        }
    }

    #[test]
    fn test_isolated_nodes_multiple() {
        let mut graph = HashMap::new();
        graph.insert("Iso1".to_string(), vec![]);
        graph.insert("Iso2".to_string(), vec![]);
        graph.insert("Iso3".to_string(), vec![]);

        let result = topological_sort(&graph).unwrap();
        assert_eq!(result.len(), 3);
        // Any order valid since no edges
    }
}
