// Rust Basics - beginner-friendly examples
// Corresponds to HTML template: <!-- Rust: Basics --> in N/Rust/#

fn main() {
    // -----------------------------
    // 1. COMMENTS
    // -----------------------------
    println!("// Single-line comment: ignored by Rust compiler");
    println!("/* Block comment: also ignored */");
    println!("");

    // -----------------------------
    // 2. VARIABLES
    // -----------------------------
    let x = 5;         // immutable integer
    let mut y = 10;    // mutable integer

    println!("let x = 5 : x = {}", x);  // prints: x = 5
    println!("let mut y = 10; : y = {}", y);  // prints: y = 10

    y = 20;  // allowed because 'y' is mutable
    println!("y = 20; : y after change = {}", y); // prints: y after change = 20
    println!("");

    // -----------------------------
    // 3. DATA TYPES
    // -----------------------------
    let my_num: i32 = 5;
    let my_double: f64 = 5.99;
    let my_letter: char = 'D';
    let my_bool: bool = true;
    let my_text: &str = "Hello";

    println!("Number: {}, Double: {}, Char: {}, Bool: {}, Text: {}",
             my_num, my_double, my_letter, my_bool, my_text);
    println!("");

    // Type inference
    let inferred_num = 42;        // Rust infers i32
    let inferred_double = 3.14;   // Rust infers f64
    let inferred_bool = false;    // Rust infers bool
    let inferred_text = "Hi!";    // Rust infers &str

    println!("Using Type inference is when you are good with Rust and you wanna make the code look nice, easy to read, neat, and you trust the compiler because you understand how Rust works.");
    println!("Inferred: {}, {}, {}, {}", inferred_num, inferred_double, inferred_bool, inferred_text);
    println!("");

    // -----------------------------
    // 4. PRINTING / PLACEHOLDERS
    // -----------------------------
    print!("Hello ");             // no newline
    println!("World!");           // prints: Hello World!
    println!("");

    let name = "Liam";
    let age = 29;
    println!("");

    println!("Hello, {}", name);                  // prints: Hello, Liam
    println!("{} is {} years old", name, age);   // prints: Liam is 21 years old
    println!("First {0}, Second {1}, Again {0}", 10, 20); // prints: First 10, Second 20, Again 10
    println!("Hi {name}", name="Bob");           // prints: Hi Bob
    println!("");

    // -----------------------------
    // 5. CONSTANTS
    // -----------------------------
    /*
    let x = 5;        // runtime value
    const Y: i32 = 5; // compile-time constant
    Both x and Y can’t be reassigned — but:

    x exists in memory while your program runs,

    Y doesn’t even exist at runtime — the compiler literally replaces it with 5 wherever you use it.

    So const is a constraint on time and scope, not just on changeability.
    */
    const MAX_SPEED: i32 = 120;
    const PI: f64 = 3.14159;
    const MINUTES_PER_HOUR: i32 = 60;

    println!("Max speed: {}, PI: {}, Minutes per hour: {}", MAX_SPEED, PI, MINUTES_PER_HOUR);
}


 