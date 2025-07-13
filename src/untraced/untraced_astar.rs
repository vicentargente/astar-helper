use std::{collections::HashSet, hash::Hash};

use crate::{open_list::OpenList, untraced::{result::UntracedResult, state::UntracedState}};

pub fn untraced_astar<S, K>(initial_state: S) -> Option<UntracedResult<S, K>>
where
    S: UntracedState<K>,
    K: Clone + Eq + Hash
{
    let mut open_list = OpenList::new();
    let mut closed_list = HashSet::new();

    open_list.insert(initial_state.key(), initial_state);

    while let Some(current_state) = open_list.extract_min() {
        if current_state.is_goal() {
            let final_state = current_state;
            let iterations = closed_list.len();

            return Some(
                UntracedResult::new(
                    iterations,
                    final_state
                )
            );
        }

        closed_list.insert(current_state.key());

        let successors = current_state.generate_successors();
        for successor in successors {
            let successor_key = successor.key();

            if closed_list.contains(&successor_key) {
                continue;
            }

            open_list.insert(successor_key, successor);
        }
    }

    None
}
