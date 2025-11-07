// ===========================================
// RUST: Scope and Shadowing
// ===========================================

// SECTION 1: Function scope
fn inside_function() {
    // 'x' exists only inside this function.
    // Once we leave, it's gone (function scope).
    let x = 5;
    println!("Inside function, x = {}", x);
}

// SECTION 2: Block scope
fn inside_block() {
    let y = 10; // outer y
    {
        // This 'y' is a new variable that only lives inside this block.
        // It shadows the outer 'y', meaning it temporarily replaces it.
        let y = 20;
        println!("Inside inner block, y = {}", y);
    }
    // The inner 'y' is now out of scope.
    println!("Outside inner block, y = {}", y);
}

// SECTION 3: Shadowing in the same scope
fn shadowing_example() {
    let z = 2;
    // Shadowing: declare a new variable with the same name.
    // The old 'z' is dropped; this new one replaces it.
    let z = z + 3;
    println!("After shadowing, z = {}", z);

    // Contrast with 'mut' (same variable, different value):
    let mut a = 1;
    a = 2; // same variable, just updated
    println!("Mutable variable changed: a = {}", a);
}

/*
  💡 Why both exist:
  - Use `mut` when a value *changes over time* (like counters or loop indexes).
  - Use shadowing when you want to *redefine* a value safely or *change its type*.
  Shadowing gives you flexibility while keeping immutability by default.
*/

// SECTION 4: Main function (entry point)
fn main() {
    inside_function();
    inside_block();
    shadowing_example();
}

