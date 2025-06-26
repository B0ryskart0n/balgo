use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct CandidateNode<Id, Cost>
where
    Id: Eq + Copy + Hash,
    Cost: Ord + Add<Output = Cost> + Default,
{
    pub this: Id,
    pub cost: Cost,
    pub distance: Cost,
}
impl<Id, Cost> CandidateNode<Id, Cost>
where
    Id: Eq + Copy + Hash,
    Cost: Ord + Copy + Clone + Add<Output = Cost> + Default + Add<Output = Cost> + Default,
{
    fn score(&self) -> Cost {
        self.cost + self.distance
    }
}
impl<Id, Cost> PartialEq for CandidateNode<Id, Cost>
where
    Id: Eq + Copy + Hash,
    Cost: Ord + Copy + Clone + Add<Output = Cost> + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.score().eq(&other.score())
    }
}
impl<Id, Cost> Eq for CandidateNode<Id, Cost>
where
    Id: Eq + Copy + Hash,
    Cost: Ord + Copy + Clone + Add<Output = Cost> + Default,
{
}
impl<Id, Cost> PartialOrd for CandidateNode<Id, Cost>
where
    Id: Eq + Copy + Hash,
    Cost: Ord + Copy + Clone + Add<Output = Cost> + Default,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<Id, Cost> Ord for CandidateNode<Id, Cost>
where
    Id: Eq + Copy + Hash,
    Cost: Ord + Copy + Clone + Add<Output = Cost> + Default,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.score()
            .cmp(&other.score())
            // Flipped ordering because BinaryHeap is a max-heap and we want min-heap
            .reverse()
    }
}
