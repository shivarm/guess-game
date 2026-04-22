use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    const MAX_ATTEMPTS: u32 = 7;
    const HINT_THRESHOLD: u32 = 3;
    const HINT_RANGE: u32 = 10;
    let mut attempts = 0;

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts += 1;

        println!("You guessed: {guess}");

        // compare guess to secret_number

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win in {attempts} attempts!");
                break;
            }
        }

        // Give a hint after HINT_THRESHOLD attempts
        if attempts >= HINT_THRESHOLD && guess != secret_number {
            let lower = if secret_number > HINT_RANGE { secret_number - HINT_RANGE } else { 1 };
            let upper = if secret_number + HINT_RANGE < 100 { secret_number + HINT_RANGE } else { 100 };
            println!("Hint: The number is between {lower} and {upper}.");
        }

        if attempts > MAX_ATTEMPTS {
            println!("You have used all your attempts! The number was {secret_number} ");
            break;
        }
    }
}
