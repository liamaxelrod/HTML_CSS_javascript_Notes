// ==========================================================
// Rust Loops: loop, while, and for
// Corresponds to HTML template: <!-- RUST: loops, while, for. --> in N/Rust/#
// ==========================================================

fn main() {
    println!("=== Rust Loops Demo ===\n");

    // SECTION 1: Infinite loop example
    // ⚠️ This example would run forever if not stopped manually (Ctrl + C),
    // so we'll skip actually running it here — uncomment to test if desired.
    //
    // loop {
    //     println!("This runs forever!");
    // }

    // SECTION 2: loop with break returning a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            // C6: break can return a value!
            break counter * 2; // returns 6
        }
    };
    println!("Result from loop with break: {}\n", result);

    // SECTION 3: while loop countdown
    let mut number = 3;
    while number != 0 {
        if number == 2 {
            number -= 1;
            continue; // C7: skips printing 2
        }
        println!("Countdown: {}", number);
        number -= 1;
    }
    println!("LIFTOFF!\n");

    // SECTION 4: for loop over collection
    let arr = [10, 20, 30];
    for val in arr.iter() { // arr.iter() gives an iterator over references so 10, 20, 30 are borrowed by val
        if *val == 20 { //*val dereferences means go follow this reference and get the value it points to.
            // C6: break exits the loop early
            break;
        }
        println!("Value: {}", val);
    }

    println!("\n=== End of demo ===");
}

