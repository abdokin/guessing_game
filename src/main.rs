use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the Number ! in range [1..100]");
    let secret_number = rand::thread_rng().gen_range(1..=1000);
    let mut guess = String::new();

    loop {
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }

    println!("You guessed: {secret_number}");
}
