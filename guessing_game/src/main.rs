extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

struct Guess {
    value: u32
}

impl Guess {
    fn new(value: u32) -> Result<Self, &'static str> {
        if value < 0 || 100 < value {
            return Err("must be between 0 and 100");
        }

        Ok(Guess { value })
    }

    fn from(value: &str) -> Result<Self, &str> {
        if let Ok(value) = value.trim().parse::<i32>() {
            Ok(Guess::new(value as u32)?)
        } else {
            Err("not a number")
        }
    }

    fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut attempts = 0;

    loop {

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match Guess::from(&guess) {
            Ok(guess) => guess.value(),
            Err(err) => {
                println!("Bad value: {}", err);
                continue;
            }
        };

        attempts += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

    println!("The secret number was: {}", secret_number);
    println!("You made {} attempt{}.", attempts, if attempts > 1 { "s" } else { "" });
}
