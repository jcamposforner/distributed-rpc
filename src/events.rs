use std::net::TcpStream;

#[allow(unused_variables)]
pub trait Events {
    fn on_connect(&self, stream: &TcpStream) {}
    fn on_error(&self, err: &str) {}
    fn on_message(&self, stream: &TcpStream) {}
    fn on_disconnect(&self, stream: &TcpStream) {}
}

pub struct Logger;

impl Events for Logger {
    fn on_connect(&self, stream: &TcpStream) {
        println!("Peer connected {}", stream.peer_addr().unwrap());
    }

    fn on_error(&self, err: &str) {
        println!("error: {}", err);
    }

    fn on_disconnect(&self, stream: &TcpStream) {
        println!("Peer disconnect: {:?}", stream.peer_addr().unwrap());
    }
}
