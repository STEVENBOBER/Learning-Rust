use rand::Rng; // Random number generator for creating the secret number.
use std::cmp::Ordering; // Enum for comparing guesses to the secret number.
use std::io; // Module for reading user input.

fn main() {
    println!("Guess the number!"); // Game starts, prompts the user.

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Generate a random number between 1 and 100.

    // Uncomment for debugging: prints the secret number.
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess."); // Prompt for the next guess.

        let mut guess = String::new();
        // Mutable variable to store user input as a string.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Read input and handle potential errors.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // Successfully parsed input to a number.
            Err(_) => continue, // Invalid input, skip to the next loop iteration.
        };

        println!("You guessed: {guess}"); // Display the guess.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"), // Guess is too low.
            Ordering::Greater => println!("Too big!"), // Guess is too high.
            Ordering::Equal => {
                println!("You win!"); // Correct guess, user wins.
                break; // Exit the loop and end the game.
            }
        }
    }
}
