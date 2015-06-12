use std::collections::HashMap;
use std::fs::File;
use std::io::Result as IoResult;
use std::io::Read;
use rustc_serialize::json;
use world::*;

const NUM_TESTS: u32 = 1;

pub fn test(offset: u32) -> IoResult<Dungeon> {

    let mut level_str = String::new();
    let mut tile_str = String::new();
    let mut level_json = File::open("./res/dat/level.json").unwrap();
    let mut tile_json = File::open("./res/dat/tiles.json").unwrap();
    level_json.read_to_string(&mut level_str).unwrap();
    tile_json.read_to_string(&mut tile_str).unwrap();
    let mut level_params: DungeonParams = json::decode(&level_str).unwrap();
    level_params.seed += offset;
    let tile_vec: Vec<TileInfo> = json::decode(&tile_str).unwrap();
    let tile_map: HashMap<String, TileInfo> = tile_vec.iter().cloned()
            .map(|t| (t.name.clone(), t)).collect();
    println!("Generating...");
    let mut d = generate(level_params, tile_map);
    println!("...Done.");
    Ok(d)
}

pub fn main() {
    for i in 0..NUM_TESTS {
        println!("Running test {} of {}...", i+1, NUM_TESTS);
        test(i).unwrap().print();
    }
}