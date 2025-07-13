use std::hash::Hash;

use crate::astar_state::AStarState;

pub trait TracedState<K, C>: AStarState<K>
where
    K: Clone + Eq + Hash,
{
    fn generate_traced_successors(&self) -> Vec<(Self, C)>;
}


pub(super) struct TracedStateWrapper<T, K, C>
where
    T: TracedState<K, C>,
    K: Clone + Eq + Hash
{
    pub state: T,
    pub prev_key: Option<K>,
    pub change: Option<C>
}

impl<T, K, C> TracedStateWrapper<T, K, C>
where
    T: TracedState<K, C>,
    K: Clone + Eq + Hash
{
    pub fn new(state: T) -> Self {
        TracedStateWrapper {
            state,
            prev_key: None,
            change: None
        }
    }

    pub fn generate_states(&self) -> Vec<Self> {
        TracedState::generate_traced_successors(&self.state)
            .into_iter()
            .map(|(successor, change)| {
                TracedStateWrapper {
                    state: successor,
                    prev_key: Some(self.key()),
                    change: Some(change)
                }
            })
            .collect()
    }
}

impl<T, K, C> AStarState<K> for TracedStateWrapper<T, K, C>
where
    T: TracedState<K, C>,
    K: Clone + Eq + Hash
{
    fn key(&self) -> K {
        self.state.key()
    }

    fn h(&self) -> usize {
        self.state.h()
    }

    fn f(&self) -> usize {
        self.state.f()
    }

    fn g(&self) -> usize {
        self.state.g()
    }

    fn is_goal(&self) -> bool {
        self.state.is_goal()
    }
}
