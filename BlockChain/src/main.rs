mod network;

use std::sync::Mutex;
use crate::network::transport::{NetAddr, Transport};
use crate::network::server::{Server, ServerOptions};


fn main() {
    let mut transport1 = Transport::LocalTransport {
        aadr: NetAddr::new("teste"),
        consume_channel: Mutex::new(vec![]),
        peers: Mutex::new(vec![]),
        lock: String::from(""),
    };

    let mut transport2 = Transport::LocalTransport {
        aadr: NetAddr::new("teste3"),
        consume_channel: Mutex::new(vec![]),
        peers: Mutex::new(vec![]),
        lock: String::from(""),
    };

    transport1.connect(&transport2);
    transport1.send_message(&transport2, String::from("teste"));

    let mut server: Server = Server::new_server(ServerOptions::new_options());
    server.add_transport(transport1);
    server.add_transport(transport2);

    server.start();
}


// serer
// Transport layer
// blocks and trx