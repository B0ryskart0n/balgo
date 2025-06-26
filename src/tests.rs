use super::*;

#[test]
fn none() {
    let graph = HashMap::from([('A', vec![])]);

    let start_node = 'A';
    let goal_node = 'B';

    let result = a_star::<char, u32>(&graph, start_node, goal_node);

    let expected = None;

    assert_eq!(result, expected);
}
#[test]
fn trivial() {
    let graph = HashMap::from([('A', vec![])]);

    let start_node = 'A';
    let goal_node = 'A';

    let expected = Some((vec!['A'], 0));
    let result = a_star::<char, u32>(&graph, start_node, goal_node);

    assert_eq!(result, expected);
}

#[test]
fn simple() {
    let mut graph = HashMap::new();
    graph.insert('A', vec![('B', 1), ('C', 3)]);
    graph.insert('B', vec![('D', 3), ('C', 1)]);
    graph.insert('C', vec![('D', 1)]);
    graph.insert('D', vec![]);

    let start_node = 'A';
    let goal_node = 'D';

    let result = a_star::<char, u32>(&graph, start_node, goal_node);

    let expected = Some((vec!['A', 'B', 'C', 'D'], 3));

    assert_eq!(result, expected);
}

#[test]
fn complex() {
    let mut graph = HashMap::new();
    graph.insert('A', vec![('B', 2), ('C', 5), ('D', 10)]);
    graph.insert('B', vec![('E', 4)]);
    graph.insert('C', vec![('E', 3), ('F', 8)]);
    graph.insert('D', vec![('G', 2)]);
    graph.insert('E', vec![('H', 6)]);
    graph.insert('F', vec![('I', 1)]);
    graph.insert('G', vec![('H', 9), ('J', 12)]);
    graph.insert('H', vec![('J', 3)]);
    graph.insert('I', vec![('J', 5)]);
    graph.insert('J', vec![]);

    let start_node = 'A';
    let goal_node = 'J';

    let expected = Some((vec!['A', 'B', 'E', 'H', 'J'], 15));
    let actual = a_star::<char, u32>(&graph, start_node, goal_node);

    assert_eq!(actual, expected);
}
use std::collections::HashMap;

#[test]
fn very_complex() {
    let mut graph: HashMap<i32, Vec<(i32, u32)>> = HashMap::new();
    let num_nodes = 150;
    let start_node = 0;
    let goal_node = num_nodes - 1; // 149

    // --- Graph Construction ---

    // 1. First, create the "Golden Path". This is the actual shortest path
    //    we want the algorithm to find. It has a very low cost per step.
    //    Path: 0 -> 15 -> 30 -> 45 -> ... -> 135 -> 149
    let mut golden_path_nodes = vec![start_node];
    let mut current_node = start_node;
    while current_node < 135 {
        graph
            .entry(current_node)
            .or_insert_with(Vec::new)
            .push((current_node + 15, 1));
        current_node += 15;
        golden_path_nodes.push(current_node);
    }
    // Final leg of the golden path
    graph
        .entry(135)
        .or_insert_with(Vec::new)
        .push((goal_node, 1));
    golden_path_nodes.push(goal_node);

    let expected_cost = (golden_path_nodes.len() - 1) as u32; // Each step costs 1

    // 2. Create a dense, highly-connected graph around the golden path.
    //    Each node connects to its next few neighbors with a medium cost.
    for i in 0..num_nodes {
        // Ensure every node has an entry in the graph
        graph.entry(i).or_insert_with(Vec::new);

        // Connect to next 3 nodes with moderate cost
        for j in 1..=3 {
            if i + j < num_nodes {
                let cost = (j * 4) as u32; // e.g., cost to i+1 is 4, i+2 is 8, etc.
                graph.entry(i).or_insert_with(Vec::new).push((i + j, cost));
            }
        }
    }

    // 3. Add "trap" paths. These seem like good ideas but are actually more expensive.

    // Trap 1: A very expensive "shortcut" from the start to a node near the end.
    graph
        .entry(start_node)
        .or_insert_with(Vec::new)
        .push((goal_node - 5, 100));

    // Trap 2: A cheap path that leads into an expensive area.
    // 0 -> 1 -> 2 is very cheap, but the path from 2 onwards will be costly.
    graph.entry(0).or_insert_with(Vec::new).push((1, 1));
    graph.entry(1).or_insert_with(Vec::new).push((2, 1));
    // Now make paths from node 2 expensive
    graph.entry(2).or_insert_with(Vec::new).push((20, 50));
    graph.entry(2).or_insert_with(Vec::new).push((30, 50));

    // 4. Add more random connections to increase density and complexity.
    //    This makes the search space much larger.
    for i in 0..num_nodes {
        // Connect to a node much further away
        let target1 = (i + 37) % num_nodes;
        // Connect to a node based on a different calculation
        let target2 = (i * 3) % num_nodes;

        if target1 != i {
            graph.entry(i).or_insert_with(Vec::new).push((target1, 25));
        }
        if target2 != i {
            graph.entry(i).or_insert_with(Vec::new).push((target2, 35));
        }
    }

    // Ensure the goal node has no outgoing edges (it's the end)
    graph.insert(goal_node, vec![]);

    let expected = Some((golden_path_nodes, expected_cost));
    let actual = a_star::<i32, u32>(&graph, start_node, goal_node);

    assert_eq!(actual, expected);
}
