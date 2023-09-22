use std::io::Bytes;
use std::sync::Mutex;

#[derive(PartialEq, Clone)]
pub struct NetAddr {
    addr: String,
}

impl NetAddr {
    pub fn new(addr: &str) -> NetAddr {
        NetAddr {
            addr: addr.to_string()
        }
    }
}

pub struct RPC {
    from: NetAddr,
    paylod: Bytes<usize>,
}

pub enum Transport<'a> {
    LocalTransport { aadr: NetAddr, consume_channel: Mutex<Vec<String>>, peers: Mutex<Vec<&'a Transport<'a>>>, lock: String }
}

impl<'a> Transport<'a> {
    pub fn connect(&'a self, transport: &'a Transport<'a>) {
        match self {
            Transport::LocalTransport { peers, .. } => {
                peers.lock().unwrap().push(transport);
                println!("Hello")
            }
        }
    }
    pub fn send_message(&self, to: &Transport, message: String) {
        match self {
            Transport::LocalTransport { .. } => {
                to.receive_message(message)
            }
        }
    }
    pub fn receive_message(&self, message: String) {
        match self {
            Transport::LocalTransport { consume_channel, .. } => {
                consume_channel.lock().unwrap().push(message);
            }
        }
    }
    pub fn consume(&self) -> &Mutex<Vec<String>> {
        match self {
            Transport::LocalTransport { consume_channel, .. } => {
                consume_channel
            }
        }
    }
}

//TODO: messages not eorking with bytes.
