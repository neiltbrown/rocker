#![allow(non_snake_case)]
extern crate rustc_serialize;
use rustc_serialize::json;
use std::collections::BTreeMap;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Container {
    pub Id: String,
    pub Names: Vec<String>,
    pub Image: String,
    pub Command: String,
    pub Created: u64,
    pub Status: String,
    pub Ports: Vec<Port>,
    pub Labels: BTreeMap<String, String>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Port {
    pub PrivatePort: u16,
    pub PublicPort: u16,
    pub Type: String,
}
