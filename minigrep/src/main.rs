use std::env;
use minigrep::ParseConfig;



fn main() {
    let args: Vec<String> = env::args().collect();
    let parse_data = ParseConfig::new(&args).unwrap_or_else(|err| {
        println!("参数个数不足: {}", err);
        std::process::exit(1);
    });
    println!("用户输入的参数 {}", parse_data.query);
    minigrep::run(parse_data);
}
