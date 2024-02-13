// use ferris_says::say;
use std::io;
use rand::Rng;
// 引入./所有权/owner_ship.rs 的函数
#[derive(Debug)]

struct Test {
    id: i32,
    name: String,
    address: String
}


fn main() {
    print_struct()
}

fn print_struct() {
    let test_data = Test {
        id: 1,
        name: String::from("xiaofeng"),
        address: String::from("徐州春雨花园")
    };
    println!("{:?}", test_data)
}

fn slice_test() {
    let s = String::from("hello_world");
    let a = &s[..1];
    println!("{}", a);
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

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // 返回s的引用,这里是空指针，因为s在函数结束后会被释放
// }

fn mutableReference() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
    println!("{}", s);
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);                                                                                                                                                                                                                        
    }
    for number in (1..=4).rev() {
        println!("{}!", number);
    }
}

fn if_message() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("number < 0")
}

fn test_loop() {
    let mut counter = 0;
    let result = loop {
        println!("again!");
        counter += 1;
        if counter == 10000 {
            break counter * 2;
        }
    };
    println!("the result is {}", counter);
}

// for循环
// fn for_loop() {

// }
