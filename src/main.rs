mod ribbit_client;

use std::net::{TcpStream};
use std::io::{Read, Write};
use mailparse::*;
use std::option::Option;
use ribbit_client::ribbit_client::{get, Region, FileType};

fn main() {
    let data = match get(Region::US, FileType::BGDL, "pro") {
        Some(x) => x,
        None => String::from("failed to parse data")
    };

    println!("{}", data);
}