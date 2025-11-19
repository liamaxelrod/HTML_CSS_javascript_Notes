# Title / What it covers

**Interactive Rust: match, if/else, and User Input Handling**

---

# Description

This snippet demonstrates how to handle interactive user input in Rust, using `match` for branching, `if/else` for conditionals, and parsing strings into numbers safely. It is beginner-friendly and shows how Rust manages control flow, error handling, and type conversion in a real-world, interactive program.

---

# Code Purpose

The purpose of this code is to teach beginners how to interact with users in Rust and respond to different inputs safely. It demonstrates structured control flow using `match` and `if/else`, handling parsing errors gracefully, and performing calculations based on user input.

---

# How it works

1. **User Input**
   * Uses `io::stdin().read_line(&mut input)` to read input into a mutable `String`.
   * `trim()` removes whitespace and newline characters.
   * Input can be compared using `eq_ignore_ascii_case` to handle case-insensitive commands like `"exit"`.
2. **Control Flow with `match`**
   * The user’s choice (`"1"`, `"2"`, `"3"`) is evaluated with `match`.
   * Each branch executes a different operation: positivity check, even/odd check, or multiply by 10.
   * `_` branch handles invalid choices safely.
3. **Parsing Numbers Safely**
   * `parse::<i32>()` converts the string input into a number.
   * Returns a `Result` type:
     * `Ok(num)` means parsing succeeded.
     * `Err(_)` means parsing failed, prompting a retry.
4. **Using `if/else` for Logic**
   * Within `Ok(num)`, `if/else` is used to determine:
     * Positive, negative, or zero for a number.
     * Even or odd numbers.
     * Multiplication calculation.
5. **Looping**
   * The outer `loop` keeps the program running until the user types `"exit"`.
   * Ensures continuous interaction without restarting the program.

---

# Output / Example Run

```
Main Menu:
1. Test number positivity
2. Check even or odd
3. Multiply by 10
Type 'exit' to quit.

> 1
Enter a number to test if it's positive or negative:
> 7
7 is positive.

Main Menu:
1. Test number positivity
2. Check even or odd
3. Multiply by 10
Type 'exit' to quit.

> 2
Enter a number to check if it's even or odd:
> 8
8 is even.

Main Menu:
1. Test number positivity
2. Check even or odd
3. Multiply by 10
Type 'exit' to quit.

> 3
Enter a number to multiply by 10:
> 5
5 * 10 = 50

Main Menu:
1. Test number positivity
2. Check even or odd
3. Multiply by 10
Type 'exit' to quit.

> exit
Goodbye!
```

> Note: If the user enters a non-number where a number is expected, the program prints `Invalid number. Please try again.` This demonstrates Rust’s `Result` type handling for error safety.

---

# Key Concepts / Lessons

* **User Input** : Reading input with `io::stdin()` and handling it safely.
* **`match` Statement** : Branches logic cleanly and handles default/error cases.
* **`if/else` Conditionals** : Used within branches to evaluate numeric conditions.
* **Parsing with `parse::<i32>()`** : Converts strings to numbers, producing a `Result` type.
* **Error Handling** : `Ok(num)` and `Err(_)` ensure the program handles invalid input gracefully.
* **Loops** : Outer `loop` allows continuous interaction until exit.
* **Type Safety** : Rust enforces that inputs are converted to the expected type before calculations.

---

# Tips / Beginner Notes

* Experiment with adding more menu options or operations to practice branching logic.
* Try entering invalid input (like letters) to see how Rust prevents crashes with `Result`.
* Observe how `trim()` and `eq_ignore_ascii_case` make input handling more user-friendly.
* Use `parse::<f64>()` if you want to accept floating-point numbers instead of integers.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace `https://example.com` with the actual link to your code template or website when ready.

---
