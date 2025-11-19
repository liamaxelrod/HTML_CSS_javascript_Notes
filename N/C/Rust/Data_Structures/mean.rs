// WARNING: ENUM and STRUCT Don't use certain Variables in the list. As such Rust gives A warning. This is normal and can be ignored if you choose so. 
// ================================================
// Rust: Common Data Structures
// Arrays, Vectors, Tuples, HashMaps, Structs, Enums
// ================================================
//
// This file shows clear, beginner-friendly examples of:
// 1. Arrays (fixed size)
// 2. Vectors (growable lists)
// 3. Tuples (group different types together)
// 4. HashMaps (key → value)
// 5. Structs (custom types with named fields)
// 6. Enums (different states or choices)
//
// Each section is isolated so you can play with it.
// ================================================

use std::collections::HashMap; // Needed for HashMap

fn main() {
    println!("--- ARRAY EXAMPLE ---");
    array_example();

    println!("\n--- VECTOR EXAMPLE ---");
    vector_example();

    println!("\n--- TUPLE EXAMPLE ---");
    tuple_example();

    println!("\n--- HASHMAP EXAMPLE ---");
    hashmap_example();

    println!("\n--- STRUCT EXAMPLE ---");
    struct_example();

    println!("\n--- ENUM EXAMPLE ---");
    enum_example();
}

// ---------------------------
// 1. ARRAY
// ---------------------------
fn array_example() {
    // Array: fixed-size list of values (cannot grow or shrink)
    let temperatures: [i32; 7] = [70, 72, 68, 65, 74, 73, 71];

    println!("Monday temperature: {}", temperatures[0]);

    // Loop through array with index + value
    for (day, temp) in temperatures.iter().enumerate() {
        println!("Day {}: {}°F", day + 1, temp);
    }
}

// ---------------------------
// 2. VECTOR
// ---------------------------
fn vector_example() {
    // Vector: growable list of values
    let mut shopping_list: Vec<&str> = vec!["eggs", "milk", "bread"];

    // Add items at runtime
    shopping_list.push("apples");
    shopping_list.push("orange juice");

    // Loop through vector
    for item in &shopping_list {
        println!("- {}", item);
    }
}

// ---------------------------
// 3. TUPLE
// ---------------------------
fn tuple_example() {
    // Tuple: group different types together
    let user_profile: (&str, i32, bool) = ("Alice", 30, true);

    println!(
        "Name: {}, Age: {}, Active: {}",
        user_profile.0, user_profile.1, user_profile.2
    );

    // Example function returning a tuple
    let numbers = [4, 7, 1, 9];
    let (min, max) = min_max(&numbers);
    println!("Min: {}, Max: {}", min, max);
}

// Helper function for tuple example
fn min_max(numbers: &[i32]) -> (i32, i32) {
    // Start with first number
    let mut min = numbers[0];
    let mut max = numbers[0];

    // Update min/max
    for &num in numbers {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    (min, max) // return tuple
}

// ---------------------------
// 4. HASHMAP
// ---------------------------
fn hashmap_example() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 90);
    scores.insert("Bob", 75);
    scores.insert("Charlie", 82);

    // Safe lookup using .get() which returns Option<&i32>
    if let Some(score) = scores.get("Alice") {
        println!("Alice scored: {}", score);
    }

    // Iterate through all key-value pairs
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}

// ---------------------------
// 5. STRUCTS
// ---------------------------
fn struct_example() {
    // Simple struct with two fields
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    println!("Point coordinates: ({}, {})", p1.x, p1.y);

    // More complex struct
    struct Person {
        name: String,
        age: u8,
        active: bool,
    }

    let user = Person {
        name: "Bob".to_string(),
        age: 25,
        active: true,
    };

    println!("{} is {} years old", user.name, user.age);
}
// WARNING: The field `active` is never read anywhere in the program.
// Rust gives this warning when a struct has a field that is defined
// but never actually used. The code still works fine, but Rust wants
// you to know that this field currently serves no purpose.

// ---------------------------
// 6. ENUMS
// ---------------------------
fn enum_example() {
    // Enum with 3 possible states
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down."),
        TrafficLight::Green => println!("Go!"),
    }
}
// WARNING: The variants `Yellow` and `Green` are never used in the code.
// Rust warns you when some enum variants are defined but never constructed.
// The enum is correct, but these variants are just unused in this program.