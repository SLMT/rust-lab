extern crate clap;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;

use clap::{Arg, App};

use chatroom::DEFAULT_PORT;
use chatroom::protocol::command::Command;

fn main() {
    let (ip, port) = read_program_args();

    println!("The server is initializing...");

    // Listen on a ip and a port
    let listener = TcpListener::bind((ip.as_str(), port))
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

fn read_program_args() -> (String, u16) {
    let matches = App::new("Chat Room Server").version("1.0")
        .author("SLMT").about("A server for managing a chatroom service")
        .arg(Arg::with_name("port").short("p").long("port").value_name("PORT")
            .help("The port number of the server").takes_value(true))
        .arg(Arg::with_name("ip").value_name("IP").required(true)
            .help("The ip address of the server").takes_value(true))
        .get_matches();
    
    let ip = matches.value_of("ip").expect("unexpected error").to_string();
    let port = if matches.is_present("port") {
        u16::from_str_radix(matches.value_of("port").unwrap(), 10).unwrap()
    } else {
        DEFAULT_PORT
    };

    (ip, port)
}

fn handle_client_connection(stream: TcpStream) {
    let mut read_buf: Vec<u8> = Vec::new();

    loop {
        let command = Command::read_from_stream(&stream, &mut read_buf)
        .expect("fails to read a command");
        dbg!(command);
    }
}