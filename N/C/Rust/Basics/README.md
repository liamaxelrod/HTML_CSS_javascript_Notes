# Title / What it covers

**Rust Basics: Comments, Variables, Data Types, Printing, and Constants**

---

# Description

This snippet introduces fundamental Rust concepts for beginners. It demonstrates how to write comments, declare immutable and mutable variables, use different data types, format output with `println!`, leverage type inference, and define compile-time constants.

---

# Code Purpose

The purpose of this code is to teach Rust beginners the essentials needed to start writing safe, readable programs. It covers foundational features, explains their differences, and provides examples of using variables and constants safely while understanding Rust’s strict type system.

---

# How it works

1. **Comments**
   * Single-line (`//`) and block (`/* ... */`) comments are ignored by the compiler.
   * Helps explain code without affecting execution.
2. **Variables**
   * `let x = 5;` defines an immutable variable.
   * `let mut y = 10;` defines a mutable variable that can change.
   * Rust enforces immutability by default to prevent accidental changes.
3. **Data Types**
   * Explicitly declared: `i32`, `f64`, `char`, `bool`, `&str`.
   * Type inference allows Rust to automatically determine types for readability and simplicity.
4. **Printing / Placeholders**
   * `println!` prints with newline; `print!` prints without newline.
   * Placeholders `{}`, `{0}`, `{name}` allow formatted strings.
   * Supports multiple variables, repeated indices, and named parameters.
5. **Constants**
   * `const` defines compile-time constants that exist only during compilation.
   * Immutable and inlined wherever used.
   * Differ from `let`, which is runtime and can optionally be mutable.

---

# Output / Example Run

```
// Single-line comment: ignored by Rust compiler
/* Block comment: also ignored */

let x = 5 : x = 5
let mut y = 10; : y = 10
y = 20; : y after change = 20

Number: 5, Double: 5.99, Char: D, Bool: true, Text: Hello

Using Type inference is when you are good with Rust and you wanna make the code look nice, easy to read, neat, and you trust the compiler because you understand how Rust works.
Inferred: 42, 3.14, false, Hi!
Hello World!

Hello, Liam
Liam is 29 years old
First 10, Second 20, Again 10
Hi Bob

Max speed: 120, PI: 3.14159, Minutes per hour: 60
```

---

# Key Concepts / Lessons

* **Comments** : Document code safely.
* **Immutable vs Mutable Variables** : Rust defaults to immutability; use `mut` to change values.
* **Data Types** : Basic types, type inference, and explicit type declarations.
* **Printing / Formatting** : Use placeholders and formatted strings for clear output.
* **Constants** : Compile-time, immutable, and inlined values.
* **Memory Safety** : Rust’s strict type system ensures correctness at compile time.

---

# Tips / Beginner Notes

* Experiment with changing variables and see Rust’s compile-time errors for immutable vs mutable variables.
* Try different formatting options in `println!` to get comfortable with placeholders.
* Use type inference for concise code, but explicitly declare types when clarity is needed.
* Remember constants (`const`) exist at compile-time and cannot be mutated.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace `https://example.com` with the actual link to your code template or website when ready.

---
