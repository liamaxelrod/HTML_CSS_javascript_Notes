# Title / What it covers

**Rust Loops: loop, while, and for**

---

# Description

This snippet demonstrates the three primary looping constructs in Rust: `loop`, `while`, and `for`. It shows how to create infinite loops, loops with `break` returning values, countdowns with `while`, and iteration over collections with `for`. Beginner-friendly explanations highlight Rust’s control flow features and safe iteration over arrays.

---

# Code Purpose

The purpose of this code is to teach beginners how to repeatedly execute code safely in Rust. It demonstrates different looping mechanisms, how to exit loops early with `break`, how to skip iterations with `continue`, and how to iterate over arrays with references. These are foundational skills for controlling program flow.

---

# How it works

1. **Infinite Loop (`loop`)**
   * A `loop` runs indefinitely until explicitly stopped.
   * ⚠️ In this snippet, the infinite loop is commented out to avoid running forever.
   * Useful for servers or continuous tasks when paired with `break` conditions.
2. **Loop with `break` Returning a Value**
   * `loop` can return a value using `break <value>`.
   * In the example, `counter` increments until it reaches 3, and `break counter * 2` returns 6.
   * This value can be stored in a variable (`result`) and used later.
3. **While Loop**
   * `while` loops continue as long as a condition is `true`.
   * Demonstrates a countdown from 3 to 1.
   * Uses `continue` to skip a specific iteration (skips printing 2).
4. **For Loop over a Collection**
   * `for` iterates over arrays or other collections using iterators.
   * Demonstrates borrowing via `.iter()` and dereferencing with `*val`.
   * Uses `break` to exit early when a certain condition is met (`val == 20`).
5. **Safe Iteration**
   * Rust’s iterators prevent invalid memory access and allow safe read-only or mutable iteration.
   * No manual indexing is required, avoiding off-by-one errors.

---

# Output / Example Run

```
=== Rust Loops Demo ===

Result from loop with break: 6

Countdown: 3
Countdown: 1
LIFTOFF!

Value: 10

=== End of demo ===
```

---

> Note: The infinite `loop` section is commented out to prevent the program from running indefinitely. Uncomment only if you want to experiment with it manually.

---

# Key Concepts / Lessons

* **`loop`** : Infinite loop construct, can return a value with `break`.
* **`while`** : Executes while a condition is true; `continue` can skip iterations.
* **`for`** : Iterates safely over collections; uses iterators and references.
* **Borrowing in Iteration** : `.iter()` returns references; dereferencing with `*` is needed to access values.
* **Control Flow** : `break` exits loops, `continue` skips iterations.
* **Memory Safety** : Rust enforces safe iteration and prevents out-of-bounds errors.

---

# Tips / Beginner Notes

* Try using `loop` with a conditional `break` to simulate a timer or counter.
* Experiment with `.iter_mut()` in a `for` loop to modify elements safely.
* Replace the array with a `Vec` to see iteration over heap-allocated collections.
* Observe how `break` and `continue` affect loop flow — they can simplify complex conditions.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace `https://example.com` with the actual link to your code template or website when ready.
>
