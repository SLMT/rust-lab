
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;

use super::protocol::command::Command;

mod data;

pub fn run_server(ip: &str, port: u16) {
    println!("The server is initializing...");

    // Listen on a ip and a port
    let listener = TcpListener::bind((ip, port))
        .expect("fails to listen on the given ip and port");

    println!("The server is online!");

    // Accept connections
    for stream in listener.incoming() {
        let stream = stream.expect("fails to open a connection");
        println!("Received a connection from {}",
            stream.peer_addr().expect("fails to parse a address."));

        thread::spawn(move || {
            handle_client_connection(stream);
        });
    }
}

fn handle_client_connection(stream: TcpStream) {
    let mut read_buf: Vec<u8> = Vec::new();

    loop {
        let command = Command::read_from_stream(&stream, &mut read_buf)
        .expect("fails to read a command");
        dbg!(command);
    }
}