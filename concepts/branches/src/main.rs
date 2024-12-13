fn main() {
    let number = 6;
    // Declare a variable `number` and assign it the value 6.

    // Conditional must evaluate to a boolean.
    // if number { ... }
    // This is invalid because `number` is an integer, not a boolean.

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // Check if `number` is less than 5.
    // Prints "condition was true" if the condition is true; otherwise, prints "condition was false."

    // Handling multiple conditions with `else if`.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // Checks divisibility of `number` by 4, 3, and 2 in sequence.
    // Prints the first match and skips the rest.
    // If no conditions match, it defaults to the `else` block.

    let condition = true;
    // Declare a boolean variable `condition` with the value `true`.

    let number = if condition { 5 } else { 6 };
    // Use an `if` expression in a `let` statement.
    // If `condition` is true, `number` is assigned 5; otherwise, it’s assigned 6.

    println!("The value of number is: {number}");
    // Prints the value of `number` (which is 5 in this case since `condition` is true).
}
