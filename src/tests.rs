use super::*;

#[test]
fn test1() {
    // --- Graph Definition ---
    let mut graph = HashMap::new();
    graph.insert('A', vec![('B', 1), ('C', 3)]);
    graph.insert('B', vec![('D', 5)]);
    graph.insert('C', vec![('D', 2)]);
    graph.insert('D', vec![('G', 1)]);
    graph.insert('G', vec![]);

    let start_node = 'A';
    let goal_node = 'G';

    let result = a_star(&graph, start_node, goal_node);

    let expected_cost = 6;
    let expected_path = vec!['A', 'C', 'D', 'G'];

    assert_eq!(result.0, expected_path, "The calculated path is incorrect.");
    assert_eq!(result.1, expected_cost, "The calculated cost is incorrect.");
}

#[test]
fn test2() {
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

    let result = a_star(&graph, start_node, goal_node);

    let expected_cost = 14;
    let expected_path = vec!['A', 'C', 'F', 'I', 'J'];

    assert_eq!(result.0, expected_path, "The calculated path is incorrect.");
    assert_eq!(result.1, expected_cost, "The calculated cost is incorrect.");
}
