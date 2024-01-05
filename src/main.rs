use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("GUESS THE NUMBER!");
    println!("THE SECRET NUMBER IS: {}", secret_number);
    println!("PLEASE, INPUT YOUR GUESS: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("!!! FAILED TO READ LINE !!!");

    println!("YOU GUESSED: {}", guess);
}
