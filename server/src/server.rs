use std::sync::atomic::AtomicUsize;
use std::net::{TcpListener, TcpStream};
use std::{io, thread};
use std::io::{Read, Write};
use crate::client_manager::ClientManager;
use std::sync::Arc;
use std::sync::atomic::Ordering::SeqCst;

static test_port: AtomicUsize = AtomicUsize::new(1);

// Server struct represents the server
pub struct Server {
    client_managers: Arc<ClientManager>,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            client_managers: Arc::new(ClientManager::new()),
            listener: None,
        }
    }

    // pub fn start_test_server() {
    //     let addr = format!("127.0.0.1:{}", test_port.fetch_add(1, SeqCst));
    //     thread::spawn(move || {
    //     });
    // }

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
                    let client_manager = self.client_managers.clone();
                    thread::spawn(move || {
                        handle_client(client_manager, stream)
                    });
                }
                Err(e) => {
                    println!("connection error: {}", e);
                }
            }
        }
    }
}

fn handle_client(client_manager: Arc<ClientManager>, mut stream: TcpStream) {
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
