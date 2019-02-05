use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let input_handle = io::stdin();
    input_handle.lock();

    loop {
        println!("Please input the number.");

        let mut guess = String::new();
        input_handle
            .read_line(&mut guess)
            .expect("Failed read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Not a number try again!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Your win!");
                break;
            }
        }
    }
}
