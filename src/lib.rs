#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub fn a_star<Id>(
    graph: &HashMap<Id, Vec<(Id, u32)>>,
    start: Id,
    goal: Id,
) -> Option<(Vec<Id>, u32)>
where
    Id: Debug + Eq + Clone + Hash + Copy,
{
    // Constant heurisitc.
    let distance = 0;

    let mut candidate_nodes = BinaryHeap::new();
    let mut nodes = HashMap::from([(start.clone(), (0, None))]);

    // Push start node
    candidate_nodes.push(Node {
        id: start.clone(),
        score: distance,
        prev: None,
    });
    while !candidate_nodes.is_empty() {
        // By construction we are sure that there is a candidate to unwrap.
        let current_node = candidate_nodes.pop().unwrap();
        println!("current_node: {:?}", current_node);
        if current_node.id == goal {
            return Some(construct_path(&nodes, &current_node));
        }

        // Assuming that every node has an entry, even those without neighbours.
        let neighbours = graph.get(&current_node.id).unwrap();
        for (candidate_id, edge_weight) in neighbours.iter() {
            let cost = nodes.get(&current_node.id).unwrap().0 + edge_weight;
            let candidate = Node {
                prev: Some(current_node.id),
                score: cost + distance,
                id: *candidate_id,
            };

            match nodes.get(candidate_id) {
                None => {
                    nodes.insert(candidate_id.clone(), (cost, Some(current_node.id)));
                    candidate_nodes.push(candidate);
                }
                Some((previous_cost, _)) if *previous_cost > cost => {
                    nodes.insert(candidate_id.clone(), (cost, Some(current_node.id)));
                    candidate_nodes.push(candidate);
                }
                // Means that there was already an entry with smaller weight
                _ => (),
            }
        }
        println!("candidate_nodes: {:?}", candidate_nodes);
    }

    return None;
}
fn construct_path<Id>(
    visited_nodes: &HashMap<Id, (u32, Option<Id>)>,
    node: &Node<Id>,
) -> (Vec<Id>, u32)
where
    Id: Eq + Clone + Hash,
{
    let final_cost = visited_nodes.get(&node.id).unwrap().0;
    let mut path_backwards = Vec::from([node.id.clone()]);

    let mut current_node = node.id.clone();

    while let (_, Some(previous_node)) = visited_nodes.get(&current_node).unwrap() {
        path_backwards.push(previous_node.clone());
        current_node = previous_node.clone();
    }
    path_backwards.reverse();
    return (path_backwards, final_cost);
}

#[derive(Clone, Debug)]
struct Node<Id>
where
    Id: Eq + Clone + Hash,
{
    id: Id,
    score: u32,
    prev: Option<Id>,
}
// impl<Id> for Node<Id>
// where
//     Id: std::fmt::Display,
//     Id: Eq,
//     Id: Clone,
//     Id: Hash
//     {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         match self.prev.clone() {
//             None => write!(f, "{} {} -", self.id, self.score),
//             Some(previous) => write!(f, "{} {} {}", self.id, self.score, previous),
//         }
//     }
// }
impl<Id> PartialEq for Node<Id>
where
    Id: Debug + Eq + Clone + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}
impl<Id> Eq for Node<Id> where Id: Debug + Eq + Clone + Hash {}
impl<Id> PartialOrd for Node<Id>
where
    Id: Debug + Eq + Clone + Hash,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<Id> Ord for Node<Id>
where
    Id: Debug + Eq + Clone + Hash,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            // Flipped ordering because BinaryHeap is a max-heap and we want min-heap
            .reverse()
    }
}
