use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("错误 {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        println!("错误2: {}", e)
    }
}
