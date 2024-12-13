// Function that returns a value.
fn five() -> i32 {
    5 // Returns the value 5. No semicolon makes this an expression.
}

fn main() {
    // call `five` function that returns 5
    let y = five();
    // Calls `plus_one` with the argument 5 and assigns the result (6) to `x`.
    let x = plus_one(5);

    // Prints the value of `y` to the console.
    println!("The value of y is: {y}");

    // Prints the value of `x` to the console.
    println!("The value of x is: {x}");

    // Calls `another_function` with 5 as the argument.
    another_function(5);

    // Calls `print_labeled_measurement` with a value and unit label.
    print_labeled_measurement(5, 'h');
}

// Adds 1 to the given number and returns the result.
fn plus_one(x: i32) -> i32 {
    x + 1 // Expression that evaluates to `x + 1`. No semicolon = return value.
          // Adding a semicolon here would make it a statement and break the function.
}

// Prints the value of `x` to the console.
fn another_function(x: i32) {
    println!("The value of x is: {x}"); // No return value, just side effect (printing).
}

// Prints a measurement with a unit label.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}"); // Combines value and label into a formatted string.
}

// Key Notes:
// - Statements perform actions but don't return values (e.g., variable declarations).
// - Expressions evaluate to values and can be returned or assigned.
