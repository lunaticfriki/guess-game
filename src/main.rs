use rand::Rng;
use std::cmp::Ordering;
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

    let guess: u32 = guess.trim().parse().expect("!!! PLEASE TYPE A NUMBER !!!");

    println!("YOU GUESSED: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("TOO SMALL!"),
        Ordering::Greater => println!("TOO BIG!"),
        Ordering::Equal => println!("YOU WIN!"),
    }
}
