use crate::Transport;
use std::thread;

pub struct Server<'a> {
    server_options: ServerOptions<'a>,
    message_channel: Vec<String>,
}

pub struct ServerOptions<'b> {
    transports: Vec<Transport<'b>>,
}

impl<'b> ServerOptions<'b> {
    pub fn new_options() -> ServerOptions<'b> {
        ServerOptions {
            transports: Vec::new()
        }
    }
}

impl<'a> Server<'a> {
    pub fn new_server(opts: ServerOptions<'a>) -> Server {
        Server {
            server_options: opts,
            message_channel: Vec::new(),
        }
    }

    pub fn add_transport(&mut self, transport: Transport<'a>) {
        self.server_options.transports.push(transport)
    }

    pub fn init_transports(&'static mut self) {
        let t = thread::spawn(|| {
            for transport in &self.server_options.transports {
                for message in transport.consume().lock().unwrap().iter() {
                    self.message_channel.push(message.clone());
                }
            }
        });

        t.join();
    }

    pub fn start(&mut self) {
        self.init_transports();

        loop {
            if !self.message_channel.is_empty() {
                println!("{}", self.message_channel.len())
            }
        }
    }
}
