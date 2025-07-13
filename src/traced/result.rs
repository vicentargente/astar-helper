use std::hash::Hash;

use crate::traced::state::TracedState;

pub struct TracedResult<S, K, C>
where
    K: Clone + Eq + Hash,
    S: TracedState<K, C>
{
    pub path: Vec<C>,
    pub iterations: usize,
    pub final_state: S,
    _marker: std::marker::PhantomData<K>
}

impl<S, K, C> TracedResult<S, K, C>
where
    K: Clone + Eq + Hash,
    S: TracedState<K, C>
{
    pub fn new(path: Vec<C>, iterations: usize, final_state: S) -> Self {
        Self {
            path,
            iterations,
            final_state,
            _marker: std::marker::PhantomData,
        }
    }
}
