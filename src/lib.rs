#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

// One cannot easily remove/replace specific node in the BinaryHeap, replacing (C, 3, A) with (C, 2, B)
// so the (C, 3, A) will remain for a potential pop in the future, but this pop does not make sense, since we know
// this is a suboptimal path.
// This begs the question whether a BinaryHeap is really the best solution. It might be better to keep a HashMap
// of known nodes and their cost + path, so when a (C, 2, B) is found it will replace the (C, 3, A) subsolution.
//
// Since a crucial part of the algorithm is selecting the next node based on it's cost BinaryHeap should still be used,
// because it enables that in O(log(n)) time. While keeping track of only the best path would require a sort of linear
// data structure that would mean O(n) time looking for least cost.

// One could consider keeping track of `visited_nodes` instead of `known_nodes`, which would mean smaller HashMap,
// but I'm pretty sure that having more information in `known_nodes` is better because it enables us to
// make better (more informed) decisions in adding (or not adding) candidate neighbours to the candidate queue.

pub fn a_star<Id>(
    // TODO Support different formats
    graph: &HashMap<Id, Vec<(Id, u32)>>,
    start: Id,
    goal: Id,
) -> Option<(Vec<Id>, u32)>
where
    Id: Eq + Hash + Copy,
{
    // Constant heurisitc.
    let distance = 0;

    let start_node = CandidateNode {
        this: start,
        prev: None,
        cost: 0,
        distance: distance,
    };

    let mut candidate_nodes = BinaryHeap::new();
    candidate_nodes.push(start_node);
    // Smallest (known) cost path to node and the preceding node.
    let mut known_nodes: HashMap<Id, (u32, Option<Id>)> = HashMap::new();
    known_nodes.insert(start_node.this, (start_node.cost, None));

    while let Some(current) = candidate_nodes.pop() {
        // TODO Verify if such logic is beneficial
        // if let Some((previous_cost, _)) = known_nodes.get(&current.this) {
        //     if *previous_cost < current.cost {
        //         // We know of a node with a lighter path, no need to check this one
        //         continue;
        //     }
        // }

        if current.this == goal {
            return Some(construct_path(&known_nodes, current.this));
        }

        graph.get(&current.this).unwrap_or(&vec![]).iter().for_each(
            |(candidate_id, edge_weight)| {
                let candidate = CandidateNode {
                    this: *candidate_id,
                    prev: Some(current.this),
                    cost: current.cost + *edge_weight,
                    distance: distance,
                };
                match known_nodes.get(&candidate.this) {
                    // We know a better (or equivalent) path to this candidate neighbour.
                    // Testing with `<=` because it should prevent more allocations and
                    // if the path are really equal then it does not matter.
                    Some((known_cost, _)) if *known_cost <= candidate.cost => (),
                    // Otherwise add neighbour as a candidate for future graph exploration.
                    _ => {
                        candidate_nodes.push(candidate);
                        known_nodes.insert(candidate.this, (candidate.cost, Some(current.this)));
                    }
                }
            },
        );
    }

    return None;
}
// TODO Only return path
fn construct_path<Id>(
    visited_nodes: &HashMap<Id, (u32, Option<Id>)>,
    final_node_id: Id,
) -> (Vec<Id>, u32)
where
    Id: Eq + Copy + Hash,
{
    // TODO Handle uwrap
    let final_cost = visited_nodes.get(&final_node_id).unwrap().0;
    let mut path = Vec::from([final_node_id]);

    let mut current_node = final_node_id;

    // TODO Handle unwrap
    while let (_, Some(previous_node)) = visited_nodes.get(&current_node).unwrap() {
        path.push(*previous_node);
        current_node = *previous_node;
    }
    path.reverse();
    return (path, final_cost);
}

#[derive(Copy, Clone)]
struct CandidateNode<Id>
where
    Id: Eq + Copy + Hash,
{
    this: Id,
    prev: Option<Id>,
    cost: u32,
    distance: u32,
}
impl<Id> CandidateNode<Id>
where
    Id: Eq + Copy + Hash,
{
    fn score(&self) -> u32 {
        self.cost + self.distance
    }
}
impl<Id> PartialEq for CandidateNode<Id>
where
    Id: Eq + Copy + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.score().eq(&other.score())
    }
}
impl<Id> Eq for CandidateNode<Id> where Id: Eq + Copy + Hash {}
impl<Id> PartialOrd for CandidateNode<Id>
where
    Id: Eq + Copy + Hash,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<Id> Ord for CandidateNode<Id>
where
    Id: Eq + Copy + Hash,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.score()
            .cmp(&other.score())
            // Flipped ordering because BinaryHeap is a max-heap and we want min-heap
            .reverse()
    }
}
