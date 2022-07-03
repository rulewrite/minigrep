use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("인자 파싱에 문제가 발생했습니다: {}", err);
        process::exit(1);
    });

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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("인자가 충분하지 않습니다.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
