extern crate clap;

use std::io::{Read, Write, stdin, stdout};
use std::net::TcpStream;
use std::thread;
use std::sync::Arc;

use clap::{Arg, App};

use chatroom::DEFAULT_PORT;
use chatroom::protocol::command::Command;
use chatroom::client::parser;

fn main() {
    let (ip, port) = read_program_args();

    println!("The client is initializing...");

    // Connect to the server
    let stream = Arc::new(TcpStream::connect((ip.as_str(), port))
        .expect("connection fails"));

    // setup a thread that handles the response
    let stream_ref = stream.clone();
    thread::spawn(move || {
        handle_server_response(stream_ref.as_ref());
    });

    println!("Welcome to Chatroom!");

    // Start accepting the user input
    let stream_ref = stream.clone();
    handle_user_input(stream_ref.as_ref());
}

fn read_program_args() -> (String, u16) {
    let matches = App::new("Chat Room Client").version("1.0")
        .author("SLMT").about("A client for connecting to a chat room server.")
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

fn handle_server_response(mut stream: &TcpStream) {
    let mut buffer = String::new();
    // Magic! For more details: 
    // https://stackoverflow.com/questions/36233193/why-can-i-just-pass-an-immutable-reference-to-bufreader-instead-of-a-mutable-re
    let stream = &mut stream;

    loop {
        stream.read_to_string(&mut buffer).expect("cannot receive response");
        if buffer.len() > 0 {
            println!("Server: {}", buffer);
            buffer.clear();
        }
    }
}

fn handle_user_input(stream: &TcpStream) {
    let mut input = String::new();
    loop {
        // Show the promot characters
        print!("> ");
        stdout().flush().expect("flush error");

        // Read the user input
        input.clear();
        stdin().read_line(&mut input).expect("stdin error");

        let command = match parser::parse(&input) {
            Ok(c) => c,
            Err(msg) => {
                println!("Error: {}", msg);
                continue;
            }
        };

        if let Command::Help(msg) = command {
            println!("{}", msg);
        } else {
            command.write_to_stream(stream).expect("writing fails");
            dbg!(command);
        }
    }
}