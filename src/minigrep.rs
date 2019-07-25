use std::{env, process};

use experiment;
use experiment::Config;

fn main() {
	let args: Vec<String> = env::args().collect();

	let config = Config::new(&args).unwrap_or_else(|err| {
		eprintln!("{}", err);
		process::exit(1);
	});

	if let Err(e) = experiment::run(config) {
		eprintln!("{}", e);
		process::exit(2);
	}
}