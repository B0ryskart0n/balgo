use std::cmp::Ordering;
use std::hash::Hash;

#[derive(Copy, Clone)]
pub struct CandidateNode<Id>
where
    Id: Eq + Copy + Hash,
{
    pub this: Id,
    pub cost: u32,
    pub distance: u32,
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
