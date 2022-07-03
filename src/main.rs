use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("Searching for {:?}", query);
    println!("In file {:?}", filename);

    let contents = fs::read_to_string(filename).expect("파일을 읽는 동안 문제가 발생했습니다.");
    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
