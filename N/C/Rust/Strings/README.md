# Title / What it covers

**Strings in Rust: &str, String, Mutability, Concatenation, and Lengths**

---

# Description

This snippet demonstrates different ways to handle text in Rust, including immutable string slices (`&str`), owned `String` types, mutable strings, string concatenation, and measuring string lengths, including UTF-8 considerations. It’s beginner-friendly and focuses on ownership, borrowing, and safe manipulation of text.

---

# Code Purpose

The purpose of this code is to teach beginners how Rust manages text data. It shows the difference between borrowed slices (`&str`) and owned strings (`String`), how to modify strings safely, and how concatenation works differently with `+` versus `format!`. It also illustrates how Rust treats string lengths in bytes versus characters.

---

# How it works

1. **String Slice (`&str`)**
   * `string_slice_example()` demonstrates an immutable, borrowed view of a string literal.
   * `&str` does not own the data, so it’s fast and memory-safe, but cannot be modified.
2. **Owned String (`String`)**
   * `owned_string_example()` shows two ways to create an owned string: `.to_string()` and `String::from()`.
   * Owned strings hold their own memory on the heap. By default, they are immutable unless declared `mut`.
3. **Mutable String**
   * `mutable_string_example()` declares a mutable `String` and appends content with `.push_str()`.
   * Mutability allows safe in-place modification of heap-allocated data.
4. **Concatenation**
   * `concatenation_example()` demonstrates two ways to combine strings:
     * Using `+` moves the first string (`s1`) and borrows the second (`&s2`).
     * Using `format!` creates a new string without taking ownership, leaving originals intact.
5. **String Lengths and UTF-8**
   * `string_lengths_example()` shows how `.len()` returns  **bytes** , while `.chars().count()` returns  **Unicode scalar values** .
   * Demonstrates emoji handling: one emoji may take multiple bytes but counts as a single character.

---

# Output / Example Run

```
&str slice: Hello
Owned Strings: s1 = Hi, s2 = Hola
Mutable String after push_str: Hello, world!
Concatenated with + : HiHola
Concatenated with format!: Hola Hello, world!
Length in bytes: 17
Length in characters: 16
Emoji '😊' length in bytes: 4, length in chars: 1
```

---

> Note: Using `+` operator moves `s1`, so any attempt to use it afterward will cause a compile-time error. This demonstrates Rust’s ownership rules in practice.

---

# Key Concepts / Lessons

* **Ownership** : `String` owns its heap-allocated memory; `&str` is a borrowed view.
* **Borrowing** : `&s2` lets `+` concatenate without transferring ownership.
* **Mutability** : `mut` allows modifying an owned string safely.
* **Shadowing vs Mutation** : Not used here, but good to remember you can reassign variables with `let` for shadowing.
* **String Concatenation** : `+` consumes the first string; `format!` creates a new string.
* **UTF-8 Awareness** : `.len()` counts bytes; `.chars().count()` counts Unicode characters, crucial for emojis or multi-byte symbols.

---

# Tips / Beginner Notes

* Experiment with adding `mut` to `s2` and trying `.push_str()` to see compile-time restrictions.
* Try concatenating multiple strings with `format!` to see how ownership differs from `+`.
* Use `.chars().count()` whenever working with Unicode-aware string operations (like counting emoji).
* Always consider ownership and borrowing when passing strings to functions — Rust prevents unsafe memory use at compile time.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace the `https://example.com` with the actual link to your code template or website when ready.
