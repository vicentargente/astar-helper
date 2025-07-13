use std::hash::Hash;

use crate::astar_state::AStarState;

pub trait UntracedState<K>: AStarState<K>
where
    K: Clone + Eq + Hash,
{
    fn generate_successors(&self) -> Vec<Self>;
}
