use std::{collections::HashMap, hash::Hash};

use crate::{astar_state::AStarState, open_list::OpenList, traced::{result::TracedResult, state::{TracedState, TracedStateWrapper}}};

pub fn traced_astar<S, K, C>(initial_state: S) -> Option<TracedResult<S, K, C>>
where
    S: TracedState<K, C>,
    K: Clone + Eq + Hash
{
    let mut open_list: OpenList<K, TracedStateWrapper<S, K, C>> = OpenList::new();
    let mut closed_list: HashMap<K, TracedStateWrapper<S, K, C>> = HashMap::new();

    open_list.insert(initial_state.key(), TracedStateWrapper::new(initial_state));

    while let Some(current_state) = open_list.extract_min() {
        if current_state.is_goal() {
            let TracedStateWrapper { state, prev_key, change } = current_state;
            
            let final_state= state;
            let iterations = closed_list.len();
            let mut path = Vec::new();

            if let Some(change) = change {
                path.push(change);
            }

            if let Some(prev_key) = prev_key {
                let mut curr_key = prev_key;

                while let Some(prev_state) = closed_list.remove(&curr_key) {
                    if let Some(change) = prev_state.change {
                        path.push(change);
                    }

                    if let Some(prev_key) = prev_state.prev_key {
                        curr_key = prev_key;
                    }
                    else {
                        break;
                    }
                }
            }

            path.reverse();

            return Some(
                TracedResult::new(
                    path,
                    iterations,
                    final_state
                )
            );
        }

        let successors = current_state.generate_states();

        closed_list.insert(current_state.key(), current_state);

        for successor in successors {
            let successor_key = successor.key();

            if closed_list.contains_key(&successor_key) {
                continue;
            }

            open_list.insert(successor_key, successor);
        }
    }

    None
}
