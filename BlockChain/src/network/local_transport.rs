/*use std::borrow::{Borrow, BorrowMut};
use std::io::Bytes;
use crate::network::transport::{NetAddr, RPC};
*/
/*pub fn connect(from: Transport, transport: &dyn Transport) -> Result<(), String> {
    from.peers.push((transport));
    Ok(())
}*/

/*fn consume(&mut self, message: Bytes<usize>) -> Result<(), String> {
    self.consume_channel.push(message);
    Ok(())
}

fn send_message(&self, to: Box<dyn Transport>, message: Bytes<usize>) -> Result<(), String> {
    for mut transport in self.peers {
        if transport.addr() == to.addr() {
            transport.consume(message.clone()).unwrap();
            Ok(())
        }
    }
    Err(("Could Not find transport".to_string()))
}

fn addr(&self) -> NetAddr {
    self.addr.clone()
}*/

