use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("파일을 읽는 동안 문제가 발생했습니다.");
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("인자가 충분하지 않습니다.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
