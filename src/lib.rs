#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub fn a_star(graph: &HashMap<Id, Vec<(Id, Weight)>>, start: Id, goal: Id) -> (Vec<Id>, Weight) {
    let mut path: Vec<Id> = Vec::new();
    let mut weight = 0;

    return (path, weight);
}

type Id = char;
type Weight = u32;
