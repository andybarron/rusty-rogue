// TODO this is ugly as sin... refactor!

use crate::util::map_range_f32;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::vec::Vec;

#[derive(Clone, Copy)]
pub struct Tile {
    pub x: isize,
    pub y: isize,
    pub t: TileType,
    pub e: Option<Entity>,
}

#[derive(Clone, Copy)]
pub enum Entity {
    Monster(usize),
    // Treasure,
    // Key,
    // Missingno,
}
pub use self::Entity::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Corridor,
    StairsUp,
    StairsDown,
    // Unknown,
}
pub use self::TileType::*;

#[derive(Clone)]
pub struct Dungeon {
    pub width: isize,
    pub height: isize,
    pub tiles: Vec<Tile>,
    path_length: usize,
    pub start_coords: (isize, isize),
    pub end_coords: (isize, isize),
}

#[derive(Clone, Copy)]
pub struct DungeonParams {
    room_count: isize,
    room_size_min: isize,
    room_size_max: isize,
    hall_width_min: isize,
    hall_width_max: isize,
    hall_length_min: isize,
    hall_length_max: isize,
    room_monsters_max: isize,
    hall_monsters_max: isize,
    hall_chance: f32,
    map_width: isize,
    map_height: isize,
}

/********************/
/* public functions */
/********************/

impl Dungeon {
    pub fn empty(w: isize, h: isize) -> Dungeon {
        Dungeon {
            width: w,
            height: h,
            tiles: (0..((w * h) as usize))
                .map(|i: usize| {
                    let i = i as isize;
                    let x: isize = i % w;
                    let y: isize = i / w;
                    Tile {
                        x,
                        y,
                        t: Wall,
                        e: None,
                    }
                })
                .collect(),
            path_length: 0,
            start_coords: (0, 0),
            end_coords: (0, 0),
        }
    }

    pub fn get_tile<'a>(&'a self, x: isize, y: isize) -> Option<&'a Tile> {
        let idx = x + y * self.width;

        if x >= self.width
            || y >= self.height
            || x < 0
            || y < 0
            || idx < 0
            || idx >= self.tiles.len() as isize
        {
            None
        } else {
            Some(&self.tiles[idx as usize])
        }
    }

    pub fn get_tile_type(&self, x: isize, y: isize) -> Option<TileType> {
        let idx = x + y * self.width;

        if x >= self.width
            || y >= self.height
            || x < 0
            || y < 0
            || idx < 0
            || idx >= self.tiles.len() as isize
        {
            None
        } else {
            Some(self.tiles[idx as usize].t.clone())
        }
    }

    pub fn get_tile_vector<'a>(&'a self) -> &'a Vec<Tile> {
        &self.tiles
    }

    pub fn print(&self) {
        let dungeon = self;
        for y in 0..dungeon.height {
            for x in 0..dungeon.width {
                let t: char = match dungeon.get_tile(x, y) {
                    None => '?',
                    Some(tile) => match tile.e.clone() {
                        Some(ent) => match ent {
                            Monster(_) => 'M',
                            // Treasure => 'T',
                            // Key => 'K',
                            // _ => '?',
                        },
                        None => match tile.t {
                            Floor => ' ',
                            Wall => '#',
                            Corridor => ' ',
                            Door => '|',
                            StairsUp => '^',
                            StairsDown => 'V',
                            // _ => '?',
                        },
                    },
                };
                print!("{}", t);
            }
            print!("\n");
        }
        println!("Size: {}x{}", dungeon.width, dungeon.height);
        println!("Path length: {}", dungeon.path_length);
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }
}

impl Default for DungeonParams {
    fn default() -> Self {
        DungeonParams {
            room_count: 25,
            room_size_min: 5,
            room_size_max: 12,
            hall_width_min: 1,
            hall_width_max: 1,
            hall_length_min: 3,
            hall_length_max: 12,
            room_monsters_max: 10,
            hall_monsters_max: 2,
            hall_chance: 0.25,
            map_width: 250,
            map_height: 250,
        }
    }
}

pub fn generate_default(seed: [u8; 32]) -> Dungeon {
    generate(seed, &DungeonParams::default())
}

// TODO monsters and treasure
// TODO stair key in second-furthest room (not adjacent to exit)
pub fn generate(seed: [u8; 32], params: &DungeonParams) -> Dungeon {
    let mut rng: StdRng = StdRng::from_seed(seed);

    // let mut rng = task_rng();
    let mut d = Dungeon::empty(params.map_width, params.map_height);

    let mut rooms: Vec<Room> = Vec::new();
    let mut actual_rooms: usize = 0;

    let mut neighbors: Vec<Vec<usize>> = Vec::new();

    while actual_rooms < params.room_count as usize {
        // don't start with a hall
        let is_first = rooms.is_empty();
        let is_hall = !is_first && rng.gen_range(0f32, 1f32) < params.hall_chance;

        // declare room fields
        let x;
        let y;
        let w;
        let h;
        let mut c_x: Option<isize> = None;
        let mut c_y: Option<isize> = None;
        let mut add_door = true;

        // are we adding
        let mut exist_idx = -1;

        if is_hall {
            let len = rng.gen_range(params.hall_length_min, params.hall_length_max + 1);
            let wid = rng.gen_range(params.hall_width_min, params.hall_width_max + 1);
            if rng.gen_range(0.0f32, 1.0f32) < 0.5f32 {
                // east-west hall
                w = len;
                h = wid;
            } else {
                // north-south hall
                w = wid;
                h = len;
            }
        } else {
            w = rng.gen_range(params.room_size_min, params.room_size_max + 1);
            h = rng.gen_range(params.room_size_min, params.room_size_max + 1);
        }

        if is_first {
            x = rng.gen_range(1, d.width - 1 - w);
            y = rng.gen_range(1, d.height - 1 - h);
        } else {
            exist_idx = rng.gen_range(0, rooms.len() as isize);
            let existing = rooms.get(exist_idx as usize).expect("nooooooo");

            // TODO is this necessary?
            // don't directly connect rooms
            if !is_hall && !existing.hall {
                continue;
            }

            // don't put doors in corridor
            if is_hall && existing.hall {
                add_door = false;
            }

            // pick cardinal direction to attch room
            let direction = rng.gen_range(0, 4);
            // pick x point
            let connect_x = match direction {
                0 | 2 => rng.gen_range(existing.x, existing.x + existing.w),
                1 | 3 => match rng.gen_range(0, 2) {
                    0 => existing.x - 1,
                    1 => existing.x + existing.w,
                    _ => panic!("A"),
                },
                _ => panic!("B"),
            };
            // pick y point
            let connect_y = match direction {
                1 | 3 => rng.gen_range(existing.y, existing.y + existing.h),
                0 | 2 => match rng.gen_range(0, 2) {
                    0 => existing.y - 1,
                    1 => existing.y + existing.h,
                    _ => panic!("C"),
                },
                _ => panic!("D"),
            };
            // now that we have a connector point,
            // figure out where we want to put the new room
            x = match direction {
                // we must ADD ONE in this range because
                // the rng is exclusive on the high end!!!
                // what a bug...
                0 | 2 => rng.gen_range(connect_x - (w - 1), connect_x + 1),
                1 => connect_x + 1,
                3 => connect_x - w,
                _ => panic!("E"),
            };
            y = match direction {
                // same as above
                1 | 3 => rng.gen_range(connect_y - (h - 1), connect_y + 1),
                2 => connect_y + 1,
                0 => connect_y - h,
                _ => panic!("F"),
            };
            c_x = Some(connect_x);
            c_y = Some(connect_y);
        }

        let room = Room {
            x,
            y,
            w,
            h,
            hall: is_hall,
        };

        let fill_type = if is_hall { Corridor } else { Floor };
        if d.fill_room(&room, fill_type) {
            rooms.push(room);

            // add door unless both are halls
            let door_tile = if add_door { Door } else { fill_type };
            match (c_x, c_y) {
                (Some(x), Some(y)) => {
                    d.set_tile(x, y, door_tile);
                }
                _ => {}
            }

            // update room count if not hall
            if !room.hall {
                actual_rooms += 1;
            }

            // update adjacency list
            neighbors.push(Vec::new());
            if exist_idx != -1 {
                let new_idx = rooms.len() - 1;
                neighbors[exist_idx as usize].push(new_idx as usize);
                neighbors[new_idx].push(exist_idx as usize);
            }
        } // else try again :-(
    }

    // let's do this differently
    let mut start_idx_opt: Option<usize> = None;

    while start_idx_opt.is_none() || rooms[start_idx_opt.expect("Shouldn't be here lawl")].hall {
        start_idx_opt = Some(rng.gen_range(0, rooms.len() as isize) as usize);
    }

    let start_idx = start_idx_opt.expect("Okay cool good wut");

    let mut distances: Vec<Option<usize>> = Vec::new();
    for _ in 0..rooms.len() {
        distances.push(None)
    }
    let mut queue: Vec<usize> = Vec::new();
    distances[start_idx] = Some(0);
    queue.push(start_idx);

    let mut visited_idx: Vec<usize> = Vec::new();

    while !queue.is_empty() {
        let current_idx = queue.remove(0);
        let current_dist = distances[current_idx].expect("Distance should be set");
        let adjacent = neighbors.get(current_idx).expect("mr skeltal");
        for neighbor in adjacent.iter() {
            if visited_idx.contains(neighbor) {
                continue;
            }

            let old_dist = distances[neighbor.clone() as usize].clone();
            let new_dist = current_dist + 1;
            if old_dist.is_none() || old_dist.expect("wat") > new_dist {
                distances[neighbor.clone() as usize] = Some(new_dist);
            }
            queue.push(neighbor.clone() as usize);
        }

        visited_idx.push(current_idx);
    }

    // for i in 0..distances.len() {
    // 	println!("{}->{}",i,distances.get(i).expect("Distance not set?!"));
    // }

    // find room with furthest distance
    let mut furthest_idx: Option<usize> = None;
    let mut furthest_dist: Option<usize> = None;
    for i in 0..rooms.len() {
        let room = rooms[i].clone();
        if room.hall {
            continue;
        }
        let dist: usize = distances[i].expect("Distance somehow isn't set");
        if furthest_dist.is_none() || furthest_dist.expect("O_o") < dist {
            furthest_dist = Some(dist);
            furthest_idx = Some(i);
        }
    }

    // println!("Furthest room is room {} with distance {}",furthest_idx.unwrap(),furthest_dist.unwrap());

    let end = rooms
        .get_mut(furthest_idx.expect("Furthest index not set"))
        .expect("pls no")
        .clone();
    d.path_length = furthest_dist.expect("Srsly wat");

    // put up stairs in start
    let start_room: &Room = rooms.get(start_idx).expect("uuugh");
    let end_room: &Room = &end;

    //println!("Start room: {},{}",start_room.x,start_room.y);
    //println!("End room: {},{}",end_room.x,end_room.y);

    let start_x = start_room.x + start_room.w / 2;
    let start_y = start_room.y + start_room.h / 2;
    let set_start = d.set_tile(start_x, start_y, StairsUp);
    d.start_coords = (start_x, start_y);

    let end_x = end_room.x + end_room.w / 2;
    let end_y = end_room.y + end_room.h / 2;
    let set_end = d.set_tile(end_x, end_y, StairsDown);
    d.end_coords = (end_x, end_y);

    if !set_start || !set_end {
        panic!("Failed to set start/end ({}/{})", set_start, set_end);
    }

    // now let's add some enemies
    // let mut total_monsters = 0;
    let min_room_area = f32::powf(params.room_size_min as f32, 2.);
    let max_room_area = f32::powf(params.room_size_max as f32, 2.);
    let min_hall_area = (params.hall_width_min * params.hall_length_min) as f32;
    let max_hall_area = (params.hall_width_max * params.hall_length_max) as f32;
    for room in rooms.iter() {
        let area = (room.w * room.h) as f32;

        let (min_area, max_area) = if room.hall {
            (min_hall_area, max_hall_area)
        } else {
            (min_room_area, max_room_area)
        };

        let max_possible = if room.hall {
            params.hall_monsters_max
        } else {
            params.room_monsters_max
        } as f32;
        let max_monsters =
            map_range_f32(area, min_area, max_area, 0.0, max_possible, true).round() as isize;
        let monster_count = rng.gen_range(0, max_monsters + 1);
        // total_monsters += monster_count;

        let mut placed_monsters = 0;
        while placed_monsters < monster_count {
            let x = rng.gen_range(room.x, room.x + room.w);
            let y = rng.gen_range(room.y, room.y + room.h);
            let tile = d
                .get_tile_mut(x, y)
                .expect("This should NOT be out of range");
            match tile.e {
                Some(_) => continue,
                None => {
                    tile.e = Some(Monster(rng.gen()));
                    placed_monsters += 1;
                }
            }
        }
    }
    d
}

/*************/
/* internals */
/*************/

impl Dungeon {
    fn fix_coords(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = x + y * self.width;
                let tile = self.tiles.get_mut(idx as usize).expect("Just ... no");
                tile.x = x;
                tile.y = y;
                match tile.t {
                    StairsUp => self.start_coords = (tile.x, tile.y),
                    StairsDown => self.end_coords = (tile.x, tile.y),
                    _ => {}
                }
            }
        }
    }

    pub fn shrink(&mut self) {
        loop {
            if !self.shrink_h(1, 0) {
                break;
            }
        }
        loop {
            let w = self.width;
            if !self.shrink_h(w - 2, w - 1) {
                break;
            }
        }
        loop {
            if !self.shrink_v(1, 0) {
                break;
            }
        }
        loop {
            let h = self.height;
            if !self.shrink_v(h - 2, h - 1) {
                break;
            }
        }
        self.fix_coords();
    }

    fn shrink_h(&mut self, x_check: isize, x_remove: isize) -> bool {
        for y in 0..self.height {
            match self.get_tile(x_check, y) {
                None => return false,
                Some(tile) => match tile.t {
                    Wall => {}
                    _ => return false,
                },
            }
        }

        for y in 0..self.height {
            let z = self.height - y - 1;
            let idx = z * self.width + x_remove;
            self.tiles.remove(idx as usize);
        }
        self.width -= 1;
        true
    }

    fn shrink_v(&mut self, y_check: isize, y_remove: isize) -> bool {
        for x in 0..self.width {
            match self.get_tile(x, y_check) {
                None => return false,
                Some(tile) => match tile.t {
                    Wall => {}
                    _ => return false,
                },
            }
        }

        for x in 0..self.width {
            let x2 = self.width - x - 1;
            let idx = y_remove * self.width + x2;
            self.tiles.remove(idx as usize);
        }
        self.height -= 1;
        true
    }

    fn get_tile_mut<'a>(&'a mut self, x: isize, y: isize) -> Option<&'a mut Tile> {
        let idx = x + y * self.width;

        if x >= self.width
            || y >= self.height
            || x < 0
            || y < 0
            || idx < 0
            || idx >= self.tiles.len() as isize
        {
            None
        } else {
            self.tiles.get_mut(idx as usize)
        }
    }

    fn set_tile<'a>(&'a mut self, x: isize, y: isize, t: TileType) -> bool {
        match self.get_tile_mut(x, y) {
            None => false,
            Some(tile) => {
                tile.t = t;
                true
            }
        }
    }

    fn fill_tiles<'a>(&'a mut self, x: isize, y: isize, w: isize, h: isize, t: TileType) -> bool {
        for y in y..(y + h) {
            for x in x..(x + w) {
                match self.get_tile_mut(x, y) {
                    None => return false,
                    Some(_) => {}
                }
            }
        }
        for y in y..(y + h) {
            for x in x..(x + w) {
                self.set_tile(x, y, t);
            }
        }
        true
    }

    fn fill_room(&mut self, room: &Room, t: TileType) -> bool {
        if self.is_room_position_valid(room.x, room.y, room.w, room.h) {
            self.fill_tiles(room.x, room.y, room.w, room.h, t)
        } else {
            false
        }
    }

    fn is_room_position_valid(&self, x: isize, y: isize, w: isize, h: isize) -> bool {
        if x == 0 || y == 0 || w == 0 || h == 0 {
            return false;
        }

        for y in (y - 1)..(y + h + 1) {
            for x in (x - 1)..(x + w + 1) {
                match self.get_tile(x, y) {
                    None => return false,
                    Some(tile) => match tile.t {
                        Wall => {}
                        _ => return false,
                    },
                }
            }
        }
        true
    }
}

#[derive(Clone, Copy)]
struct Room {
    x: isize,
    y: isize,
    w: isize,
    h: isize,
    hall: bool,
}
