use std::net::TcpListener;

pub struct Server {
    address: String,
    port: usize,
}

impl Server {
    pub fn new(address: &str, port: usize) -> Self {
        Self {
            address: address.to_owned(),
            port,
        }
    }

    pub fn bind(&self) -> TcpListener {
        println!("Listening on {}:{}", self.address, self.port);
        TcpListener::bind(format!("{}:{}", self.address, self.port)).expect("Could not bind")
    }
}
