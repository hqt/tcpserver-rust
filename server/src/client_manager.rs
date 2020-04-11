use std::collections::HashMap;
use std::net::TcpStream;
use crate::id_generator::IDGenerator;

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