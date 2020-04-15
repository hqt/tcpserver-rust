use std::sync::atomic::AtomicUsize;
use std::net::{TcpListener, TcpStream};
use std::{io, thread};
use std::io::{Read, Write};
use crate::client_manager::ClientManager;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::Ordering::SeqCst;
use std::borrow::Borrow;

static TEST_PORT: AtomicUsize = AtomicUsize::new(20_000);

// Server struct represents the server
pub struct Server {
    core: Arc<ServerCore>,
    listener: Option<TcpListener>,
}

struct ServerCore {
    client_managers: ClientManager,
}

impl Server {
    pub fn new() -> Server {
        Server {
            core: Arc::new(ServerCore {
                client_managers: ClientManager::new(),
            }),
            listener: None,
        }
    }

    pub fn start_test_server(&mut self) -> Result<(String), io::Error> {
        let addr = format!("127.0.0.1:{}", TEST_PORT.fetch_add(1, SeqCst));
        let result = TcpListener::bind(&addr);
        match result {
            Err(e) => Err(e),
            Ok(listener) => {
                let server = self.core.clone();
                thread::spawn(move || {
                    serve(&listener, server);
                });
                Ok(addr)
            }
        }
    }

    pub fn start_server(&mut self, addr: &str) -> Result<(), io::Error> {
        return self.serve(addr);
    }

    fn serve(&mut self, addr: &str) -> Result<(), io::Error> {
        let result = TcpListener::bind(addr);
        match result {
            Err(e) => Err(e),
            Ok(listener) => {
                let server = self.core.clone();
                serve(&listener, server);
                Ok(())
            }
        }
    }
}

fn serve(listener: &TcpListener, server: Arc<ServerCore>) {
    for stream in listener.incoming() {
        let wrapped = server.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(wrapped, stream)
                });
            }
            Err(e) => {
                println!("connection error: {}", e);
            }
        }
    }
}

fn handle_client(server: Arc<ServerCore>, mut stream: TcpStream) {
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
