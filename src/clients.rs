use std::collections::HashMap;
use std::sync::Mutex;
use std::net::TcpStream;

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<String, TcpStream>> = Mutex::new(HashMap::new());
}