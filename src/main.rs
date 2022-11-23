extern crate core;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::build(&args).unwrap_or_else(|_err| {
        println!("{}", _err);
        process::exit(1);
    });

    if let Err(_e) = minigrep::run(config) {
        process::exit(1);
    }
}


