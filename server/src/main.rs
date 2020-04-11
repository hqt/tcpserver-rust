mod id_generator;
mod client_manager;

mod server;

fn main() {
    let mut server = server::Server::new();
    let res = server.start_server("127.0.0.1:3000");
    match res {
        Err(e) => panic!(e),
        _ => {}
    }
}
