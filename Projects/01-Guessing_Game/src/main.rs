use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};

fn main() {
    println!("Guess a number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error on during ");

        println!("you guessed {}", guess);

        let guess_number: i16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Wow! correct");
                break;
            }
        }
    }
}
