# Title / What it covers

**Rust Common Data Structures: Arrays, Vectors, Tuples, HashMaps, Structs, and Enums**

---

# Description

This snippet demonstrates Rust’s most commonly used data structures. Beginners will see how to work with fixed-size arrays, growable vectors, tuples for grouping mixed types, key-value mappings with `HashMap`, custom types with structs, and state representation using enums.

---

# Code Purpose

The purpose of this code is to provide clear, beginner-friendly examples of Rust’s core data structures. It teaches how to store, access, and manipulate data safely and efficiently, emphasizing Rust’s ownership and borrowing rules, as well as memory safety.

---

# How it works

1. **Arrays**
   * Fixed-size collection of values: `let arr: [i32; 7]`.
   * Cannot grow or shrink.
   * Iterated with `.iter().enumerate()` for index-value pairs.
2. **Vectors**
   * Growable, heap-allocated lists: `Vec<T>`.
   * Use `.push()` to add elements at runtime.
   * Iterated safely using `for item in &vec`.
3. **Tuples**
   * Group different types together: `(String, i32, bool)`.
   * Access with `.0`, `.1`, `.2` or destructure: `let (min, max) = min_max(&numbers)`.
   * Functions can return tuples to return multiple values.
4. **HashMaps**
   * Key-value mappings: `HashMap<K, V>`.
   * Insert with `.insert(key, value)`.
   * Safe lookup with `.get()` returns `Option<&V>`.
   * Iterate with `for (k, v) in &hashmap`.
5. **Structs**
   * Custom types with named fields.
   * Fields can be of different types (`String`, `i32`, `bool`).
   * Access fields with `struct_instance.field_name`.
   * Rust may warn about unused fields (like `active`), which is normal.
6. **Enums**
   * Represent distinct states or variants: `enum TrafficLight { Red, Yellow, Green }`.
   * Use `match` to handle each variant safely.
   * Rust may warn if some variants are defined but not used.

---

# Output / Example Run

```
--- ARRAY EXAMPLE ---
Monday temperature: 70
Day 1: 70°F
Day 2: 72°F
Day 3: 68°F
Day 4: 65°F
Day 5: 74°F
Day 6: 73°F
Day 7: 71°F

--- VECTOR EXAMPLE ---
- eggs
- milk
- bread
- apples
- orange juice

--- TUPLE EXAMPLE ---
Name: Alice, Age: 30, Active: true
Min: 1, Max: 9

--- HASHMAP EXAMPLE ---
Alice scored: 90
Alice: 90
Bob: 75
Charlie: 82

--- STRUCT EXAMPLE ---
Point coordinates: (10, 20)
Bob is 25 years old

--- ENUM EXAMPLE ---
Stop!
```

> Notes:
>
> * Warnings about unused struct fields (`active`) or enum variants (`Yellow`, `Green`) are normal in Rust and do not prevent the program from running.

---

# Key Concepts / Lessons

* **Arrays vs Vectors** : Fixed-size vs growable collections.
* **Tuples** : Group multiple values of different types; useful for returning multiple results from a function.
* **HashMap** : Key-value storage, safe lookups, iteration over entries.
* **Structs** : Custom data types, field access, beginner-friendly representation of objects.
* **Enums** : Represent choices or states; match ensures all cases handled.
* **Rust Safety** : Ownership, borrowing, and immutable references apply to all collections.
* **Compiler Warnings** : Rust warns about unused fields/variants to encourage clean, maintainable code.

---

# Tips / Beginner Notes

* Experiment with adding elements to vectors and accessing them by index.
* Try destructuring tuples to access multiple values at once.
* Use `.get()` for HashMaps to safely handle missing keys.
* Add more fields to structs and handle them in the program to see warnings disappear.
* Extend enums with more variants and handle them all in `match`.

---

# Reference / Related Link

[View the template in the Pacific site](https://example.com)

> Replace `https://example.com` with the actual link to your code template or website when ready.

---
