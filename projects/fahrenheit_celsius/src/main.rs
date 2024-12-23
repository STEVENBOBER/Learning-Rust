use std::io;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    loop {
        println!("Enter Fahrenheit to convert to Celsius(or type 'exit' to quit):");

        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        if fahrenheit.trim().eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        let result = fahrenheit_to_celsius(fahrenheit);
        println!("{fahrenheit}°F is equal to {result:.2}°C");
    }
}
