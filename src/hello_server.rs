mod thread_pool;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = thread_pool::ThreadPool::new(4);

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		pool.execute(|| { handle_connection(stream); });
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 8192];
	stream.read(&mut buffer).unwrap();

	let get = b"GET / HTTP/1.1\r\n";

	let (status, filename) = if buffer.starts_with(get) {
		(200, "src/hello.html")
	} else {
		(404, "src/hello.html") // 我懒得再搞一个404文件
	};

	stream.write(format!("HTTP/1.1 {} OK\r\n\r\n", status).as_bytes()).unwrap();
	stream.write(fs::read(filename).unwrap().as_slice()).unwrap();
	stream.flush().unwrap();
}