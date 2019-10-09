#[macro_use]
extern crate lazy_static;

use events::{Events, Logger};
use server::Server;
use std::io::{Read, Write};
use std::net::Shutdown;
use transaction::Transaction;
use std::thread;
use std::sync::{Arc, Mutex};

mod clients;
mod events;
mod server;
mod transaction;

fn main() {
    let server = Server::new("0.0.0.0", 8000).bind();
    let _ = server.set_ttl(5);
    let genesis_transaction_id = Arc::new(Mutex::new(0));

    for stream in server.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(mut stream) => {
                let transaction_id = Arc::clone(&genesis_transaction_id);
                thread::spawn(move || {
                    Logger.on_connect(&stream);
                    let mut data = [0 as u8; 1024];
                    match stream.read(&mut data) {
                        Ok(size) => {
                            let mut transaction_id = transaction_id.lock().unwrap();
                            *transaction_id += 1;
                            Transaction {
                                id: *transaction_id,
                                from: &stream.peer_addr().unwrap(),
                                to: "127.0.0.1",
                                event_type: "on_connect",
                            };
                            println!("{}", transaction_id);
                            drop(transaction_id);
                            stream.write(&data[0..size]).unwrap();
                        }
                        Err(_) => {
                            println!(
                                "An error occurred, terminating connection with {}",
                                stream.peer_addr().unwrap()
                            );
                            stream.shutdown(Shutdown::Both).unwrap();
                        }
                    }   
                    Logger.on_disconnect(&stream);
                });
            }
        }
    }
}
