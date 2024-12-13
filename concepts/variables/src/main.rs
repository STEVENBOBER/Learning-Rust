fn main() {
    let x = 5;
    // Declares a new immutable variable `x` with an initial value of 5.
    // The type of `x` is inferred by the compiler as `i32` by default.

    let x = x + 1;
    // Shadows the previous variable `x` with a new immutable variable `x`.
    // This new `x` takes the value of the previous `x` (5) and adds 1, resulting in `x = 6`.

    {
        // Starts a new inner scope (block) where a new variable can be declared or shadowed.
        let x = x * 2;
        // Shadows the outer `x` with a new `x` in this inner scope.
        // Takes the value of the outer `x` (6) and multiplies it by 2, resulting in `x = 12`.
        println!("The value of x in the inner scope is: {x}");
        // Prints the value of the inner `x` (12) to the console.
    }
    // The inner `x` goes out of scope here. The outer `x` (6) becomes accessible again.

    println!("The value of x is: {x}");
    // Since the inner `x` is out of scope, this statement references the `x` defined before the inner block.
}
