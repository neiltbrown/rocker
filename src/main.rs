#![feature(unix_socket)]
extern crate rustc_serialize;

mod http;
mod docker;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use http::request::Request;
use http::request::Method;
use rustc_serialize::json;


fn main() {

    let req = Request {
        method: Method::Get,
        path: "/containers/json".to_string(),
    };
    let mut stream = UnixStream::connect("/var/run/docker.sock").unwrap();
    stream.write_all(req.build().as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let mut parts: Vec<&str> = response.split("\r\n").collect();
    parts
        .pop()
        .map(|x|
             println!("{}",x)
        );

}
