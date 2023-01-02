use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("Seceret_number {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = guess.trim().parse().expect("Please enter number");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

}
