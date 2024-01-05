use std::io;

fn main() {
    println!("GUESS THE NUMBER!");
    println!("PLEASE, INPUT YOUR GUESS: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("!!! FAILED TO READ LINE !!!");

    println!("YOU GUESSED: {}", guess)
}
