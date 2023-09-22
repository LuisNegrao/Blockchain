use std::io::Bytes;
use std::sync::{Arc, Mutex};

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

pub type TransportRef = Arc<Transport>;


pub enum Transport {
    LocalTransport {
        // Sending to myself
        aadr: NetAddr,
        consume_channel: crossbeam_channel::Sender<String>,
        lock: String,
    },
    TcpTransport {
        sender_channel: crossbeam_channel::Sender<String>
    },
}

impl Transport {

    pub fn send_message(&self, message: String) {
        match self {
            Transport::LocalTransport { .. } => {
                self.receive_message(message)
            }
            Transport::TcpTransport { .. } => {
                //TCP Server, serialize message, send through TCP
            }
        }
    }

    /// Only called from TCP layer or when we are sending to ourselves
    pub fn receive_message(&self, message: String) {
        match self {
            Transport::LocalTransport { consume_channel, .. } => {
                consume_channel.send(message).unwrap();
            }
            Transport::TcpTransport { sender_channel} => {
                // Already deserialized
                sender_channel.send(message).unwrap();
            }
        }
    }
}

//TODO: messages not eorking with bytes.
