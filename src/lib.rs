#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn a_star(
    graph: &HashMap<Id, Vec<(Id, Weight)>>,
    start: Id,
    goal: Id,
) -> Option<(Vec<Id>, Weight)> {
    let mut discovered_nodes = BinaryHeap::<Node>::new();
    let mut visited_nodes = HashMap::<Id, Option<(Id, Weight)>>::new();
    // Push start node
    discovered_nodes.push(Node {
        prev: None,
        this: start,
        weight: 0,
    });

    while !discovered_nodes.is_empty() {
        let current = discovered_nodes.pop().unwrap();
        println!("Current:\t{:?}", current);
        match visited_nodes.entry(current.this) {
            Entry::Vacant(entry) => {
                entry.insert(current.prev.map(|prev| (prev, current.weight)));
            }
            Entry::Occupied(mut entry) if entry.get().unwrap().1 > current.weight => {
                entry.insert(current.prev.map(|prev| (prev, current.weight)));
            }
            _ => (),
        };

        if current.this == goal {
            return Some((construct_path(&visited_nodes, &current), current.weight));
        }

        let neighbours: Vec<Node> = graph
            .get(&current.this)
            .unwrap()
            .iter()
            .map(|(next_id, edge_weight)| Node {
                prev: Some(current.this.clone()),
                weight: current.weight + edge_weight + 0,
                this: *next_id,
            })
            .collect();
        println!("New:\t\t{:?}", neighbours);

        neighbours
            .iter()
            .for_each(|node| discovered_nodes.push(node.clone()));

        println!("Queue:\t\t{:?}\n", discovered_nodes);
    }

    return None;
}
fn construct_path(visited_nodes: &HashMap<Id, Option<(Id, Weight)>>, node: &Node) -> Vec<Id> {
    let mut path_backwards = Vec::from([node.this.clone()]);

    let mut current_node = node.this.clone();

    while let Some((previous_node, _)) = visited_nodes.get(&current_node).unwrap() {
        path_backwards.push(previous_node.clone());
        current_node = *previous_node;
    }
    path_backwards.reverse();
    return path_backwards;
}

type Id = char;
type Weight = u32;

#[derive(Clone)]
struct Node {
    prev: Option<Id>,
    weight: Weight,
    this: Id,
}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.prev {
            None => write!(f, "? --[{}]--> {}", self.weight, self.this),
            Some(prev) => write!(f, "{} --[{}]--> {}", prev, self.weight, self.this),
        }
    }
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
