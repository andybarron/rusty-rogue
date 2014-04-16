extern crate rand;

use std::vec::Vec;
use std::io::{stdin,BufferedReader};
use std::from_str;
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
	Unknown
}

struct Room {
	x: int,
	y: int,
	w: int,
	h: int,
	hall: bool
}

struct Dungeon {
	width: int,
	height: int,
	tiles: Vec<Tile>
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
			})
		}
	}

	fn shrink(&mut self) {
		let mut d = self;
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
					_ => '?'
				}
			};
			print!("{}",t);
		}
		print!("\n");
	}
	println!("Size: {}x{}",dungeon.width,dungeon.height);
}

static ROOM_COUNT: int = 100;
static ROOM_S_MIN: int = 3;
static ROOM_S_MAX: int = 10;
static HALL_W_MIN: int = 1;
static HALL_W_MAX: int = 2;
static HALL_L_MIN: int = 2;
static HALL_L_MAX: int = 10;
static HALL_CHANCE: f32 = 0.5;

static MAP_W: int = 150;
static MAP_H: int = 150;

fn random_range(task: &mut TaskRng, lo: int, hi: int) -> int {
	if lo == hi {
		fail!("FFFF");
	} else {
		task.gen_range(lo,hi)
	}
}

fn generate() -> Dungeon {

	let mut rng = task_rng();
	let mut d = Dungeon::empty(MAP_W,MAP_H);

	let mut rooms: Vec<Room> = Vec::new();

	let mut start: Option<Room> = None;
	let mut end: Option<Room> = None;

	while rooms.len() < ROOM_COUNT as uint {

		// don't start with a hall
		let is_first = rooms.len() == 0;
		let is_hall = !is_first && random::<f32>() < HALL_CHANCE;

		// declare room fields
		let mut x;
		let mut y;
		let mut w;
		let mut h;
		let mut c_x: Option<int> = None;
		let mut c_y: Option<int> = None;
		let mut add_door = true;

		if is_hall {
			let len = random_range(&mut rng, HALL_L_MIN,HALL_L_MAX+1);
			let wid = random_range(&mut rng, HALL_W_MIN,HALL_W_MAX+1);
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

			w = random_range(&mut rng, ROOM_S_MIN,ROOM_S_MAX+1);
			h = random_range(&mut rng, ROOM_S_MIN,ROOM_S_MAX+1);

		}

		if is_first {
			x = random_range(&mut rng, 1, d.width - 1 - w);
			y = random_range(&mut rng, 1, d.height - 1 - h);
		} else {
			let exist_idx = random_range(&mut rng, 0,rooms.len() as int);
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
		}
	}
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
		let d = generate();
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