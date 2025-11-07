// ======================================================
// RUST: Functions, Parameters, and Return Values
// File: mean.rs
// ======================================================

// SECTION 1: Simple function
fn say_hello() {
    println!("Hello, world!");
}

// SECTION 2: Function with parameters
fn greet(name: &str) { // &str means “borrowed string slice”, not owned data
    // You’re *borrowing* the text "Alice" just long enough to read it.
    // Rust enforces this at compile time — no copies or unsafe memory access.
    println!("Hello, {}!", name);
}

// SECTION 3: Function with explicit return
fn add_explicit(a: i32, b: i32) -> i32 {
    // The `return` keyword immediately exits the function with this value.
    // Any code after a `return` line would never run.
    return a + b;
}

// SECTION 4: Function with implicit return (no semicolon)
fn add_implicit(a: i32, b: i32) -> i32 {
    // In Rust, *every function* is an expression.
    // The last line (without a semicolon) is *automatically returned*.
    // This is the idiomatic, shorter Rust style.
    a + b // ← returned implicitly
    // In Rust, everything is an expression — that means most code produces a value.
    // But a semicolon (;) changes an expression into a statement, which means “do this, but don’t return a value.”
}

// SECTION 5: Calling functions
fn main() {
    say_hello();                  // basic call
    greet("Alice");               // passes a borrowed string literal

    let sum1 = add_explicit(5, 7); // uses explicit `return`
    println!("Sum with explicit return: {}", sum1);

    let sum2 = add_implicit(3, 4); // returns the final expression automatically
    println!("Sum with implicit return: {}", sum2);
}



