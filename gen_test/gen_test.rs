extern crate rand;

use std::vec::Vec;
use std::io::{stdin,BufferedReader};
use rand::{task_rng,Rng,TaskRng,random};

struct Tile {
	x: int,
	y: int,
	t: TileType
}

enum TileType {
	Floor,
	Wall,
	Door,
	Corridor,
	StairsUp,
	StairsDown,
	Unknown
}

#[deriving(Clone)]
struct Room {
	x: int,
	y: int,
	w: int,
	h: int,
	hall: bool
}

pub struct Dungeon {
	width: int,
	height: int,
	tiles: Vec<Tile>,
	path_length: int
}

impl Dungeon {
	pub fn empty(w: int, h: int) -> Dungeon {
		Dungeon {
			width: w,
			height: h,
			tiles: Vec::from_fn( (w*h) as uint, |i| {
				let i = i as int;
				let x: int = i % w;
				let y: int = i / w;
				Tile { x: x, y: y, t: Wall }
			}),
			path_length: 0
		}
	}

	fn fix_coords(&mut self) {
		for y in range(0,self.height) {
			for x in range(0,self.width) {
				let idx = x + y*self.width;
				let tile = self.tiles.get_mut(idx as uint);
				tile.x = x;
				tile.y = y;
			}
		}
	}

	fn shrink(&mut self) {
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

	fn get_tile<'a>(&'a self, x: int, y: int) -> Option<&'a Tile> {

		let idx = x+y*self.width;

		if x >= self.width || y >= self.height || x < 0 || y < 0 || idx < 0 || idx >= self.tiles.len() as int {
			None
		} else {
			Some(self.tiles.get(idx as uint))
		}

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

fn print_dungeon(dungeon: &Dungeon) {
	for y in range(0,dungeon.height) {
		for x in range(0,dungeon.width) {
			let t: char = match dungeon.get_tile(x,y) {
				None => '?',
				Some(tile) => match tile.t {
					Floor => ' ',
					Wall => '#',
					Corridor => '.',
					Door => 'X',
					StairsUp => '^',
					StairsDown => 'V',
					_ => '?'
				}
			};
			print!("{}",t);
		}
		print!("\n");
	}
	println!("Size: {}x{}",dungeon.width,dungeon.height);
	println!("Path length: {}",dungeon.path_length);
}

fn random_range(task: &mut TaskRng, lo: int, hi: int) -> int {
	if lo == hi {
		fail!("FFFF");
	} else {
		task.gen_range(lo,hi)
	}
}

pub struct DungeonParams {
	room_count: int,
	room_size_min: int,
	room_size_max: int,
	hall_width_min: int,
	hall_width_max: int,
	hall_length_min: int,
	hall_length_max: int,
	hall_chance: f32,
	map_width: int,
	map_height: int
}

impl DungeonParams {
	pub fn default() -> DungeonParams {
		DungeonParams {
			room_count: 50,
			room_size_min: 3,
			room_size_max: 10,
			hall_width_min: 1,
			hall_width_max: 1,
			hall_length_min: 2,
			hall_length_max: 10,
			hall_chance: 0.25,
			map_width: 200,
			map_height: 200
		}
	}
}

pub fn generate_default() -> Dungeon {
	generate(&DungeonParams::default())
}

pub fn generate(params: &DungeonParams) -> Dungeon {

	let mut rng = task_rng();
	let mut d = Dungeon::empty(params.map_width,params.map_height);

	let mut rooms: Vec<Room> = Vec::new();
	let mut actual_rooms: uint = 0;

	let mut neighbors: Vec<Vec<int>> = Vec::new();

	let mut start: Option<Room> = None;

	while actual_rooms < params.room_count as uint {

		// don't start with a hall
		let is_first = rooms.len() == 0;
		let is_hall = !is_first && random::<f32>() < params.hall_chance;

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
			let len = random_range(&mut rng, params.hall_length_min,params.hall_length_max+1);
			let wid = random_range(&mut rng, params.hall_width_min,params.hall_width_max+1);
			if random::<f32>() < 0.5 {
				// east-west hall
				w = len;
				h = wid;
			} else {
				// north-south hall
				w = wid;
				h = len;
			}

		} else {

			w = random_range(&mut rng, params.room_size_min,params.room_size_max+1);
			h = random_range(&mut rng, params.room_size_min,params.room_size_max+1);

		}

		if is_first {
			x = random_range(&mut rng, 1, d.width - 1 - w);
			y = random_range(&mut rng, 1, d.height - 1 - h);
		} else {
			exist_idx = random_range(&mut rng, 0,rooms.len() as int);
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
			let direction = random_range(&mut rng, 0,4);
			// pick x point
			let connect_x = match direction {
				0|2 => random_range(&mut rng, existing.x,existing.x+existing.w),
				1|3 => match random_range(&mut rng, 0,2) {
					0 => existing.x-1,
					1 => existing.x + existing.w,
					_ => fail!("A")
				},
				_ => fail!("B")
			};
			// pick y point
			let connect_y = match direction {
				1|3 => random_range(&mut rng, existing.y,existing.y+existing.h),
				0|2 => match random_range(&mut rng, 0,2) {
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
				0|2 => random_range(&mut rng, connect_x-(w-1),connect_x+1),
				1 => connect_x+1,
				3 => connect_x-w,
				_ => fail!("E")
			};
			y = match direction {
				// same as above
				1|3 => random_range(&mut rng, connect_y-(h-1),connect_y+1),
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
			if is_first{ start = Some(room) }
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
		}
	}


	// let's do this differently
	let mut distances: Vec<Option<int>> = Vec::new();
	for _ in range(0,rooms.len()) { distances.push(None) }
	let mut queue: Vec<uint> = Vec::new();
	*distances.get_mut(0) = Some(0);
	queue.push(0);

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
	let start_room: &Room = &start.expect("Start is broken");
	let end_room: &Room = &end;

	//println!("Start room: {},{}",start_room.x,start_room.y);
	//println!("End room: {},{}",end_room.x,end_room.y);

	let set_start = d.set_tile(
		start_room.x + start_room.w / 2,
		start_room.y + start_room.h / 2,
		StairsUp
	);

	let set_end = d.set_tile(
		end_room.x + end_room.w / 2,
		end_room.y + end_room.h / 2,
		StairsDown
	);

	if !set_start || !set_end {
		fail!("Failed to set start/end ({}/{})",set_start,set_end);
	}

	// ROOM POSITIONS ARE INVALID
	// AFTER SHRINKING!!!
	// heeerrrppp dddeeerrrpppp
	d.shrink();
	d
}

fn int_from_reader<T: Reader>(reader: &mut BufferedReader<T>) -> Option<int> {
	let input = reader.read_line().ok().expect("Invalid input!");
	let opt = from_str::<int>(input.slice_to(input.len() - 1));
	opt
}

fn main() {

	let mut reader = BufferedReader::new(stdin());

	print!("How many dungeons to generate: ");
	let count = int_from_reader(&mut reader).expect("That's not a number!");
	let mut dungeons = Vec::new();

	let mut total_w: f32 = 0.;
	let mut total_h: f32 = 0.;
	let mut total: f32 = 0.;
	for i in range(0,count) {
		println!("Generating dungeon {}...",i);
		let d = generate_default();
		total_w += d.width as f32;
		total_h += d.height as f32;
		total += 1.0;
		dungeons.push(d);
	}
	println!("Average size across {} dungeons was {}x{}.",count, (total_w/total).round(), (total_h/total).round());

	loop {
		print!("Enter a number to view that dungeon, or any non-number to exit: ");
		let idx = int_from_reader(&mut reader);
		match idx {
			None => {println!("Ooookaaaay byyyyye~"); break;}
			Some(number) => {
				let idx = number as uint;
				if idx < 0 || idx >= dungeons.len() {
					println!("That's out of range!");
					continue;
				} else {
					print_dungeon(dungeons.get(idx));
				}
			}
		}
	}
}