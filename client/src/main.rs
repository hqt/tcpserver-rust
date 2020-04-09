use std::net::TcpStream;
use std::io::{Write, Read};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    let _ = stream.write(&[1]); // ignore the Result
    let _ = stream.read(&mut [0; 128]); // ignore this too
}
