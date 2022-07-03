use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("인자 파싱에 문제가 발생했습니다: {}", err);
        process::exit(1);
    });

    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
