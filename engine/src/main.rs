#[macro_use]
extern crate lazy_static;

pub mod core;
pub mod engine;
pub mod ugmi;

use crate::ugmi::client::Client;

fn main() -> std::io::Result<()> {
    let client = Client::new();

    client.run()
}
