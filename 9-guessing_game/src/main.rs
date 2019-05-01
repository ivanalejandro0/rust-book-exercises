use std::cmp::Ordering;
use std::io;
use std::error::Error;

use rand::Rng;
use simple_error::bail;

#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, Box<Error>> {
        if value < 1 || value > 100 {
            bail!("You should use a number between 1 and 100.");
        }

        Ok(Guess {
            value,
        })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("E: unable to parse input, please try again");
                continue;
            },
        };

        let guess = match Guess::new(guess) {
            Ok(guess) => guess,
            Err(err) => { println!("{}", err); continue; }
        };

        println!("You guessed: {:?}", guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
