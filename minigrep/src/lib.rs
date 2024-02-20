use std::fs;

pub struct ParseConfig<'a> {
    pub query: &'a str,
    filename: &'a str,
}

impl ParseConfig<'_> {
    pub fn new<'a>(args: &'a [String]) -> Result<ParseConfig<'a>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        Ok(ParseConfig { query, filename })
    }
}

pub fn run(parse_data: ParseConfig) {
    let contents =
        fs::read_to_string(parse_data.filename).expect("Something went wrong reading the file");
    // println!("文件内容:\n{}", contents);
    for line in search(parse_data.query, &contents) {
        println!("当前行内容：{}", line);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
