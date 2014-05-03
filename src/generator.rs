use std::num::pow;
use std::vec::Vec;
use rand::{Rng,XorShiftRng,SeedableRng};
use util::map_range_f32;

#[deriving(Clone)]
pub struct Tile {
	pub x: int,
	pub y: int,
	pub t: TileType,
	pub e: Option<Entity>
}

#[deriving(Clone)]
pub enum Entity {
	Monster(uint),
	Treasure,
	Key,
	Missingno
}

#[deriving(Clone,Eq)]
pub enum TileType {
	Floor,
	Wall,
	Door,
	Corridor,
	StairsUp,
	StairsDown,
	Unknown
}

#[deriving(Clone)]
pub struct Dungeon {
	pub width: int,
	pub height: int,
	pub tiles: Vec<Tile>,
	path_length: int,
	pub start_coords: (int,int),
	pub end_coords: (int,int),
}

#[deriving(Clone)]
pub struct DungeonParams {
	room_count: int,
	room_size_min: int,
	room_size_max: int,
	hall_width_min: int,
	hall_width_max: int,
	hall_length_min: int,
	hall_length_max: int,
	room_monsters_max: int,
	hall_monsters_max: int,
	hall_chance: f32,
	map_width: int,
	map_height: int
}

/********************/
/* public functions */
/********************/

impl Dungeon {

	pub fn empty(w: int, h: int) -> Dungeon {
		Dungeon {
			width: w,
			height: h,
			tiles: Vec::from_fn( (w*h) as uint, |i| {
				let i = i as int;
				let x: int = i % w;
				let y: int = i / w;
				Tile { x: x, y: y, t: Wall, e: None }
			}),
			path_length: 0,
			start_coords: (0,0),
			end_coords: (0,0),
		}
	}

	pub fn get_tile<'a>(&'a self, x: int, y: int) -> Option<&'a Tile> {

		let idx = x+y*self.width;

		if x >= self.width || y >= self.height || x < 0 || y < 0 || idx < 0 || idx >= self.tiles.len() as int {
			None
		} else {
			Some(self.tiles.get(idx as uint))
		}

	}

	pub fn get_tile_type(&self, x: int, y: int) -> Option<TileType> {
		let idx = x+y*self.width;

		if x >= self.width || y >= self.height || x < 0 || y < 0 || idx < 0 || idx >= self.tiles.len() as int {
			None
		} else {
			Some(self.tiles.get(idx as uint).t)
		}
	}

	pub fn get_tile_vector<'a>(&'a self) -> &'a Vec<Tile> {
		&self.tiles
	}

	pub fn print(&self) {
		let dungeon = self;
		for y in range(0,dungeon.height) {
			for x in range(0,dungeon.width) {
				let t: char = match dungeon.get_tile(x,y) {
					None => '?',
					Some(tile) => match tile.e {
						Some(ent) => match ent {
							Monster(_) => 'M',
							Treasure => 'T',
							Key => 'K',
							_ => '?'
						},
						None => match tile.t {
							Floor => ' ',
							Wall => '#',
							Corridor => ' ',
							Door => '|',
							StairsUp => '^',
							StairsDown => 'V',
							_ => '?'
						}
					}
				};
				print!("{}",t);
			}
			print!("\n");
		}
		println!("Size: {}x{}",dungeon.width,dungeon.height);
		println!("Path length: {}",dungeon.path_length);
	}

	pub fn width(&self) -> int {
		self.width
	}

	pub fn height(&self) -> int {
		self.height
	}

}

impl DungeonParams {
	pub fn default() -> DungeonParams {
		DungeonParams {
			room_count: 50,
			room_size_min: 3,
			room_size_max: 7,
			hall_width_min: 1,
			hall_width_max: 1,
			hall_length_min: 2,
			hall_length_max: 10,
			room_monsters_max: 10,
			hall_monsters_max: 2,
			hall_chance: 0.25,
			map_width: 200,
			map_height: 200
		}
	}
	pub fn small() -> DungeonParams {
		DungeonParams {
			room_count: 10,
			room_size_min: 5,
			room_size_max: 10,
			hall_width_min: 1,
			hall_width_max: 2,
			hall_length_min: 2,
			hall_length_max: 5,
			room_monsters_max: 8,
			hall_monsters_max: 1,
			hall_chance: 0.15,
			map_width: 200,
			map_height: 200
		}
	}
	pub fn huge() -> DungeonParams {
		DungeonParams {
			room_count: 100,
			room_size_min: 3,
			room_size_max: 10,
			hall_width_min: 1,
			hall_width_max: 3,
			hall_length_min: 3,
			hall_length_max: 15,
			room_monsters_max: 30,
			hall_monsters_max: 5,
			hall_chance: 0.33,
			map_width: 250,
			map_height: 250
		}
	}
}


pub fn generate_default(seed: u32) -> Dungeon {
	generate(seed,&DungeonParams::default())
}

// TODO monsters and treasure
// TODO stair key in second-furthest room (not adjacent to exit)
pub fn generate(seed: u32, params: &DungeonParams) -> Dungeon {

	let seed_array = [seed+1,seed/2,seed/4,seed/8];
	let mut rng: XorShiftRng = SeedableRng::from_seed(seed_array);

	// let mut rng = task_rng();
	let mut d = Dungeon::empty(params.map_width,params.map_height);

	let mut rooms: Vec<Room> = Vec::new();
	let mut actual_rooms: uint = 0;

	let mut neighbors: Vec<Vec<int>> = Vec::new();

	while actual_rooms < params.room_count as uint {

		// don't start with a hall
		let is_first = rooms.len() == 0;
		let is_hall = !is_first && rng.gen_range(0f32,1f32) < params.hall_chance;

		// declare room fields
		let mut x;
		let mut y;
		let mut w;
		let mut h;
		let mut c_x: Option<int> = None;
		let mut c_y: Option<int> = None;
		let mut add_door = true;

		// are we adding
		let mut exist_idx = -1;

		if is_hall {
			let len = rng.gen_range(params.hall_length_min,params.hall_length_max+1);
			let wid = rng.gen_range(params.hall_width_min,params.hall_width_max+1);
			if rng.gen_range(0.0f32,1.0f32) < 0.5f32 { // east-west hall
				w = len;
				h = wid;
			} else { // north-south hall
				w = wid;
				h = len;
			}
		} else {
			w = rng.gen_range(params.room_size_min,params.room_size_max+1);
			h = rng.gen_range(params.room_size_min,params.room_size_max+1);
		}

		if is_first {
			x = rng.gen_range(1, d.width - 1 - w);
			y = rng.gen_range(1, d.height - 1 - h);
		} else {
			exist_idx = rng.gen_range(0,rooms.len() as int);
			let existing = rooms.get(exist_idx as uint);

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
			let direction = rng.gen_range(0,4);
			// pick x point
			let connect_x = match direction {
				0|2 => rng.gen_range(existing.x,existing.x+existing.w),
				1|3 => match rng.gen_range(0,2) {
					0 => existing.x-1,
					1 => existing.x + existing.w,
					_ => fail!("A")
				},
				_ => fail!("B")
			};
			// pick y point
			let connect_y = match direction {
				1|3 => rng.gen_range(existing.y,existing.y+existing.h),
				0|2 => match rng.gen_range(0,2) {
					0 => existing.y-1,
					1 => existing.y + existing.h,
					_ => fail!("C")
				},
				_ => fail!("D")
			};
			// now that we have a connector point,
			// figure out where we want to put the new room
			x = match direction {
				// we must ADD ONE in this range because
				// the rng is exclusive on the high end!!!
				// what a bug...
				0|2 => rng.gen_range(connect_x-(w-1),connect_x+1),
				1 => connect_x+1,
				3 => connect_x-w,
				_ => fail!("E")
			};
			y = match direction {
				// same as above
				1|3 => rng.gen_range(connect_y-(h-1),connect_y+1),
				2 => connect_y+1,
				0 => connect_y-h,
				_ => fail!("F")
			};
			c_x = Some(connect_x);
			c_y = Some(connect_y);
		}

		let room = Room { x: x, y: y, w: w, h: h, hall: is_hall };

		let fill_type = if is_hall { Corridor } else { Floor };
		if d.fill_room(&room,fill_type) {
			rooms.push(room);

			// add door unless both are halls
			let door_tile = if add_door { Door } else { fill_type };
			match (c_x,c_y) {
				(Some(x),Some(y)) => { d.set_tile(x,y,door_tile); }
				_ => {}
			}

			// update room count if not hall
			if !room.hall {
				actual_rooms += 1;
			}

			// update adjacency list
			neighbors.push(Vec::new());
			if exist_idx != -1 {
				let new_idx = rooms.len()-1;
				neighbors.get_mut(exist_idx as uint).push(new_idx as int);
				neighbors.get_mut(new_idx).push(exist_idx as int);
			}
		} // else try again :-(
	}


	// let's do this differently
	let mut start_idx_opt = None;

	while start_idx_opt.is_none() || rooms.get(start_idx_opt.expect("Shouldn't be here lawl")).hall {
		start_idx_opt = Some( rng.gen_range(0, rooms.len() as int) as uint );
	}

	let start_idx = start_idx_opt.expect("Okay cool good wut");

	let mut distances: Vec<Option<int>> = Vec::new();
	for _ in range(0,rooms.len()) { distances.push(None) }
	let mut queue: Vec<uint> = Vec::new();
	*distances.get_mut(start_idx) = Some(0);
	queue.push(start_idx);

	let mut visited_idx: Vec<int> = Vec::new();

	while queue.len() > 0 {
		let current_idx = queue.remove(0).expect("Queue shouldn't be empty");
		let current_dist = distances.get(current_idx).expect("Distance should be set");
		let adjacent = neighbors.get(current_idx);
		for neighbor in adjacent.iter() {

			if visited_idx.contains( neighbor ) { continue; }

			let old_dist = distances.get(neighbor.clone() as uint).clone();
			let new_dist = current_dist + 1;
			if old_dist.is_none() || old_dist.expect("wat") > new_dist {
				*distances.get_mut(neighbor.clone() as uint) = Some(new_dist);
			}
			queue.push(neighbor.clone() as uint);
		}

		visited_idx.push(current_idx as int);
	}

	// for i in range(0,distances.len()) {
	// 	println!("{}->{}",i,distances.get(i).expect("Distance not set?!"));
	// }

	// find room with furthest distance
	let mut furthest_idx: Option<uint> = None;
	let mut furthest_dist: Option<int> = None;
	for i in range(0,rooms.len()) {
		let room = rooms.get(i);
		if room.hall { continue; }
		let dist = distances.get(i).clone().expect("Distance somehow isn't set");
		if furthest_dist.is_none() || furthest_dist.expect("O_o") < dist {
			furthest_dist = Some(dist);
			furthest_idx = Some(i);
		}
	}

	// println!("Furthest room is room {} with distance {}",furthest_idx.unwrap(),furthest_dist.unwrap());

	let end = rooms.get_mut(furthest_idx.expect("Furthest index not set")).clone();
	d.path_length = furthest_dist.expect("Srsly wat");

	// put up stairs in start
	let start_room: &Room = rooms.get(start_idx);
	let end_room: &Room = &end;

	//println!("Start room: {},{}",start_room.x,start_room.y);
	//println!("End room: {},{}",end_room.x,end_room.y);

	let start_x = start_room.x + start_room.w / 2;
	let start_y = start_room.y + start_room.h / 2;
	let set_start = d.set_tile(
		start_x,
		start_y,
		StairsUp
	);
	d.start_coords = (start_x,start_y);

	let end_x = end_room.x + end_room.w / 2;
	let end_y = end_room.y + end_room.h / 2;
	let set_end = d.set_tile(
		end_x,
		end_y,
		StairsDown
	);
	d.end_coords = (end_x,end_y);

	if !set_start || !set_end {
		fail!("Failed to set start/end ({}/{})",set_start,set_end);
	}

	// now let's add some enemies
	let mut total_monsters = 0;
	let min_room_area = pow(params.room_size_min as f32, 2);
	let max_room_area = pow(params.room_size_max as f32, 2);
	let min_hall_area = (params.hall_width_min*params.hall_length_min) as f32;
	let max_hall_area = (params.hall_width_max*params.hall_length_max) as f32;
	for room in rooms.iter() {
		let area = (room.w * room.h) as f32;

		let (min_area, max_area) = if room.hall {
			(min_hall_area,max_hall_area)
		} else {
			(min_room_area,max_room_area)
		};

		let max_possible = if room.hall { params.hall_monsters_max } else { params.room_monsters_max } as f32;
		let max_monsters = map_range_f32( area, min_area, max_area, 0.0, max_possible, true ).round() as int;
		let monster_count = rng.gen_range(0, max_monsters + 1);
		total_monsters += monster_count;

		let mut placed_monsters = 0;
		while placed_monsters < monster_count {
			let x = rng.gen_range(room.x, room.x+room.w);
			let y = rng.gen_range(room.y, room.y+room.h);
			let tile = d.get_tile_mut(x,y).expect("This should NOT be out of range");
			match tile.e {
				Some(_) => continue,
				None => {
					tile.e = Some(Monster(rng.gen()));
					placed_monsters += 1;
				}
			}
		}
	}

	// println!("{} total monsters", total_monsters);

	// ALL POSITIONS ARE INVALID
	// AFTER SHRINKING!!!
	// heeerrrppp dddeeerrrpppp
	// d.shrink();

	// TODO do we actually want to shrink?

	d
}

/*************/
/* internals */
/*************/

impl Dungeon {

	fn fix_coords(&mut self) {
		for y in range(0,self.height) {
			for x in range(0,self.width) {
				let idx = x + y*self.width;
				let tile = self.tiles.get_mut(idx as uint);
				tile.x = x;
				tile.y = y;
				match tile.t {
					StairsUp => self.start_coords = (tile.x,tile.y),
					StairsDown => self.end_coords = (tile.x,tile.y),
					_ => {}
				}
			}
		}
	}

	pub fn shrink(&mut self) {
		let d = self;
		while d.shrink_h(1,0) {
			// println!("Removed a left column");
		}
		while d.shrink_h(d.width-2,d.width-1) {
			// println!("Removed a right column");
		}
		while d.shrink_v(1,0) {
			// println!("Removed a top row");
		}
		while d.shrink_v(d.height-2,d.height-1) {
			// println!("Removed a bottom row");
		}
		d.fix_coords();
	}

	fn shrink_h(&mut self, x_check: int, x_remove: int) -> bool {

		for y in range(0,self.height)
		{
			match self.get_tile(x_check,y) {
				None => return false,
				Some(tile) => match tile.t {
					Wall => {}
					_ => return false
				}
			}
		}

		for y in range(0,self.height) {
			let z = self.height - y - 1;
			let idx = z*self.width+x_remove;
			self.tiles.remove(idx as uint);
		}
		self.width -= 1;
		true
	}

	fn shrink_v(&mut self, y_check: int, y_remove: int) -> bool {

		for x in range(0,self.width)
		{
			match self.get_tile(x,y_check) {
				None => return false,
				Some(tile) => match tile.t {
					Wall => {}
					_ => return false
				}
			}
		}

		for x in range(0,self.width) {
			let x2 = self.width - x - 1;
			let idx = y_remove*self.width+x2;
			self.tiles.remove(idx as uint);
		}
		self.height -= 1;
		true
	}

	fn get_tile_mut<'a>(&'a mut self, x: int, y: int) -> Option<&'a mut Tile> {

		let idx = x+y*self.width;

		if x >= self.width || y >= self.height || x < 0 || y < 0 || idx < 0 || idx >= self.tiles.len() as int {
			None
		} else {
			Some(self.tiles.get_mut(idx as uint))
		}

	}

	fn set_tile<'a>(&'a mut self, x: int, y: int, t: TileType) -> bool {
		match self.get_tile_mut(x,y) {
			None => false,
			Some(tile) => { tile.t = t; true }
		}
	}

	fn fill_tiles<'a>(&'a mut self, x: int, y: int, w: int, h: int, t: TileType) -> bool {
		for y in range (y, y+h) {
			for x in range(x, x+w) {
				match self.get_tile_mut(x,y) {
					None => return false,
					Some(_) => {}
				}
			}
		}
		for y in range (y, y+h) {
			for x in range(x, x+w) {
				self.set_tile(x,y,t);
			}
		}
		true
	}

	fn fill_room(&mut self, room: &Room, t: TileType) -> bool {
		if self.is_room_position_valid(room.x,room.y,room.w,room.h) {
			self.fill_tiles(room.x,room.y,room.w,room.h,t)
		} else {
			false
		}
	}

	fn is_room_position_valid(&self, x: int, y: int, w: int, h: int) -> bool {
		if x == 0 || y == 0 || w == 0 || h == 0 { return false; }

		for y in range (y-1, y+h+1) {
			for x in range (x-1, x+w+1) {
				match self.get_tile(x,y) {
					None => return false,
					Some(tile) => match tile.t {
						Wall => {},
						_ => return false
					}
				}
			}
		}
		return true;
	}

}

#[deriving(Clone)]
struct Room {
	x: int,
	y: int,
	w: int,
	h: int,
	hall: bool
}