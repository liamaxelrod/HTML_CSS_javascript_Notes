# Title / What it covers

**Rust Operators: Arithmetic, Assignment, Comparison, and Logical Operators**

---

# Description

This snippet demonstrates the basic operators in Rust for performing arithmetic, assignment, comparison, and logical operations. It is beginner-friendly and shows how to manipulate variables safely while adhering to Rust’s ownership and mutability rules.

---

# Code Purpose

The purpose of this code is to teach beginners how Rust handles different types of operations. It explains how arithmetic and assignment work, how comparisons evaluate to boolean values, and how logical operations combine or negate conditions. This snippet also highlights the importance of `mut` when changing variable values.

---

# How it works

1. **Arithmetic Operators**
   * Demonstrates addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`), and remainder (`%`).
   * Works on numeric types like integers (`i32`) and floats (`f32`).
2. **Assignment Operators**
   * Shows how to update a mutable variable with `+=`, `-=`, `*=`, `/=`, and `%=`.
   * Rust requires `mut` to modify a variable; otherwise, reassignment is forbidden.
3. **Comparison Operators**
   * Compares values using `==`, `!=`, `>`, `<`, `>=`, and `<=`.
   * All comparisons return `bool` (`true` or `false`).
4. **Logical Operators**
   * Demonstrates `&&` (AND), `||` (OR), and `!` (NOT).
   * Useful for combining boolean expressions and controlling program flow.
5. **Notes / Learning Tips**
   * Arithmetic operators are type-specific; Rust does **not** automatically convert between number types.
   * Assignment operators require mutable variables.
   * Comparison and logical operators are essential for control flow and conditional statements.

---

# Output / Example Run

```
Arithmetic Operators:
5 + 3 = 8
5 - 3 = 2
5 * 3 = 15
10 / 2 = 5
10 % 3 = 1

Assignment Operators:
Start: x = 5
After x += 3 → x = 8
After x -= 2 → x = 6
After x *= 4 → x = 24
After x /= 2 → x = 12
After x %= 2 → x = 0

Comparison Operators:
5 == 5 → true
5 != 3 → true
7 > 3 → true
2 < 5 → true
5 >= 5 → true
3 <= 4 → true

Logical Operators:
true && false → false
true || false → true
!true → false

Notes:
- Arithmetic operators work on numeric types (i32, f32, etc.)
- Assignment operators require mutable variables (mut)
- Comparison and logical operators return true or false
- Rust never auto-converts between number types — you must cast manually if needed
```

---

> Note: Rust’s type system is strict. Operations between mismatched numeric types (e.g., `i32` and `f64`) will cause a compile-time error unless explicitly cast.

---

# Key Concepts / Lessons

* **Arithmetic Operators** : Perform basic calculations safely with no implicit type conversion.
* **Assignment Operators** : Update mutable variables efficiently.
* **Comparison Operators** : Produce `bool` values for conditional logic.
* **Logical Operators** : Combine or invert boolean expressions for decision-making.
* **Mutability** : `mut` is required to modify variable values.
* **Rust Safety** : Compile-time checks prevent accidental type mismatches or unintentional modifications.

---

# Tips / Beginner Notes

* Experiment with floating-point numbers (`f32` / `f64`) to see how arithmetic behaves with decimals.
* Try combining multiple logical operators in a single expression to see precedence rules.
* Observe compile-time errors if you forget `mut` or try to mix incompatible numeric types.
* Use arithmetic and comparison operators together in `if` statements to control program flow.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace `https://example.com` with the actual link to your code template or website when ready.
