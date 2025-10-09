// Rust Operations - beginner-friendly examples
// Corresponds to HTML template: <!-- Rust: Operations --> in N/Rust/#


fn main() {
// -----------------------------
// 1. ARITHMETIC OPERATORS
// -----------------------------
let sum = 5 + 3; // addition
let difference = 5 - 3; // subtraction
let product = 5 * 3; // multiplication
let quotient = 10 / 2; // division
let remainder = 10 % 3; // remainder (modulus)


println!("Arithmetic Operators:");
println!("5 + 3 = {}", sum);
println!("5 - 3 = {}", difference);
println!("5 * 3 = {}", product);
println!("10 / 2 = {}", quotient);
println!("10 % 3 = {}", remainder);
println!("");


// -----------------------------
// 2. ASSIGNMENT OPERATORS
// -----------------------------
let mut x = 5; // must be mutable to change its value


println!("Assignment Operators:");
println!("Start: x = {}", x);


x += 3; // x = x + 3
println!("After x += 3 → x = {}", x);


x -= 2; // x = x - 2
println!("After x -= 2 → x = {}", x);


x *= 4; // x = x * 4
println!("After x *= 4 → x = {}", x);


x /= 2; // x = x / 2
println!("After x /= 2 → x = {}", x);


x %= 2; // x = x % 2
println!("After x %= 2 → x = {}", x);
println!("");


// -----------------------------
// 3. COMPARISON OPERATORS
// -----------------------------
println!("Comparison Operators:");
println!("5 == 5 → {}", 5 == 5);
println!("5 != 3 → {}", 5 != 3);
println!("7 > 3 → {}", 7 > 3);
println!("2 < 5 → {}", 2 < 5);
println!("5 >= 5 → {}", 5 >= 5);
println!("3 <= 4 → {}", 3 <= 4);
println!("");


// -----------------------------
// 4. LOGICAL OPERATORS
// -----------------------------
let a = true;
let b = false;


println!("Logical Operators:");
println!("true && false → {}", a && b); // AND
println!("true || false → {}", a || b); // OR
println!("!true → {}", !a); // NOT
println!("");


// -----------------------------
// NOTES / LEARNING TIPS
// -----------------------------
println!("Notes:");
println!("- Arithmetic operators work on numeric types (i32, f32, etc.)");
println!("- Assignment operators require mutable variables (mut)");
println!("- Comparison and logical operators return true or false");
println!("- Rust never auto-converts between number types — you must cast manually if needed");
println!("");


}

