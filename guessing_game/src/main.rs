use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("input your guess");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=10);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {}", guess);
    println!("the number is: {}", secret_number);
}
