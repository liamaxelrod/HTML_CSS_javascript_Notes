// SECTION 1: String slice (&str)
fn string_slice_example() {
    // Borrowed, fixed, immutable view of text.
    let greeting: &str = "Hello";
    println!("&str slice: {}", greeting);
} 

// Section 1 = borrowed view (&str, immutable), 
// Section 2 = owned string (String, can mutate if mut)

// SECTION 2: Creating owned Strings that can be changed. 
fn owned_string_example() {
    let s1 = "Hi".to_string();          // owned String from literal
    let s2 = String::from("Hola");      // another way to create owned String
    println!("Owned Strings: s1 = {}, s2 = {}", s1, s2);
}

// Section 2 → owned but immutable (cannot grow/change unless declared mut).
// Section 3 → owned and mutable, can append or change content safely.

// SECTION 3: Mutable String
fn mutable_string_example() {
    let mut s3 = String::from("Hello"); // must declare mut to change
    s3.push_str(", world!");            // append text safely
    println!("Mutable String after push_str: {}", s3);
}

// Section 3 = change owned string in place;
// Section 4 = combine strings into a new string

// SECTION 4: Concatenation
fn concatenation_example() {
    let s1 = "Hi".to_string();
    let s2 = String::from("Hola");

    // Using + operator
    // Note: s1 is moved (cannot use s1 afterward), s2 is borrowed
    let s4 = s1 + &s2; // (&) &s2 = borrow s2, so + can append without taking ownership              
    println!("Concatenated with + : {}", s4);

    /*
    + consumes the first string (s1) and appends a borrowed slice (&s2).
    format! leaves all original strings intact and returns a brand-new string.
    */

    // Using format! macro (does NOT take ownership)
    let s5 = format!("{} {}", s2, "Hello, world!"); 
    println!("Concatenated with format!: {}", s5);
}

// SECTION 5: Lengths and UTF-8
fn string_lengths_example() {
    let s = format!("{} {}", "Hola", "Hello, world!");
    
    let length_bytes = s.len();             // length in bytes
    let length_chars = s.chars().count();   // length in Unicode scalar values
    println!("Length in bytes: {}", length_bytes);
    println!("Length in characters: {}", length_chars);

    // EXTRA: Experiment with emoji to see UTF-8 effect
    let emoji = "😊";
    println!("Emoji '{}' length in bytes: {}, length in chars: {}", emoji, emoji.len(), emoji.chars().count());
}

fn main() {
    string_slice_example();
    owned_string_example();
    mutable_string_example();
    concatenation_example();
    string_lengths_example();
}

