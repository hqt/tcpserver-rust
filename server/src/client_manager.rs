use std::collections::HashMap;
use std::net::TcpStream;
use crate::id_generator::IDGenerator;
use std::io::{Write, Read};

pub struct ClientManager {
    id_generator: IDGenerator,
    clients: HashMap<usize, TcpStream>,
}

impl ClientManager {
    pub fn new() -> ClientManager {
        ClientManager {
            id_generator: IDGenerator::new(),
            clients: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, conn: TcpStream) {
        let id = self.id_generator.next_id();
        self.clients.insert(id, conn);
    }
}

#[cfg(test)]
mod client_manager_test {
    use crate::id_generator::IDGenerator;
    use crate::server;
    use std::net::TcpStream;
    use std::io::{Write, Read};

    #[test]
    fn add_client() {
        let mut server = server::Server::new();
        let res = server.start_test_server();
        assert_eq!(false, res.is_err());

        let result = TcpStream::connect(res.unwrap());
        assert_eq!(false, result.is_err());

        let mut stream = result.unwrap();
        let _ = stream.write(&[1]); // ignore the Result
        let _ = stream.read(&mut [0; 128]); // ignore this too
        println!("done")
    }
}
