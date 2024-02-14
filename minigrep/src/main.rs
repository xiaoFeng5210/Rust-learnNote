use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let parse_data = ParseConfig::new(&args).unwrap_or_else( |err| {
        println!("参数个数不足: {}", err);
        std::process::exit(1);
    });
    println!("用户输入的参数 {}", parse_data.query);
    run(parse_data);
}

fn run(parse_data: ParseConfig) {
    let contents = fs::read_to_string(parse_data.filename).expect("Something went wrong reading the file");
    println!("文件内容:\n{}", contents);
}

struct ParseConfig<'a> {
    query: &'a str,
    filename: &'a str,
}

impl ParseConfig<'_> {
    fn new<'a>(args: &'a [String]) -> Result<ParseConfig<'a>, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        Ok(ParseConfig { query, filename })
    }
}
