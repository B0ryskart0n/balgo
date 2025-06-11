#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn a_star(
    graph: &HashMap<Id, Vec<(Id, Weight)>>,
    start: Id,
    goal: Id,
) -> Option<(Vec<Id>, Weight)> {
    let mut discovered_nodes = BinaryHeap::<Node>::new();
    let mut previous_nodes = HashMap::<Id, Option<Id>>::new();
    let start_node = Node {
        prev: None,
        this: start,
        weight: 0,
    };
    discovered_nodes.push(start_node);

    while !discovered_nodes.is_empty() {
        let node = discovered_nodes.pop().unwrap();
        println!("Current node: {:?}", node);
        previous_nodes.insert(node.this.clone(), node.prev);

        if node.this == goal {
            return Some((construct_path(&previous_nodes, &node), node.weight));
        }

        for (next_node, edge_weight) in graph.get(&node.this).unwrap().iter() {
            match graph.get(&next_node) {
                Some(Node {
                    prev: _,
                    this: _,
                    weight: w,
                }) => (),
                None => (),
            }
            discovered_nodes.push(Node {
                prev: Some(node.this.clone()),
                this: next_node.clone(),
                weight: node.weight + edge_weight + 0,
            });
        }
    }

    return None;
}
fn construct_path(previous_nodes: &HashMap<Id, Option<Id>>, node: &Node) -> Vec<Id> {
    println!("Previous_nodes: {:?}", previous_nodes);

    let mut path_backwards = Vec::from([node.this.clone()]);

    let mut current_node = node.this.clone();

    while let Some(previous_node) = previous_nodes.get(&current_node).unwrap() {
        path_backwards.push(previous_node.clone());
        current_node = *previous_node;
    }
    path_backwards.reverse();
    return path_backwards;
}

type Id = char;
type Weight = u32;

#[derive(Clone, Debug)]
struct Node {
    prev: Option<Id>,
    this: Id,
    weight: Weight,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight
            .cmp(&other.weight)
            // Flipped ordering because BinaryHeap is a max-heap and we want min-heap
            .reverse()
    }
}
