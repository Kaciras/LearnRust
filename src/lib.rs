use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
	pub case_sensitive: bool,
	pub query: String,
	pub filename: String,
}

impl Config {
	pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {

		// 在match里的return返回的是函数返回值而不是匹配结果
		match args.len() {
			1 => {
				return Err("啥参数都没有你想干啥");
			}
			2 => {
				return Err("还缺个文件路径参数哦");
			}
			_ => (), // match 必须穷尽，也就是得写这个蛋疼玩意
		};

		let query = args[1].clone();
		let filename = args[2].clone();
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		return Ok(Config { case_sensitive, query, filename });
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(&config.filename)?;

	// 【坑】要返回值的化里面的语句不能加分号
	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line);
	}

	Ok(()) // 巨TM丑
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
	return results;
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}
	return results;
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}