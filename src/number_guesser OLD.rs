use rand::Rng;

const HIGHEST: i32 = 10;
const LOWEST: i32 = 1;


pub fn run() {
    
    // Hold the input string outside of the loop so we can access it
    let mut input = String::new();
    let mut attempts: i32 = 10;
    const HIGHEST: i32 = 10;
    const LOWEST: i32 = 1;

    let mut rng = rand::thread_rng();
    let guess = rng.gen_range(LOWEST..=HIGHEST);
    // Beginning of loop
    loop {
        // check if we have attempts
        if attempts == 0{
            break;
        }

        println!("Type your guess, the number range is {low} and {high}, and you also have {attempt} attempts.",low = LOWEST,high= HIGHEST, attempt = attempts);
        // Ask for input from the user
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Check if its valid input an i32, an INT
        let num = input.trim().parse::<i32>();
        match num {
            Ok(n) => {
                println!("You guessed: {}\n", n);
                if n >= LOWEST && n <= HIGHEST{
                    attempt_guess(n,guess);
                    attempts -= 1
                } else {
                    println!("That was not in range of the numbers {} and {} \n",LOWEST,HIGHEST)
                };
            },
            Err(e) => {
                println!("Error: {}", e);
            },
        }
        //Clear the input of previous strings
        input.clear();
    }
    // Outside of loop
}

fn attempt_guess(guess: i32, number: i32) {
    if guess == number{
        println!("You guessed the number!: {}", number);
        std::process::exit(0)
    } else if guess > number {
        println!("Lower");
    } else {
        println!("Higher");
    }
}