#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

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

    let mut candidate_nodes = BinaryHeap::new();
    candidate_nodes.push(CandidateNode {
        id: start,
        cost: 0,
        distance: 0,
    });

    let mut nodes = HashMap::new();
    nodes.insert(start, (0, None));

    while let Some(CandidateNode { id, cost, .. }) = candidate_nodes.pop() {
        if id == goal {
            return Some(construct_path(&nodes, id));
        }

        graph
            .get(&id)
            .unwrap_or(&vec![])
            .iter()
            .for_each(|(candidate_id, edge_weight)| {
                let candidate = CandidateNode {
                    cost: cost + *edge_weight,
                    distance: distance,
                    id: *candidate_id,
                };

                match nodes.get(candidate_id) {
                    // TODO Could those two arms be merged in some way?
                    None => {
                        nodes.insert(candidate.id.clone(), (candidate.cost, Some(id)));
                        candidate_nodes.push(candidate);
                    }
                    Some((previous_cost, _)) if *previous_cost > candidate.cost => {
                        nodes.insert(candidate.id.clone(), (candidate.cost, Some(id)));
                        candidate_nodes.push(candidate);
                    }
                    // Means that there was already an entry with smaller weight
                    _ => (),
                }
            });
        graph.capacity();
    }

    return None;
}
fn construct_path<Id>(
    visited_nodes: &HashMap<Id, (u32, Option<Id>)>,
    final_node_id: Id,
) -> (Vec<Id>, u32)
where
    Id: Eq + Copy + Hash,
{
    let final_cost = visited_nodes.get(&final_node_id).unwrap().0;
    let mut path = Vec::from([final_node_id]);

    let mut current_node = final_node_id;

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
    id: Id,
    // Used solely for determining order when inserting to the BinaryHeap
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
