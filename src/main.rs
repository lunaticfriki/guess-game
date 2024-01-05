use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("GUESS THE NUMBER!");
    println!("THE SECRET NUMBER IS: {}", secret_number);

    loop {
        println!("PLEASE, INPUT YOUR GUESS: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("!!! FAILED TO READ LINE !!!");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("!!! INVALID INPUT, PLEASE TYPE A NUMBER  !!!");
                continue;
            }
        };

        println!("YOU GUESSED: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "TOO SMALL!".red()),
            Ordering::Greater => println!("{}", "TOO BIG!".red()),
            Ordering::Equal => {
                println!("{}", "YOU WIN!".green());
                break;
            }
        }
    }
}
