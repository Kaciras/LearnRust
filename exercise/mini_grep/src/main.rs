use std::{env, process};

use mini_grep;
use mini_grep::Config;

fn main() {
    let config = mini_grep::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = mini_grep::run(config) {
        eprintln!("{}", e);
        process::exit(2);
    }
}
