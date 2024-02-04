// use ferris_says::say;
use std::io;
use rand::Rng;

fn main() {
    guess_number();
}

fn guess_number() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("you guessed: {}", guess)
}

// for循环
fn for_loop() {
    let mut x = 200;
    
}
