use std::net::{TcpListener, TcpStream};
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
	stream.write(b"<html><body><h1>Hello World!</h1></body></html>");
}

fn main() {
    	println!("Hello, world!");

	let listener = TcpListener::bind("138.197.222.129:80").unwrap();
	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
	    match stream {
		Ok(stream) => { handle_client(stream); }
		Err(e) => { /* connection failed */ }
	    }
	}
}
