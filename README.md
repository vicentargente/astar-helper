# A\* Helper for Rust

This is a small Rust library designed to simplify the implementation of the A\* pathfinding algorithm. It provides a framework where you only need to define the logic for your problem's `State`, and the library will handle the A\* search for you.

This project includes two main variants of the A\* solver: a **traced** version that records the path to the solution, and an **untraced** version that only returns the final state. It also demonstrates a performance optimization using `Rc` for state keys to avoid costly cloning operations.

-----

## 🌟 Features

  * **Generic A\* Implementation**: Easily adaptable to any problem that can be modeled with states and transitions.
  * **Traced and Untraced Solvers**: Choose between getting the full path to the solution or just the final state.
  * **Performance-conscious Design**: Guidance on using `Rc` to optimize for speed and memory, especially with complex state keys.
  * **Ready-to-run Example**: Includes a solver for a challenging puzzle from "Professor Layton and the Curious Village" to showcase the library's usage.

-----

## 🚀 Getting Started

To use this A\* helper in your project, you need to define a `State` for your problem that implements the `AStarState` trait.

### `AStarState` Trait

This is the core trait of the library. Your state representation must implement the following methods:

```rust
pub trait AStarState<K>
where
    K: Clone + Eq + Hash,
    Self: Sized
{
    fn key(&self) -> K;
    fn h(&self) -> usize; // Heuristic cost to goal
    fn f(&self) -> usize; // Total cost (usually g + h)
    fn g(&self) -> usize; // Cost from start
    fn is_goal(&self) -> bool;
}
```

  * `key()`: Returns a unique identifier for the state. This is used to keep track of visited states.
  * `h()`: The heuristic function, which estimates the cost from the current state to the goal.
  * `g()`: The cost of the path from the start node to the current state.
  * `f()`: The total estimated cost of a solution through the current state (`f = g + h`). The A\* algorithm will prioritize states with a lower `f` value.
  * `is_goal()`: A function that returns `true` if the current state is the goal.

Depending on whether you need to trace the solution path, you will also need to implement either `UntracedState` or `TracedState`.

-----

## ↔️ Traced vs. Untraced Solvers

This library provides two different solver functions, `traced_astar` and `untraced_astar`.

### Untraced Solver

The **untraced** solver is simpler and slightly faster. It's ideal for problems where the solution is implicitly contained within the final state, and you don't need to know the exact sequence of steps taken to get there. To use it, your state must implement the `UntracedState` trait:

```rust
pub trait UntracedState<K>: AStarState<K>
where
    K: Clone + Eq + Hash,
{
    fn generate_successors(&self) -> Vec<Self>;
}
```

Then, you can call the solver like this:

```rust
use astar_helper::untraced::untraced_astar::untraced_astar;

// Assuming 'initial_state' is an instance of your state struct
if let Some(result) = untraced_astar(initial_state) {
    println!("Solution found in {} iterations!", result.iterations);
    // The final state is in result.final_state
}
```

### Traced Solver

The **traced** solver is the most common choice. It allows you to store a "change" object at each step, which is used to reconstruct the full path from the initial state to the goal. This is useful for providing step-by-step solutions. To use the traced solver, your state must implement the `TracedState` trait:

```rust
pub trait TracedState<K, C>: AStarState<K>
where
    K: Clone + Eq + Hash,
{
    fn generate_traced_successors(&self) -> Vec<(Self, C)>;
}
```

Here, `C` is the type of your "change" object (e.g., a `Movement` enum). The solver is called similarly:

```rust
use astar_helper::traced::traced_astar::traced_astar;

// Assuming 'initial_state' is an instance of your state struct
if let Some(result) = traced_astar(initial_state) {
    println!("Solution found in {} iterations!", result.iterations);
    println!("Path to solution: {:?}", result.path);
}
```

-----

## ⚡ Performance: `Clone` vs. `Rc` Keys

The A\* algorithm needs to store and compare state keys frequently. If your key is a large or complex object, cloning it repeatedly can become a performance bottleneck.

To mitigate this, you can return a reference-counted pointer (`Rc<YourKey>`) instead of the key itself. This way, only the pointer is cloned, which is much cheaper than cloning the entire key data.

To enforce this pattern and prevent accidental clones of the key, you can define your key `struct` without deriving `Clone`.

See the example bellow to see the performance of each approach.

-----

## 🧩 Example: Professor Layton's Puzzle 132

This project includes a solver for **Puzzle 132** from *Professor Layton and the Curious Village*. The puzzle is a sliding block puzzle where the goal is to move a specific 2x2 red block to a target location on the board.

<div style="display: flex; flex-wrap: wrap; gap: 10px; justify-content: center;">
    <img alt="Initial puzzle state" src="https://i.imgur.com/RRGvLog.png" style="max-width: 100%; height: auto;">
    <img alt="Initial puzzle state" src="https://i.imgur.com/IoqZc5L.png" style="max-width: 100%; height: auto;">
</div>

This example is implemented in two ways to showcase the library's features (they can be found in the examples folder for reference):

1.  `layton1_puzzle_132_cloning_keys.rs`: A version that clones the state key.
2.  `layton1_puzzle_132_rc_keys.rs`: A version that uses `Rc<PuzzleKey>` for better performance.

The `PuzzleKey` struct is designed to be an optimization itself. Two puzzle states are considered equal if pieces of the same size are in the same positions, regardless of the pieces' unique IDs. To achieve this, the key consists on an array of (coordinates, piece_size), avoiding checking the same piece more than once and always in the same order, so that two states that are visually the same, are also considered the same by the algorithm.

Due to this design, which offers us a really massive performance improvement, the key must be an array, which could produce a small overhead when being cloned. In this case could be interesting wrapping the key with an Rc, and not deriving nor implementing cloning on our key to prevent internal .clone() from cloning our object instead of the Rc.

The benchmark bellow shows a noticeable performance improvement when using `Rc` for the puzzle's key.

Here is a summary of the benchmark results (1000 runs, times in milliseconds):

| operation             |   avg    |    med   |    p95   |    p99   |   max   |   min   |
| --------------------- | :------: | :------: | :------: | :------: | :-----: | :-----: |
| traced\_cloning\_keys   | 130.767  | 130.283  | 132.836  | 137.242  | 186.361 | 128.833 |
| traced\_rc\_keys      | 123.648  | 122.407  | 128.946  | 133.710  | 176.683 | 120.988 |
| untraced\_cloning\_keys | 107.850  | 107.596  | 109.004  | 111.952  | 131.772 | 106.795 |
| untraced\_rc\_keys    | 102.179  | 101.505  | 105.006  | 108.724  | 125.847 | 101.040 |

-----

## 📜 License

This project is licensed under the MIT License. See the `LICENSE` file for details.