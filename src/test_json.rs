use std::io::{stdin,Read,BufReader,BufRead};
use rustc_serialize::json;
use std::convert::Into;

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct CharacterTemplate {
    name: String,
    age: u32,
}

pub fn main() {
    println!("Hello JSON!");
    println!("Example input:\n{}",
        json::encode(&CharacterTemplate{name: "Rusty".into(), age: 1}).ok()
        .expect("Unexpected error"));
    let mut reader = BufReader::new(stdin());
    loop {
        println!("Input JSON or 'exit':");
        let mut buf = String::new();
        reader.read_line(&mut buf).ok().expect("Input error");
        buf = buf.trim().into();
        if buf == "exit" { break; }
        let maybe_decoded: Option<CharacterTemplate> =
                json::decode(&buf).ok();
        match maybe_decoded {
            Some(s) => println!("Successfully decoded JSON:\n{:?}", s),
            None => println!("Invalid JSON. Try again!"),
        }
    }
}