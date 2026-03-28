use std::cmp::Ordering; // Enables comparison between value (Less, Greater, Equal)
use std::io;            // Provides input/output functionality

use rand::Rng;          // Trait required for generating random numbers

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..100);

    // Infinite loop - continues until user guesses correctly
    loop {
        println!("Please input your guess.");

        // Create a mutable string to store user input
        let mut guess = String::new();

        // Read input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input string into a number
        // If parsing fails, restart loop (ignore valid input)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Compare user's guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎉 You win!");
                break;  // Exit loop when correct guess is made.
            }
        }
}
