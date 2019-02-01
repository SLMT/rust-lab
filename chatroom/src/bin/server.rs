extern crate clap;

use clap::{Arg, App};

use chatroom::DEFAULT_PORT;

fn main() {
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

    chatroom::server::run_server(ip.as_str(), port);
}
