use std::io::{stdin,Read,BufReader,BufRead};
use rustc_serialize::json;
use std::convert::Into;
use gfx::*;
use utils::*;
use world::*;

// #[derive(RustcDecodable, RustcEncodable, Debug)]
// struct CharacterTemplate {
//     name: String,
//     age: u32,
// }

pub fn main() {

    let example =

        TileInfo {
            name: "window".into(),
            wall: true,
            opaque: false,
        };

        // SpriteInfo {
        //     texture: "gfx/all_tiles.json".into(),
        //     position: (0,0),
        //     size: (1,1),
        //     tile: None,
        // };

        // SpriteSetOptions {
        //     texture: "gfx/all_tiles.png".into(),
        //     size: (1, 1),
        //     position: vec![0, 0],
        //     tile: Some((64, 64)),
        //     ..Default::default()
        // };

    println!("{:?}", Vec2::new(1,2) * Vec2::new(3,4));

    println!("Hello JSON!");
    println!("Example input:\n{}",
        json::encode(
            // &CharacterTemplate{name: "Rusty".into(), age: 1}
            &example
        ).ok().expect("Unexpected error"));
    let mut reader = BufReader::new(stdin());
    loop {
        println!("Input JSON or 'exit':");
        let mut buf = String::new();
        reader.read_line(&mut buf).ok().expect("Input error");
        buf = buf.trim().into();
        if buf == "exit" { break; }
        let maybe_decoded: Option<TileInfo> =
                json::decode(&buf).ok();
        match maybe_decoded {
            Some(s) => println!("Successfully decoded JSON:\n{:?}", s
                /* .validate() */
            ),
            None => println!("Invalid JSON. Try again!"),
        }
    }
}
