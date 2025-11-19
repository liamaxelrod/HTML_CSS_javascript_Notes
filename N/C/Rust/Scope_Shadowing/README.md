# Title / What it covers

**Rust Scope and Shadowing: Function, Block, and Variable Shadowing**

---

# Description

This snippet demonstrates how Rust handles variable scope and shadowing. It shows function-level scope, block-level scope, and the difference between shadowing and mutable variables. Beginner-friendly explanations help illustrate why Rust enforces these rules and how to use them safely.

---

# Code Purpose

The purpose of this code is to teach beginners how Rust manages variable visibility and lifetime. It shows when variables exist, when they go out of scope, and how shadowing can safely redefine variables without making them mutable. This is important for writing safe and predictable Rust code.

---

# How it works

1. **Function Scope**
   * `inside_function()` demonstrates that a variable declared inside a function (`x`) exists only while the function is running.
   * Once the function ends, `x` is dropped and cannot be accessed.
2. **Block Scope**
   * `inside_block()` demonstrates shadowing inside a nested block.
   * The inner `y` temporarily replaces the outer `y` within the block.
   * After the block ends, the outer `y` is visible again.
3. **Shadowing in the Same Scope**
   * `shadowing_example()` shows how to declare a new variable with the same name (`z`) in the same scope.
   * The original `z` is dropped, and the new one replaces it.
   * Contrasts with `mut`, where the same variable is updated instead of redefined.
4. **Why Both Shadowing and `mut` Exist**
   * Use `mut` for values that change over time (e.g., counters).
   * Use shadowing to safely redefine a value, potentially changing its type, while keeping immutability by default.

---

# Output / Example Run

```
Inside function, x = 5
Inside inner block, y = 20
Outside inner block, y = 10
After shadowing, z = 5
Mutable variable changed: a = 2
```

---

> Note: The inner block variable `y` shadows the outer `y` temporarily. Rust ensures variables cannot be used outside their scope, preventing accidental misuse or undefined behavior.

---

# Key Concepts / Lessons

* **Function Scope** : Variables live only inside the function they are declared in.
* **Block Scope** : Inner blocks can temporarily shadow outer variables.
* **Shadowing vs Mutable Variables** :
* Shadowing creates a new variable with the same name; allows type changes.
* `mut` updates an existing variable; no type change allowed.
* **Immutability by Default** : Rust encourages immutability and safe reassignment through shadowing.
* **Memory Safety** : Once a variable goes out of scope, Rust automatically drops it, preventing dangling references.

---

# Tips / Beginner Notes

* Try shadowing a variable with a different type (e.g., `let x = "hello"; let x = x.len();`) to see type transformation.
* Use nested blocks to temporarily override values safely.
* Observe compile-time errors when trying to access out-of-scope variables — Rust prevents unsafe access.
* Use shadowing to simplify calculations without mutating the original variable.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace `https://example.com` with the actual link to your code template or website when ready.
