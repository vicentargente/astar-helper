use std::hash::Hash;

pub trait AStarState<K>
where
    K: Clone + Eq + Hash,
    Self: Sized
{
    fn key(&self) -> K;
    fn h(&self) -> usize;
    fn f(&self) -> usize;
    fn g(&self) -> usize;
    fn is_goal(&self) -> bool;
}
