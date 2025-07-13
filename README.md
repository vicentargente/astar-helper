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
    <img alt="Initial puzzle state" src="data:image/webp;base64,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" style="max-width: 100%; height: auto;">
    <img alt="Initial puzzle state" src="data:image/webp;base64,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" style="max-width: 100%; height: auto;">
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