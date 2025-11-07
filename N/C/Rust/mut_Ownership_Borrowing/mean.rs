// Rust: mut, Ownership & Borrowing
// This example demonstrates ownership, cloning, borrowing, and mutable references in Rust.

fn main() {
    println!("--- Ownership Rules ---");
    ownership_rules();

    println!("\n--- Clone Example ---");
    clone_example();

    println!("\n--- Borrowing Example ---");
    borrowing_example();

    println!("\n--- Mutable Borrow Example ---");
    mutable_borrow_example();

    println!("\n--- Guest List Example ---");
    guest_list_example();
}

// Ownership rules
fn ownership_rules() {
    let x = 5;
    let y = x; // Copy types (like integers) implement Copy, so x is still valid
    println!("x: {}, y: {}", x, y);

    // Note: For non-Copy types like String, x would be moved to y
}

// Clone example
fn clone_example() {
    let x = String::from("hello");
    let y = x.clone(); // explicitly clone value
    println!("x: {}, y: {}", x, y); // both usable
}

// Borrowing example
fn borrowing_example() {
    let s = String::from("hello");
    let r = &s; // borrow s
    println!("Borrowed reference: {}", r);
    // s is still usable here because we only borrowed it
}

// Mutable reference example
fn mutable_borrow_example() {
    let mut s = String::from("hello");
    {
        let r = &mut s; // mutable borrow
        r.push_str(" world");
        println!("Modified via mutable reference: {}", r);
    } // mutable borrow ends here
    println!("Original after mutable borrow: {}", s);
}

// Guest list example demonstrating clone, borrow, and mutable borrow
fn guest_list_example() {
    let mut guest_list = String::from("Alice, Bob");

    // Clone → backup copy before editing
    let backup = guest_list.clone();
    println!("Backup copy: {}", backup);

    // Borrow → read without taking ownership
    print_guest_count(&guest_list);

    // Mutable borrow → temporarily modify original list
    add_guest(&mut guest_list);

    println!("Updated list: {}", guest_list);
}

// Borrowing: read-only
fn print_guest_count(list: &String) {
    let count = list.split(", ").count();
    println!("Current guests: {}", count);
}

// Mutable borrow: modify
fn add_guest(list: &mut String) {
    list.push_str(", Carol");
}