use std::env;
use std::process;

use bb_rust_grep::Config;

fn main() {
    //this will panic on invalid Unicode; used here for simplicity
    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = bb_rust_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
