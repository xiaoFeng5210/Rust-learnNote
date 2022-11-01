// 从标准库中引入输入/输出功能
use std::io;

fn main() {
    // println!("Hello, world!");

    // println!("Guess the number!");
    // let mut _guess = String::new(); // mut引入一个可变的变量

    // println!("Please input your guess.");
    // io::stdin().read_line(&mut _guess).expect("Failed to line"); // &符号表示这是一个引用， &mut表示这是一个可变的引用

    // println!("You guessed: {}", _guess);

    let rect1 = Rectangle { width: 30, height: 50 };
    let param = Rectangle { width: 20, height: 10};
    println!("{:#?}", rect1.can_hold(&param));
    println!("hello world")
    
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}





