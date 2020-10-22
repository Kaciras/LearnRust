mod thread_pool;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};

fn main() {
	let address = "127.0.0.1:7878";
	let listener = TcpListener::bind(address).unwrap();
	let pool = thread_pool::ThreadPool::new(4);

	println!("Server is listening on http://{}", address);

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
		(200, "resources/hello.html")
	} else {
		(404, "resources/404.html")
	};

	stream.write(format!("HTTP/1.1 {} OK\r\n\r\n", status).as_bytes()).unwrap();
	stream.write(fs::read(filename).unwrap().as_slice()).unwrap();
	stream.flush().unwrap();
}
