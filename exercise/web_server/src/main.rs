use std::{fs, thread};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod thread_pool;

fn main() {
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).unwrap();
    let pool = thread_pool::ThreadPool::new(4);

    println!("Process working directory: {}", env::current_dir().unwrap().display());
    println!("Server is listening on http://{}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 这里 stream 的所有权已经被移入闭包内，外层不可再访问
        pool.execute(|| { handle_connection(stream); });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 8192];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        (200, "assets/hello.html")
    } else {
        (404, "assets/404.html")
    };

    stream.write(format!("HTTP/1.1 {} OK\r\n\r\n", status).as_bytes()).unwrap();
    stream.write(fs::read(filename).unwrap().as_slice()).unwrap();
    stream.flush().unwrap();
}
