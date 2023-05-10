use rand::Rng;
use std::io::{stdin,stdout,Write};

const HIGHEST: i32 = 10;
const LOWEST: i32 = 1;
const MAX_ATTEMPTS: i32 = 10;

pub fn run() {
    let mut rng = rand::thread_rng();
    let number_to_guess = rng.gen_range(LOWEST..=HIGHEST);
    let mut attempts_remaining = MAX_ATTEMPTS;

    println!("Guess the number between {} and {}.", LOWEST, HIGHEST);

    loop {
        println!("You have {} attempt(s) remaining. \n", attempts_remaining);

        let mut input = String::new();
        print!("Enter your guess: ");
        let _ = stdout().flush();

        stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(guess) => {
                if guess < LOWEST || guess > HIGHEST {
                    println!("Your guess is not within the range {} to {} \n.", LOWEST, HIGHEST);
                } else if guess == number_to_guess {
                    println!("Congratulations! You guessed the number: {} \n", number_to_guess);
                    break;
                } else {
                    println!("Your guess is {} than the number. \n", if guess < number_to_guess {"lower"} else {"higher"});
                    attempts_remaining -= 1;
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number between {} and {}.", LOWEST, HIGHEST);
            }
        }

        if attempts_remaining == 0 {
            println!("Game over. The number was {}.", number_to_guess);
            break;
        }
    }
}
