// Types of loops in Rust:

// loop: Runs indefinitely unless exited with break. Can return a value.
// while: Runs as long as a condition is true.
// for: Iterates over a collection or range, handling boundaries automatically.

fn main() {
    let mut counter = 0;
    // Declare a mutable variable `counter` initialized to 0.

    let result = loop {
        counter += 1;
        // Increment the counter by 1 on each loop iteration.

        // Break the loop when `counter` equals 10, returning `counter * 2` as the loop's result.
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("main function result is {result}");
    // Prints the result of the loop (`20` in this case).

    println!("-----------------");
    multiple_loops(); // Call the `multiple_loops` function.
    println!("-----------------");
    w_loop(); // Call the `w_loop` function.
    println!("-----------------");
    f_loop(); // Call the `f_loop` function.
}

fn multiple_loops() {
    let mut count = 0;
    // Mutable variable `count` starts at 0.

    'counting_up: loop {
        println!("count = {count}");
        // Print the current value of `count`.

        let mut remaining = 10;
        // Mutable variable `remaining` starts at 10 for the inner loop.

        loop {
            println!("remaining = {remaining}");
            // Print the value of `remaining`.

            if remaining == 9 {
                break;
                // Exit the inner loop when `remaining` equals 9.
            }
            if count == 2 {
                break 'counting_up;
                // Exit both loops when `count` equals 2 (label used).
            }

            remaining -= 1;
            // Decrement `remaining` by 1.
        }

        count += 1;
        // Increment `count` by 1.
    }
    println!("End count = {count}");
    // Print the final value of `count`.
}

fn w_loop() {
    let mut number = 3;
    // Mutable variable `number` starts at 3.

    while number != 0 {
        // Run the loop until `number` reaches 0.
        println!("{number}!");
        // Print the current value of `number`.

        number -= 1;
        // Decrement `number` by 1.
    }

    println!("HIT 0!!!!!");
    // Print a message when the loop ends.
}

fn f_loop() {
    let a = [10, 20, 30, 40, 50];
    // Array `a` with 5 elements.

    for element in a {
        println!("The value is: {element}");
        // Iterate over the array and print each value.
    }

    for number in (1..4).rev() {
        // Create a range from 1 to 4 (exclusive), reverse it, and iterate.
        println!("{number}!");
        // Print the current number in the countdown.
    }

    println!("LIFTOFF!!");
    // Print a message after the countdown ends.
}
