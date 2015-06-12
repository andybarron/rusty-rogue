use std::cmp::PartialOrd;
use rand::distributions::range::SampleRange;
use num::traits::One;
use num::integer::Integer;
use std::ops::Add;
use std::collections::HashMap;
use rand::{Rng, SeedableRng, XorShiftRng};
use super::tiles::*;
use super::dungeon::*;

// pub fn generate_default() -> Dungeon {
//     generate(Default::default())
// }

pub fn generate(params: DungeonParams, map: TileMap) -> Dungeon {
    let mut d = Dungeon::empty(params, map);
    let p = d.get_params().clone();
    let m = d.get_map().clone();
    let floor = m.get(&p.floor)
            .expect("Error: floor tile could not be found");
    let wall = m.get(&p.wall)
            .expect("Error: wall tile could not be found");
    let seed = [p.seed, p.seed/4+999, p.seed/2+3, p.seed/3+1337];
    let mut rng = XorShiftRng::from_seed(seed);
    let rng = &mut rng;
    let total_rooms = int_range(rng, p.rooms); // rng.gen_range(p.rooms.0, p.rooms.1 + 1);
    println!("total_rooms={}", total_rooms);
    let mut rooms = Vec::new();
    while rooms.len() < total_rooms {
        let room = {
            let w = int_range(rng, p.room_size);
            let h = int_range(rng, p.room_size);
            let x = int_range( rng, (0, d.get_width() - w - 1)  );
            let y = int_range( rng, (0, d.get_height() - h - 1) );
            Room {  w: w, h: h, x: x as isize, y: y as isize }
        };
        if d.fill_room(room, floor) {
            rooms.push(room);
            println!("...{}", rooms.len());
        }
    }
    d
}

/// inclusive
fn int_range<R, T>(rng: &mut R, range: (T, T)) -> T
        where R: Rng, T: PartialOrd + SampleRange + One + Integer
{
    rng.gen_range(range.0, range.1 + T::one())
}