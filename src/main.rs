use std::env;
use std::process;

use bb_rust_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = bb_rust_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
