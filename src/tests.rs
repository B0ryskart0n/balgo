use super::*;

#[test]
fn none() {
    let graph = HashMap::from([('A', vec![])]);

    let start_node = 'A';
    let goal_node = 'B';

    let result = a_star(&graph, start_node, goal_node);

    let expected = None;

    assert_eq!(result, expected);
}
#[test]
fn trivial() {
    let graph = HashMap::from([('A', vec![])]);

    let start_node = 'A';
    let goal_node = 'A';

    let expected = Some((vec!['A'], 0));
    let result = a_star(&graph, start_node, goal_node);

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

    let result = a_star(&graph, start_node, goal_node);

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
    let actual = a_star(&graph, start_node, goal_node);

    assert_eq!(actual, expected);
}
