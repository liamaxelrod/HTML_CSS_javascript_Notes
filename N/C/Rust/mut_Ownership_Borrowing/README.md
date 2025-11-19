# Title / What it covers

**Rust Ownership, Borrowing, and Mutable References**

---

# Description

This snippet demonstrates Rust’s core memory safety principles: ownership, borrowing, cloning, and mutable references. It shows how variables are copied, moved, borrowed, and modified safely. Beginner-friendly explanations highlight why Rust enforces these rules and how to work with both stack and heap data.

---

# Code Purpose

The purpose of this code is to teach beginners how Rust manages memory and variable access. It illustrates when ownership is transferred, when values can be cloned, how to borrow references safely, and how mutable references allow controlled in-place modifications. These concepts are fundamental for writing safe and efficient Rust programs.

---

# How it works

1. **Ownership Rules**
   * `ownership_rules()` shows that **Copy types** (like `i32`) are duplicated automatically when assigned, leaving the original variable usable.
   * For **non-Copy types** (like `String`), assigning moves ownership unless explicitly cloned.
2. **Cloning**
   * `clone_example()` demonstrates creating a duplicate of a non-Copy type (`String`) using `.clone()`.
   * Both the original and cloned variable are valid and independent.
3. **Borrowing**
   * `borrowing_example()` shows how a variable can be **borrowed** using `&`.
   * Borrowing allows read-only access without transferring ownership, keeping the original variable usable.
4. **Mutable Borrowing**
   * `mutable_borrow_example()` demonstrates using `&mut` to temporarily mutate a variable.
   * The mutable borrow is limited to its scope; after it ends, the original variable remains usable.
5. **Guest List Example**
   * Combines cloning, borrowing, and mutable borrowing in a practical scenario:
     * Clone → create a backup copy.
     * Borrow → read-only access to count guests.
     * Mutable borrow → safely modify the guest list in place.

---

# Output / Example Run

```
--- Ownership Rules ---
x: 5, y: 5

--- Clone Example ---
x: hello, y: hello

--- Borrowing Example ---
Borrowed reference: hello

--- Mutable Borrow Example ---
Modified via mutable reference: hello world
Original after mutable borrow: hello world

--- Guest List Example ---
Backup copy: Alice, Bob
Current guests: 2
Updated list: Alice, Bob, Carol
```

---

> Note: Rust prevents accidental use of moved values. If you tried to use a non-Copy variable after a move without cloning or borrowing, the compiler would generate an error.

---

# Key Concepts / Lessons

* **Ownership** : Each value has a single owner; when ownership is moved, the original variable becomes invalid.
* **Copy Types vs Non-Copy Types** : Small stack types (like integers) are copied automatically; heap-allocated types (like `String`) require cloning or borrowing.
* **Cloning** : Creates a full independent copy of a value on the heap.
* **Borrowing (`&`)** : Allows read-only access without transferring ownership.
* **Mutable Borrowing (`&mut`)** : Temporarily allows modification of a value while enforcing safe access rules.
* **Scope & Lifetime** : References are valid only within their scope; Rust ensures memory safety automatically.

---

# Tips / Beginner Notes

* Try modifying the guest list with multiple mutable borrows in nested scopes to see Rust’s borrowing rules in action.
* Experiment with cloning large `Vec` or `String` objects to see how it affects memory.
* Observe compile-time errors if you try to use a moved value — these teach safe ownership handling.
* Remember: only **one mutable reference** is allowed at a time, but multiple immutable references are allowed.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace `https://example.com` with the actual link to your code template or website when ready.
