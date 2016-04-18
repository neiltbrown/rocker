extern crate unix_socket;

use unix_socket::UnixStream;
use std::io::prelude::*;

fn main() {

    let mut stream = UnixStream::connect("/var/run/docker.sock").unwrap();
    stream.write_all(b"GET http:/containers/json HTTP/1.0\r\n\r\n").unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);
}
