// use ferris_says::say;
use rand::Rng;
use std::io;
use hello_rust::slice::slice_test;


#[derive(Debug)]

struct Test {
    id: i32,
    name: String,
    address: String,
}

fn main() {
    slice_test::test();
}

// 数组
// fn testFindFromArray() {
//     let a = vec![1, 2, 3, 4, 5];
//     let mut index = 0;
//     for i in a {
//         if i == 3 {
//             index = a.iter().position(|&x| -> {x == i});
//             println!("{}", index);
//         }
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Animal {
    dog: String,
    cat: String,
    index: i32,
}

impl Animal {
    fn new(dog: String, cat: String, index: i32) -> Self {
        Animal { dog, cat, index }
    }

    fn getDog(&self) -> &str {
        self.dog.as_str()
    }
}

fn print_struct() {
    let mut test_data = Test {
        id: 1,
        name: String::from("xiaofeng"),
        address: String::from("徐州春雨花园"),
    };
    test_data.id = 2;
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
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
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
