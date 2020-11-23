use std::env;
use std::env::Args;
use std::error::Error;
use std::fs;

pub struct Config {
	pub case_sensitive: bool,
	pub query: String,
	pub filename: String,
}

impl Config {

	/// 没有 &self 的方法相当于静态方法
	pub fn parse(mut args: Args) -> Result<Config, &'static str> {

		// 参数第一个值是该进程执行文件的路径，需要跳过
		args.next();

		// args.next 参数是 &mut self，故该方法参数要指定为 mut
		let query = match args.next() {
			Some(value) => value,
			None => return Err("啥参数都没有你想干啥"),
		};

		let filename = match args.next() {
			Some(value) => value,
			None => return Err("还缺个文件路径参数哦"),
		};

		// 获取环境变量的值，如果不存在则返回 Err
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

	Ok(()) // <- 恕我直言，这玩意真的巨TM丑
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	return contents.lines()
		.filter(|line| line.contains(query))
		.collect();
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = &query.to_lowercase(); // 这里要加&？
	return contents.lines()
		.filter(|line| line.to_lowercase().contains(query))
		.collect();
}

// [cfg(test)] 标识这个模块只在 test 构建时编译
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