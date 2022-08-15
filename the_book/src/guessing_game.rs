use rand::{self, Rng};
use std::{cmp::Ordering, io};

pub fn guess_the_number() {
    println!("Guess the number! ");
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);
    println!("Please enter your guess: ");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number\n ");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too high!\n "),
            Ordering::Less => println!("Too low!\n "),
            Ordering::Equal => {
                println!("You guessed it, {} is the right answer", guess);
                break;
            }
        }
        println!("enter your guess again: ");
    }
}
