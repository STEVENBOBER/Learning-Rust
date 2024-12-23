use std::io;

fn fibonacci_iterative(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }

    b
}

fn main() {
    loop {
        println!("Please enter nth Fibonacci number (or type 'exit' to quit):");
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        if n.trim().eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("The {}th fibonacci number is {}", n, fibonacci_iterative(n));
    }
}
