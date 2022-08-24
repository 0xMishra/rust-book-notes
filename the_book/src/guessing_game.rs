use rand::{self, Rng};
use std::{cmp::Ordering, io};

// pub struct Guess {
//     value: i32,
// }
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if guess < 1 || guess > 100 {
//             println!("secret number will be between 1 and 100");
//             continue;
//         }
//         Guess { value }
//     }
//     pub fn value(&self) -> i32 {
//         self.value
//     }
// }

pub fn guess_the_number() {
    println!("Guess the number! ");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Please enter your guess: ");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number\n ");
                println!("enter your guess again: ");
                continue;
            }
        };
        if guess < 1 || guess > 100 {
            println!("secret number will be between 1 and 100");
            continue;
        } // error handling if user enters a negative number or a number greater than 100

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
