use std::hash::Hash;

use crate::untraced::state::UntracedState;

pub struct UntracedResult<S, K>
where
    K: Clone + Eq + Hash,
    S: UntracedState<K>
{
    pub iterations: usize,
    pub final_state: S,
    _marker: std::marker::PhantomData<K>
}

impl<S, K> UntracedResult<S, K>
where
    K: Clone + Eq + Hash,
    S: UntracedState<K>
{
    pub fn new(iterations: usize, final_state: S) -> Self {
        Self {
            iterations,
            final_state,
            _marker: std::marker::PhantomData,
        }
    }
}
