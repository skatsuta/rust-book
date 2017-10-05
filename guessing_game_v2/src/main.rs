extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!(
                "Guess value must be between 1 and 100, got {}.",
                value
            ));
        }

        Ok(Guess { value })
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess (between 1 and 100):");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(
            "Failed to read line",
        );

        let guess = match guess.trim().parse() {
            Ok(num) => {
                match Guess::new(num) {
                    Ok(guess) => guess,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                }
            }
            Err(err) => {
                println!("Failed to parse input: {}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
