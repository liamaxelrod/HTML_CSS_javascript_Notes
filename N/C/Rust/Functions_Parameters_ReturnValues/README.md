# Title / What it covers

**Rust Functions: Parameters and Return Values**

---

# Description

This snippet demonstrates how to define and use functions in Rust. It covers basic function calls, passing parameters (borrowed strings), explicit and implicit return values, and idiomatic Rust practices. Beginners will learn how to structure reusable code safely and efficiently.

---

# Code Purpose

The purpose of this code is to teach how to define and call functions in Rust, pass data safely using references, and return values either explicitly with `return` or implicitly as the last expression. It reinforces Rust’s memory safety and expression-based syntax.

---

# How it works

1. **Simple Function**
   * `say_hello()` prints a message.
   * Demonstrates the simplest function with no parameters or return value.
2. **Function with Parameters**
   * `greet(name: &str)` takes a borrowed string slice as input.
   * Borrowing allows the function to read the string without taking ownership.
   * Rust ensures memory safety at compile time.
3. **Function with Explicit Return**
   * `add_explicit(a: i32, b: i32) -> i32` uses `return` to return a value immediately.
   * Any code after `return` would not execute.
4. **Function with Implicit Return**
   * `add_implicit(a: i32, b: i32) -> i32` returns the last expression automatically.
   * Omitting the semicolon is key to implicit returns in Rust.
   * This is the idiomatic, concise style preferred by Rustaceans.
5. **Calling Functions**
   * `main()` calls all the defined functions.
   * Demonstrates passing literals, storing return values in variables, and printing results.

---

# Output / Example Run

```
Hello, world!
Hello, Alice!
Sum with explicit return: 12
Sum with implicit return: 7
```

> Note: Borrowed parameters (`&str`) do not create copies; they reference existing data safely.

---

# Key Concepts / Lessons

* **Functions** : Define reusable blocks of code.
* **Parameters** : Accept data from the caller; borrowing with `&` avoids moving ownership.
* **Return Values** : Can be explicit (`return`) or implicit (last expression without semicolon).
* **Expression-Based Syntax** : Everything in Rust can be an expression producing a value.
* **Memory Safety** : Rust enforces safe usage of borrowed references at compile time.

---

# Tips / Beginner Notes

* Experiment with functions returning different types like `f64` or `String`.
* Try defining multiple parameters and using them in calculations or string formatting.
* Observe the difference between using `return` and letting the final expression return implicitly.
* Remember: semicolons turn expressions into statements, which do not return a value.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace `https://example.com` with the actual link to your code template or website when ready.

---
