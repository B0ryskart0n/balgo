#[cfg(test)]
mod tests;

mod candidate_node;

use candidate_node::CandidateNode;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

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
// TODO Verify the above in a benchmark

pub fn a_star<Id, Cost>(
    graph: &HashMap<Id, Vec<(Id, Cost)>>,
    start: Id,
    goal: Id,
) -> Option<(Vec<Id>, Cost)>
where
    Id: Eq + Hash + Copy,
    Cost: Ord + Copy + Clone + Add<Output = Cost> + Default,
{
    // Constant heurisitc.
    let distance = Cost::default();

    let start_node = CandidateNode {
        this: start,
        cost: Cost::default(),
        distance: distance,
    };

    let mut candidate_nodes = BinaryHeap::new();
    candidate_nodes.push(start_node);
    // Smallest (known) cost path to node and the preceding node.
    let mut known_nodes: HashMap<Id, (Cost, Option<Id>)> = HashMap::new();
    known_nodes.insert(start_node.this, (start_node.cost, None));

    while let Some(current) = candidate_nodes.pop() {
        if current.this == goal {
            return Some((construct_path(&known_nodes, current.this), current.cost));
        }

        graph.get(&current.this).unwrap_or(&vec![]).iter().for_each(
            |(candidate_id, edge_weight)| {
                let candidate = CandidateNode {
                    this: *candidate_id,
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
fn construct_path<Id, Cost>(
    known_nodes: &HashMap<Id, (Cost, Option<Id>)>,
    final_node_id: Id,
) -> Vec<Id>
where
    Id: Eq + Copy + Hash,
{
    let mut path = Vec::from([final_node_id]);

    let mut current_node = final_node_id;
    while let (_, Some(previous_node)) = known_nodes
        .get(&current_node)
        .expect("Internal implementation error: known_nodes should contain previous node.")
    {
        path.push(*previous_node);
        current_node = *previous_node;
    }
    path.reverse();
    return path;
}
