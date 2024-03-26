// Bring the 'io' module into scope from the standard library (std)
use std::io;
// Bring the 'cmp' module into scope from the standard library (std)
use std::cmp::Ordering;
// The Rng trait defines methods that random number generators implement
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!!!");
    println!("To exit the game, type 'q' and press Enter.");

    // Generating the secret Random Number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    
    loop {
        println!("\nMake your guess below! (between 1 and 100 inclusive)");
        // Creating a variable to store the user's guess
        let mut guess = String::new();

        // Receiving the user's input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converting the user's guess to an integer-type (Using Shadowing)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                // Validating the user's guess
                if num < 1 || num > 100 {
                    println!("Please input a number between 1 and 100!");
                    continue;
                }

                // Store the valid integer guess in the shadowing 'guess' variable
                num
            },
            Err(_) => {
                // User manually exits the game
                if guess.trim().to_lowercase() == "q" {
                    println!("Exiting the game...");
                    break;
                }

                // Any other invalid input - continue the game
                println!("Please input a number between 1 and 100!");
                continue;
            }
        };

        // Displaying the user's guess
        println!("You guessed: {guess}");

        // Comparing the user's guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess higher ;)"),
            Ordering::Greater => println!("Guess lower ;)"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    println!("Thanks for playing the Guessing Game!");
}