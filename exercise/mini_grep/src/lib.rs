use std::env;
use std::env::Args;
use std::fs;
use std::io;

pub struct Config {
	pub case_sensitive: bool,
	pub query: String,
	pub filename: String,
}

impl Config {

	/// 没有 &self 的方法相当于静态方法。
	pub fn parse(mut args: Args) -> Result<Config, &'static str> {

		// 参数第一个值是该进程执行文件的路径，需要跳过
		args.next();

		// Args 是个迭代器，调用 next() 会改变状态，所以要 mut。
		let query = match args.next() {
			Some(value) => value,
			None => return Err("啥参数都没有你想干啥"),
		};

		let filename = match args.next() {
			Some(value) => value,
			None => return Err("还缺个文件路径参数哦"),
		};

		// var() 获取环境变量的值，如果不存在则返回 Err，再用 is_err() 转成 bool
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		return Ok(Config { case_sensitive, query, filename });
	}
}

// 这各种包里头的错误咋都叫 Error 啊，重名多麻烦。
pub fn run(config: Config) -> Result<(), io::Error> {
	let contents = fs::read_to_string(&config.filename)?;

	// 【坑】要返回值的话里面的语句不能加分号，这么看 Rust 不能随心所欲地写啊。
	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line);
	}

	Ok(()) // <- 恕我直言，这玩意真的巨 TM 丑，就不能自动推断下吗？
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
