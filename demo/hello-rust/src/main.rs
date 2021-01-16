// 从标准库中引入输入/输出功能
use std::io;

fn main() {
    println!("Hello, world!");

    println!("Guess the number!");
    let mut _guess = String::new(); // mut引入一个可变的变量

    println!("Please input your guess.");
    io::stdin().read_line(&mut _guess).expect("Failed to line"); // &符号表示这是一个引用， &mut表示这是一个可变的引用

    println!("You guessed: {}", _guess);
}


