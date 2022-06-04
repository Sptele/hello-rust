use std::io::{stdin};
use rand::Rng;
use std::cmp::Ordering;

fn guess(prompt: &str) -> Option<u32> {
    loop {
        let mut guess: String = String::new();

        println!("{}", prompt);
        stdin().read_line(&mut guess)
            .expect("Error, could not read your input. Try again!");

        if guess.trim() == "quit" {
            return None;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { print!("Please enter a number! "); continue },
        };

        return Some(guess);
    }
}

fn main() {
    println!("Guess the fruit!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = guess("Enter your guess!");

        let guess: u32 = match guess {
            Some(guess) => guess,
            None => { println!("Bye!"); break; }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break; },
        }
    }

}
