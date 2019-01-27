extern crate clap;

use std::io::{Write, stdin, stdout};

use clap::{Arg, App};

use chatroom::DEFAULT_PORT;
use chatroom::protocol::command::Command;
use chatroom::client::parser;

fn main() {
    let (ip, port) = read_program_args();

    println!("The client is initializing...");

    // TODO: Connect to the server

    println!("Welcome to Chatroom!");

    let mut input = String::new();
    loop {
        // Show the leading characters
        print!("> ");
        stdout().flush().expect("flush error");

        // TODO: Read input
        input.clear();
        stdin().read_line(&mut input).expect("stdin error");

        dbg!(&input);

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
            // TODO: Send the command
            dbg!(command);
        }

        // TODO: Read socket (maybe in a different thread?)

        // TODO: Show output
    }
}

fn read_program_args() -> (String, u32) {
    let matches = App::new("Chat Room Client").version("1.0")
        .author("SLMT").about("A client for connecting to a chat room server.")
        .arg(Arg::with_name("port").short("p").long("port").value_name("PORT")
            .help("The port number of the server").takes_value(true))
        .arg(Arg::with_name("ip").value_name("IP").required(true)
            .help("The ip address of the server").takes_value(true))
        .get_matches();
    
    let ip = matches.value_of("ip").expect("unexpected error").to_string();
    let port = if matches.is_present("port") {
        u32::from_str_radix(matches.value_of("port").unwrap(), 10).unwrap()
    } else {
        DEFAULT_PORT
    };

    (ip, port)
}
