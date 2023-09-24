mod network;

use std::sync::{Arc, Mutex};
use crate::network::transport::{NetAddr, Transport};
use crate::network::server::{Server, ServerOptions};


fn main() {

    let (tx, rx) = crossbeam_channel::bounded(1024);

    let mut transport1 = Arc::new(Transport::LocalTransport {
        aadr: NetAddr::new("teste"),
        consume_channel: tx.clone(),
        lock: String::from(""),
    });

    let mut transport2 = Arc::new(Transport::LocalTransport {
        aadr: NetAddr::new("teste3"),
        consume_channel: tx,
        lock: String::from(""),
    });

    transport1.send_message(&transport2, String::from("teste"));

    let mut server: Server = Server::new_server(ServerOptions::new_options(), rx);
    server.add_transport(transport1);
    server.add_transport(transport2);

    server.start();
}


// serer
// Transport layer
// blocks and trx