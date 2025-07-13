use std::{collections::HashMap, hash::Hash};

use crate::astar_state::AStarState;

pub struct OpenList<K, V>
where
    K: Clone + Eq + Hash,
    V: AStarState<K>
{
    heap: Vec<(K, V)>,
    map: HashMap<K, usize>
}

impl<K, V> OpenList<K, V>
where
    K: Clone + Eq + Hash,
    V: AStarState<K>
{
    pub fn new() -> Self {
        OpenList {
            heap: Vec::new(),
            map: HashMap::new()
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn insert(&mut self, key: K, value: V) {
        let value_f = value.f();

        if let Some(&index) = self.map.get(&key) {
            if value_f < self.heap[index].1.f() {
                self.heap[index] = (key.clone(), value);
                self.bubble_up(index);
            }
        }
        else {
            let index = self.heap.len();
            self.heap.push((key.clone(), value));
            self.map.insert(key, index);
            self.bubble_up(index);
        }
    }

    #[allow(dead_code)]
    pub fn min(&self) -> Option<&V> {
        self.heap.first().map(|(_, value)| value)
    }

    pub fn extract_min(&mut self) -> Option<V> {
        if self.heap.is_empty() {
            return None;
        }

        let last_index = self.heap.len() - 1;
        self.swap(0, last_index);
        let min_value = self.pop();

        if !self.heap.is_empty() {
            self.buble_down(0);
        }

        min_value
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);

        let (key_i, _) = &self.heap[i];
        let (key_j, _) = &self.heap[j];

        self.map.insert(key_i.clone(), i);
        self.map.insert(key_j.clone(), j);
    }

    fn bubble_up(&mut self, index: usize) {
        let mut current = index;
        while current > 0 {
            let parent = (current - 1) / 2;

            let current_cost = self.heap[current].1.f();
            let parent_cost = self.heap[parent].1.f();

            if current_cost >= parent_cost {
                break;
            }

            self.swap(current, parent);
            current = parent;
        }
    }
    
    fn buble_down(&mut self, index: usize) {
        let mut current = index;
        let len = self.heap.len();

        loop {
            let left = 2 * current + 1;
            let right = 2 * current + 2;
            let mut smallest = current;

            if left < len && self.heap[left].1.f() < self.heap[smallest].1.f() {
                smallest = left;
            }
            if right < len && self.heap[right].1.f() < self.heap[smallest].1.f() {
                smallest = right;
            }

            if smallest == current {
                break;
            }

            self.swap(current, smallest);
            current = smallest;
        }
    }

    fn pop(&mut self) -> Option<V> {
        if self.heap.is_empty() {
            return None;
        }

        let last_index = self.heap.len() - 1;
        let (_, value) = self.heap.remove(last_index);
        self.map.remove(&value.key());
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A more flexible TestState struct for fine-grained control over costs.
    #[derive(Debug, PartialEq, Clone)]
    struct TestState {
        id: i32,
        g_cost: usize,
        h_cost: usize,
    }

    impl AStarState<i32> for TestState {
        fn key(&self) -> i32 {
            self.id
        }
    
        fn h(&self) -> usize {
            self.h_cost
        }
    
        fn f(&self) -> usize {
            self.g() + self.h()
        }
    
        fn g(&self) -> usize {
            self.g_cost
        }
    
        fn is_goal(&self) -> bool {
            self.h_cost == 0
        }
    }

    #[test]
    fn test_new_and_is_empty() {
        let open_list: OpenList<i32, TestState> = OpenList::new();
        assert!(open_list.is_empty());
        assert_eq!(open_list.heap.len(), 0);
        assert_eq!(open_list.map.len(), 0);
    }

    #[test]
    fn test_insert_and_extract_min_simple() {
        let mut open_list = OpenList::new();

        // States with different f-costs
        let state1 = TestState { id: 1, g_cost: 10, h_cost: 5 }; // f = 15
        let state2 = TestState { id: 2, g_cost: 5, h_cost: 5 };  // f = 10
        let state3 = TestState { id: 3, g_cost: 20, h_cost: 0 }; // f = 20

        open_list.insert(state1.key(), state1.clone());
        open_list.insert(state2.key(), state2.clone());
        open_list.insert(state3.key(), state3.clone());

        assert!(!open_list.is_empty());
        assert_eq!(open_list.heap.len(), 3);

        // The minimum element should be state2 (f=10)
        assert_eq!(open_list.min(), Some(&state2));
        assert_eq!(open_list.extract_min(), Some(state2));
        assert_eq!(open_list.heap.len(), 2);

        // Next minimum should be state1 (f=15)
        assert_eq!(open_list.min(), Some(&state1));
        assert_eq!(open_list.extract_min(), Some(state1));
        assert_eq!(open_list.heap.len(), 1);

        // Last element is state3 (f=20)
        assert_eq!(open_list.min(), Some(&state3));
        assert_eq!(open_list.extract_min(), Some(state3));
        
        assert!(open_list.is_empty());
    }

    #[test]
    fn test_extract_from_empty() {
        let mut open_list: OpenList<i32, TestState> = OpenList::new();
        assert_eq!(open_list.extract_min(), None);
    }

    #[test]
    fn test_min_on_empty() {
        let open_list: OpenList<i32, TestState> = OpenList::new();
        assert_eq!(open_list.min(), None);
    }

    #[test]
    fn test_update_existing_key_with_lower_cost() {
        let mut open_list = OpenList::new();
        let original_state = TestState { id: 1, g_cost: 10, h_cost: 10 }; // f = 20
        let updated_state = TestState { id: 1, g_cost: 5, h_cost: 5 };   // f = 10

        open_list.insert(original_state.key(), original_state);
        assert_eq!(open_list.min().unwrap().f(), 20);

        // Insert the same key, but with a better (lower) f-cost
        open_list.insert(updated_state.key(), updated_state.clone());
        
        // The list should still have only one element
        assert_eq!(open_list.heap.len(), 1);
        
        // The element should be the updated one with the lower f-cost
        assert_eq!(open_list.min(), Some(&updated_state));
        assert_eq!(open_list.min().unwrap().f(), 10);
    }

    #[test]
    fn test_ignore_existing_key_with_higher_cost() {
        let mut open_list = OpenList::new();
        let original_state = TestState { id: 1, g_cost: 5, h_cost: 5 };   // f = 10
        let worse_state = TestState { id: 1, g_cost: 10, h_cost: 10 }; // f = 20

        open_list.insert(original_state.key(), original_state.clone());
        assert_eq!(open_list.min().unwrap().f(), 10);

        // Try to insert the same key, but with a worse (higher) f-cost
        open_list.insert(worse_state.key(), worse_state);

        // The list should still have only one element
        assert_eq!(open_list.heap.len(), 1);

        // The element should be the original one, as it was better
        assert_eq!(open_list.min(), Some(&original_state));
        assert_eq!(open_list.min().unwrap().f(), 10);
    }
}
