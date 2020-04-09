use std::net::{TcpListener, TcpStream};
use std::{io, thread};
use std::io::{Read, Write};

// Server struct represents the server
pub struct Server {
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            listener: None,
        }
    }

    pub fn start_server(&mut self, addr: &str) -> Result<(), io::Error> {
        let result = TcpListener::bind(addr);
        match result {
            Err(e) => Err(e),
            Ok(listener) => {
                self.listener = Some(listener);
                self.serve();
                Ok(())
            }
        }
    }

    fn serve(&self) {
        for stream in self.listener.as_ref().unwrap().incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("connection error: {}", e);
                }
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("client");
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}
