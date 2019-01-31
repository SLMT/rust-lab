#[macro_use]
extern crate serde_derive;
extern crate bincode;

pub mod protocol;
pub mod server;
pub mod client;

pub const DEFAULT_PORT: u16 = 54321;