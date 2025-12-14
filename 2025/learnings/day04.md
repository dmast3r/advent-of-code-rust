## Key Rust Learnings
1. The Trap: Nested Iterators & Closures

I tried to replace a nested for loop with a functional flat_map chain.

**My Initial Code (Failed)**:

```rust
(0..m).flat_map(|i| {
    // ERROR: 'i' lives in this outer closure...
    (0..n).map(|j| {
        // ...but this inner closure tries to borrow it by reference (&i).
        // The outer closure finishes (returns the iterator) and 'i' dies
        // before the inner iterator ever runs!
        (i, j) 
    })
})
```

**Why it failed**: Rust iterators are lazy. `flat_map` creates a struct (a factory) and returns it. It doesn't run the loop immediately. By the time I actually call `.sum()` or `.collect()` later, the stack frame where i existed is gone. The inner iterator would be pointing to dead memory.

2. The Solution: The move Keyword
To fix the dead reference, I need the inner closure to own its own copy of i.

**The Fix**

```rust
(0..n).map(move |j| { ... })
```

`move` forces the closure to copy captured variables (`i`) into its own struct definition instead of holding a pointer. Since `i` is a `usize` (primitive integer), this copy is cheap and solves the lifetime issue.

3. The New Problem: "Moving" the Grid
When I added `move`, it tried to move everything the closure used—including the grid (`Vec<Vec<char>>`).

**Issue**: Vec is not Copy. If the first iterator (row 0) takes ownership of the `grid`, the next iterator (row 1) has nothing left to use.

**Fix**: Explicitly pass a reference.

**Final Working Pattern:**

```rust
(0..m).flat_map(|i| {
    // 1. Create a lightweight reference (pointer)
    let grid_ref = &grid; 

    // 2. Use 'move'. 
    // - 'i' is copied (it's an int).
    // - 'grid_ref' is copied (it's just a pointer).
    // The heavy 'grid' stays safely owned by the main function.
    (0..n).map(move |j| {
        if is_valid_paper(i, j, grid_ref) { 1 } else { 0 }
    })
})
```

4. Why Default is Reference?
Rust closures capture by reference (`&T`) by default because:

* **Performance**: Copying large data (like a `Vec`) is slow.
* **Ownership**: It prevents the closure from accidentally "eating" variables that the outer function might still need.

## Summary
* **Iterators** = Structs: They are state machines that run later, not loops that run now.
* *`*move`**: Use it when returning a closure or creating an iterator that outlives the current scope.

### Capture Hierarchy:
* Default: Borrow `(&T) -> Fast`, but requires valid scope.
* move + Copy type: Copies values (Safe for int/bool).
* move + Non-Copy type: Steals ownership (Variable gone from outer scope).
* move + &Non-Copy: Copies the pointer (Safe shared access).