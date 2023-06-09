use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem while parsing args: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Something went wrong: {err}");
        process::exit(1);
    };
}
