use std::env;
use std::process;

use minigrep_rulewrite::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("인자 파싱에 문제가 발생했습니다: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_rulewrite::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
