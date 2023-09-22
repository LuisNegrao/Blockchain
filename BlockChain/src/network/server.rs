use std::sync::Mutex;
use crossbeam_channel::Receiver;

use crate::network::transport::TransportRef;

pub struct Server {
    server_options: ServerOptions,
    message_channel: crossbeam_channel::Receiver<String>,
    peers: Mutex<Vec<TransportRef>>,
}

pub struct ServerOptions {
    transports: Vec<TransportRef>,
}

impl ServerOptions {
    pub fn new_options() -> ServerOptions {
        ServerOptions {
            transports: Vec::new()
        }
    }
}

impl Server {
    pub fn connect(&self, transport: TransportRef) {
        self.peers.lock().unwrap().push(transport);
        println!("Hello")
    }

    pub fn new_server(opts: ServerOptions, receiver: Receiver<String>) -> Server {
        Server {
            server_options: opts,
            message_channel: receiver,
            peers: Mutex::new(vec![]),
        }
    }

    fn send_message(&self, transport: TransportRef, message: String) {
        transport.send_message(message);
    }

    pub fn add_transport(&mut self, transport: TransportRef) {
        self.server_options.transports.push(transport)
    }

    pub fn start(&mut self) {
        loop {
            if let Ok(msg) = self.message_channel.recv() {}
        }
    }
}
