// TODO custom tile culling so we don't draw the entire level every frame

use std::mem::swap;

use rsfml::graphics::RenderWindow;
use rsfml::graphics::View;
use rsfml::graphics::Texture;
use rsfml::graphics::rc::Sprite;
use rsfml::graphics::Color;
use rsfml::graphics::IntRect;
use rsfml::graphics::FloatRect;
use rsfml::system::Vector2f;
use rsfml::window::keyboard;
use rsfml::window::keyboard::Key;
use rsfml::graphics::rc::CircleShape;
use rsfml::graphics::rc::RectangleShape;

use engine::{Game,Screen};
use generator::{generate_default,Tile,TileType,Dungeon,Floor,Corridor,Door,StairsUp,StairsDown,Monster};
use util::get_gfx_path;
use util::get_rc_resource;
use util::get_sprite_coords;
use util;

use collision::CollisionResolver;

use entities::Creature;

use graph::Graph;
use search::{SearchStrategy,AStarSearch};

pub struct GameplayScreen {
	tile_size: uint,
	dungeon: Dungeon,
	graph: Graph,
	circles: Vec<CircleShape>,
	lines: Vec<RectangleShape>,
	tiles: Vec<TileData>,
	view: View,
	zoom_index: int,
	zoom_levels: ~[f32],
	creatures: Vec<Creature>,
	debug: bool,
}

impl GameplayScreen  {

	pub fn new(dungeon: &Dungeon) -> GameplayScreen {

		// load tile texture file
		let tex_path = get_gfx_path("all_tiles.png");
		let tex = Texture::new_from_file( tex_path ).expect("Failed to load all_tiles.png");

		// get refcounted version for rc::Sprite
		let rc_tex = get_rc_resource(tex);

		// init screen
		let mut ret = GameplayScreen {
			tile_size: 16,
			dungeon: dungeon.clone(),
			graph: Graph::new(),
			circles: Vec::new(),
			lines: Vec::new(),
			zoom_index: 1,
			zoom_levels: ~[1.,2.,3.,4.],
			tiles: Vec::new(),
			view: View::new().expect("Failed to create View"),
			creatures: Vec::new(),
			debug: false
		};

		// closure to get tile coordinates from tile x/y index
		// i.e. top left tile in texture atlas is (0,0)
		let t_sz = ret.tile_size;
		let grab_tile_rect = |x: uint, y: uint| -> IntRect {
			let (tx,ty) = get_sprite_coords(x,y,t_sz,t_sz);
			IntRect{ left: tx as i32, top: ty as i32, width: t_sz as i32, height: t_sz as i32 }
		};

		// get coordinates of each tile type
		let coords_floor = grab_tile_rect(8,6);
		let coords_door = grab_tile_rect(3,0);
		let coords_hall = grab_tile_rect(7,4);
		let coords_up = grab_tile_rect(9,7);
		let coords_dn = grab_tile_rect(10,7);

		// for each tile in the dungeon
		for tile in dungeon.get_tile_vector().iter() {

			// convert x/y index to px coordinates
			let x = tile.x * t_sz as int;
			let y = tile.y * t_sz as int;

			// load tile coordinates based on tile type
			let tile_coords = match tile.t {
				Floor => ~[coords_floor],
				Door => ~[coords_hall,coords_door],
				Corridor => ~[coords_hall],
				StairsUp => ~[coords_floor,coords_up],
				StairsDown => ~[coords_floor,coords_dn],
				_ => ~[]
			};

			// load sprite from texture and add to tile list
			let half = t_sz as f32/2.0;
			let bounds = FloatRect::new( x as f32 - half, y as f32 - half, t_sz as f32, t_sz as f32 );
			let mut tile_data = TileData::new(&bounds,tile);
			for coords in tile_coords.iter() {
				let mut spr = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to create sprite");
				spr.set_texture_rect(coords);
				spr.set_origin2f(t_sz as f32/2.0, t_sz as f32/2.0);
				spr.set_position( &Vector2f::new(x as f32,y as f32) );
				tile_data.sprites.push(spr);
			}

			ret.tiles.push(tile_data);
		}

		println!("Initializing graph...");
		// initialize graph
		for y in range(0,dungeon.height) {
			for x in range(0,dungeon.width) {
				ret.graph.add_node_at(x,y);
			}
		}


		println!("Starting graph node loop...");
		// loop through the graph and
		// connect accessible nodes
		for y in range(0,dungeon.height) {
			for x in range(0,dungeon.width) {
				let idx_opt = ret.to_tile_idx( (x, y) );
				let idx = idx_opt.expect("Shouldn't be negative");
				match ret.tiles.get(idx).is_passable() {
					false => {},
					true => {
						let t_sz = t_sz as f32;
						let mut line = RectangleShape::new().expect("Rectangle faaaailure");
						line.set_size2f( t_sz, 1.0 );
						line.set_origin2f( 0.0, 0.5 );
						line.set_fill_color( &Color::new_RGBA(0,255,255,150) );
						// only check R, DR, D, DL
						// yay undirected graphs!

						// check R and D
						let c_r = connect_direct(&mut ret,x,y,(1,0));
						let c_d = connect_direct(&mut ret,x,y,(0,1));

						// diagonal
						let c_dr = connect_diag(&mut ret,x,y,(1,1),(x+1,y),(x,y+1));
						let c_dl = connect_diag(&mut ret,x,y,(-1,1),(x-1,y),(x,y+1));

						// circle shape and lines
						let radius = t_sz / 4.0;
						let mut circle = CircleShape::new_init(radius, 8).expect("Couldn't make circle");
						circle.set_origin2f(radius,radius);
						circle.set_position2f(
							x as f32 * t_sz,
							y as f32 * t_sz
						);
						line.set_position(&circle.get_position());
						circle.set_fill_color( &Color::new_RGBA(0,0,255,150) );
						ret.circles.push(circle);

						// lines
						if c_r {
							let mut line = line.clone().expect("c_r");
							ret.lines.push( line );
						}
						if c_d {
							let mut line = line.clone().expect("c_d");
							line.rotate(90.0);
							ret.lines.push( line );
						}
						line.scale2f((2.0 as f32).sqrt(),1.0);
						if c_dr {
							let mut line = line.clone().expect("c_d");
							line.rotate(45.0);
							ret.lines.push( line );
						}
						if c_dl {
							let mut line = line.clone().expect("c_d");
							line.rotate(135.0);
							ret.lines.push( line );
						}
					}
				}
			}
		}
		println!("Done with graph!");


		// load up player sprite
		let coords_hero = grab_tile_rect(4,8);
		let mut sprite_hero = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to create hero sprite");
		sprite_hero.set_texture_rect(&coords_hero);

		// create player creature
		let mut hero = Creature::new(sprite_hero,10);
		let (start_x, start_y) = dungeon.start_coords;
		hero.set_position2f( (start_x*t_sz as int) as f32, (start_y*t_sz as int) as f32 );
		hero.player = true;

		// println!{"{},{}",start_x,start_y};
		// println!("{}",ret.graph.get_neighbors_at( start_x, start_y ));

		// a bunch of monsters
		let coords_slime = grab_tile_rect(10,8);
		let mut sprite_slime = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to load slime sprite");
		sprite_slime.set_texture_rect(&coords_slime);

		// find and create monsters
		for tile in dungeon.tiles.iter() {
			match tile.e {
				Some(Monster) => {
					let mut slime = Creature::new(sprite_slime.clone().expect("Couldn't clone slime sprite"),5);
					slime.set_position2f( (tile.x*t_sz as int) as f32, (tile.y*t_sz as int) as f32 );
					ret.creatures.push(slime);
				}
				_ => {}
			}
		}

		println!("Starting A*...");
		for i in range(0,ret.creatures.len()) {
			break;
			if ret.creatures.get(i).player { continue; }

			let pos = ret.creatures.get(i).get_position();
			let start_coords = ret.to_tile_coords((pos.x,pos.y));
			let end_coords = (start_x,start_y);

			let path = AStarSearch::new_diagonal().solve(&ret.graph, start_coords, end_coords).
					expect("Couldn't solve");


			for coords in path.iter() {
				let (tx,ty) = coords.clone();
				let wx = tx as f32 * t_sz as f32; 
				let wy = ty as f32 * t_sz as f32;
				let radius = 6.0;
				let mut circle = CircleShape::new_init(radius,8).expect("fahsdkfhaslkdfh");
				circle.set_origin2f(radius,radius);
				circle.set_position2f(wx,wy);
				circle.set_fill_color( &Color::new_RGBA(255,0,0,150) );
				ret.circles.push(circle);
			}

			ret.creatures.get_mut(i).set_path(&path);
			// hero.set_position( &ret.creatures.get(i).get_position() );

			// break;
		}
		println!("DONE!");

		ret.creatures.push(hero);
		ret
	}

	fn logic(&mut self, game : &mut Game, window : &mut RenderWindow, delta : f32) {
		// figure out zoom level
		self.zoom_index = util::clamp(self.zoom_index,0,self.zoom_levels.len() as int-1);
		let mag = self.zoom_levels[self.zoom_index as uint];

		// if no player, enable panning? sure.
		let pan_spd = 16.*16.*delta/mag;

		let go_l = keyboard::is_key_pressed(keyboard::Left);
		let go_r = keyboard::is_key_pressed(keyboard::Right);
		let go_u = keyboard::is_key_pressed(keyboard::Up);
		let go_d = keyboard::is_key_pressed(keyboard::Down);

		let mut pan = Vector2f::new(0.,0.);

		if go_l { pan.x -= pan_spd };
		if go_r { pan.x += pan_spd };
		if go_u { pan.y -= pan_spd };
		if go_d { pan.y += pan_spd };

		self.view.move(&pan);

		let mut player: Option<uint> = None;

		// simple bubble depth sort -- acceptable because after init,
		// creatures swap depth values relatively rarely, and bubble
		// sort only uses O(1) memory ;)
		// TODO insertion sort because it's slightly better
		let mut shuffled = true;
		while shuffled {
			shuffled = false;
			for i in range(0,self.creatures.len()-1) {
				let a = i;
				let b = i+1;
				let apos = self.creatures.get(a).get_position();
				let bpos = self.creatures.get(b).get_position();
				let ay = apos.y;
				let by = bpos.y;
				if by < ay {
					shuffled = true;
					let a_copy = self.creatures.get(a).clone();
					*self.creatures.get_mut(a) = self.creatures.get(b).clone();
					*self.creatures.get_mut(b) = a_copy;

				}
			}
		}

		// find player...
		for i in range(0,self.creatures.len()) {
			if self.creatures.get(i).player {
				player = Some(i);
				break;
			}
		}

		// update player!
		match player {
			None => {}
			Some(hero) => {
				let dist = 64.*delta;

				let angle = match(go_u,go_d,go_l,go_r) {
					(false,false,false,true) => Some(0.),
					(false,true,false,true) => Some(45.),
					(false,true,false,false) => Some(90.),
					(false,true,true,false) => Some(135.),
					(false,false,true,false) => Some(180.),
					(true,false,true,false) => Some(225.),
					(true,false,false,false) => Some(270.),
					(true,false,false,true) => Some(315.),
					_ => None
				};

				match angle {
					None => { }
					Some(deg) => { self.creatures.get_mut(hero).move_polar_deg(dist,deg); }
				}

				// chase player!
				let hero_pos = self.creatures.get(hero).get_position();
				let chase_dist = 16.0 * delta;
				for i in range(0, self.creatures.len()) {
					if i == hero { continue; }

					let monster_pos = self.creatures.get(i).get_position();

					let has_path = self.creatures.get(i).has_path();
					match has_path {
						false => {
							if self.los(&hero_pos,&monster_pos) {
								let pos_dif = hero_pos - monster_pos;
								let (dx,dy) = (pos_dif.x,pos_dif.y);

								self.creatures.get_mut(i).move_polar_rad( chase_dist, dy.atan2(&dx) );
							}
						}
						true => {
							let tsz = self.tile_size as f32;
							let first_coords = self.creatures.get(i).get_target_node().expect("UGH");
							let (tx,ty) = first_coords;
							let (wx,wy) = (
								tx as f32 * tsz,
								ty as f32 * tsz
							);
							let wv = Vector2f::new(wx,wy);


							let pos_dif = wv - monster_pos;
							let dif_len = (pos_dif.x*pos_dif.x + pos_dif.y*pos_dif.y).sqrt();
							if dif_len < chase_dist {
								let mut cr = self.creatures.get_mut(i);
								cr.set_position(&wv);
								cr.pop_path_node();
							} else {
								let (dx,dy) = (pos_dif.x,pos_dif.y);
								self.creatures.get_mut(i).move_polar_rad( chase_dist, dy.atan2(&dx) );
							}
						}
					}
				}


				// // uncommenting this bit will paint "nearby" tiles
				let hero_box = self.creatures.get(hero).get_bounds();
				let active = self.get_active_tiles( &hero_box );

				for data in self.tiles.mut_iter() {
					for sprite in data.sprites.mut_iter() {
						sprite.set_color(&Color::white());
					}
				}

				for coords in active.iter() {
					let idx = self.to_tile_idx(coords.clone()).expect("Aw snap");
					for sprite in self.tiles.get_mut(idx).sprites.mut_iter() {
						sprite.set_color(&Color::green());
					}
				}
			}
		}


		// collision
		let mut hit = CollisionResolver::new();
		for i in range(0,self.creatures.len()) {

			let bounds = self.creatures.get(i).get_bounds();
			let active = self.get_active_tiles( &bounds );

			// creature-creature collision
			for j in range(0, self.creatures.len()) {
				if i == j { continue; }

				let i_box = &self.creatures.get(i).get_bounds();
				let j_box = &self.creatures.get(j).get_bounds();
				let offsets = hit.resolve_weighted(i_box,j_box,0.5);
				match offsets {
					None => {},
					Some(vectors) => {
						let (a,b) = vectors;
						self.creatures.get_mut(i).move(&a);
						self.creatures.get_mut(j).move(&b);
					}
				}
			}

			// creature-wall collision
			for coords in active.iter() {
				let idx = self.to_tile_idx(coords.clone()).expect("Can't collide creature/wall for negative index");
				let data = self.tiles.get(idx);
				if !data.is_passable()	{
					let creature = self.creatures.get_mut(i);
					let offset = hit.resolve_weighted(&data.bounds,&creature.get_bounds(),1.0);
					match offset {
						None => {}
						Some(coords) => {
							let (_,offset) = coords;
							creature.move(&offset);
						}
					}
				}
			}
		}

		// set up screen view
		self.view.set_size2f( (window.get_size().x as f32)/mag,
			(window.get_size().y as f32)/mag);

		match player {
			None => {},
			Some(hero) => self.view.set_center( &self.creatures.get(hero).get_position() )
		}
	}

	fn get_active_tiles(&self, bounds: &FloatRect) -> Vec<(int,int)> {
		let mut active_tiles = Vec::new();

		let top_left = (bounds.left,bounds.top);
		let bottom_right = (bounds.left + bounds.width, bounds.top + bounds.height);

		let top_left_tile = self.to_tile_coords(top_left);
		let bottom_right_tile = self.to_tile_coords(bottom_right);

		let (x1,y1) = top_left_tile;
		let (x2,y2) = bottom_right_tile;

		for y in range(y1,y2+1) {
			for x in range(x1,x2+1) {
				active_tiles.push( (x,y) );
			}
		}

		active_tiles
	}

	fn to_tile_coords(&self, pos: (f32, f32) ) -> (int,int) {
		let (x,y) = pos;
		(self.to_coord_idx(x), self.to_coord_idx(y))
	}

	fn to_coord_idx(&self, coord: f32) -> int {
		let t_size = self.tile_size as f32;
		let t_half = self.tile_size as f32 / 2.0;
		((coord + t_half)/t_size).floor() as int
	}

	fn to_tile_idx(&self, tile_coords: (int,int) ) -> Option<uint> {
		let (x_idx,y_idx) = tile_coords;

		if x_idx < 0 || y_idx < 0 { return None; }

		Some( (x_idx+y_idx*(self.dungeon.width)) as uint)
	}

	fn draw(&self, game : &mut Game, window : &mut RenderWindow) {

		window.set_view( get_rc_resource(self.view.new_copy().expect("Couldn't clone view")) );

		window.clear(&Color::black());

		let mut hero_tile_coords = None;
		let mut hero_pos = None;

		for creature in self.creatures.iter() {
			if creature.player {
				let pos = creature.get_position();
				hero_pos = Some(pos);
				hero_tile_coords = Some(self.to_tile_coords( (pos.x,pos.y) ));
				break;
			}
		}

		for data in self.tiles.iter() {
			for sprite in data.sprites.iter() {
				match hero_tile_coords {
					None => window.draw(sprite),
					Some(coords) => {
						let (x,y) = coords;
						if self.debug || data.seen || self.los_coords(x,y,data.tile.x,data.tile.y) {
							window.draw(sprite);
						}
					}
				}
			}
		}
		if self.debug {
			for line in self.lines.iter() {
				window.draw(line);
			}

			for circle in self.circles.iter() {
				window.draw(circle);
			}
		}
		for creature in self.creatures.iter() {
			match hero_pos {
				None => creature.draw(window),
				Some(ref pos) => {
					if self.debug || self.los(pos,&creature.get_position()) {
						creature.draw(window);
					}
				}
			}
		}
	}

	fn los(&self, a: &Vector2f, b: &Vector2f) -> bool {
		let (ax,ay) = self.to_tile_coords((a.x,a.y));
		let (bx,by) = self.to_tile_coords((b.x,b.y));
		self.los_coords(ax,ay,bx,by)
	}

	fn get_tile_los(&self, coords: (int,int) ) -> bool {
		let idx = self.to_tile_idx( coords ).expect("FAILED getting tile LOS");
		self.tiles.get(idx).is_clear()
	}

	fn los_coords(&self, x1: int, y1: int, x2: int, y2: int) -> bool {
		let mut xstep;
		let mut ystep;
		let mut error;
		let mut error_prev;
		let mut x = x1;
		let mut y = y1;
		let mut dx = x2 - x1;
		let mut dy = y2 - y1;
		let mut points = Vec::new();
		points.push((x1,y1));
		if dx < 0 {
			xstep = -1;
			dx *= -1;
		} else {
			xstep = 1;
		}
		if dy < 0 {
			ystep = -1;
			dy *= -1;
		} else {
			ystep = 1;
		}
		let ddx = 2*dx;
		let ddy = 2*dy;
		if ddx >= ddy {
			error = dx;
			error_prev = error;
			for i in range(0,dx) {
				x += xstep;
				error += ddy;
				if error > ddx {
					y += ystep;
					error -= ddx;
					if error + error_prev < ddx {
						points.push((x,y-ystep));
					} else if error + error_prev > ddx {
						points.push((x-xstep,y));
					} else {
						points.push((x,y-ystep));
						points.push((x-xstep,y));
					}
				}
				points.push((x,y));
				error_prev = error;
			}
		} else {
			error = dy;
			error_prev = error;
			for i in range(0,dy) {
				y += ystep;
				error += ddx;
				if error > ddy {
					x += xstep;
					error -= ddy;
					if error + error_prev < ddy {
						points.push((x-xstep,y));
					} else if error + error_prev > ddy {
						points.push((x,y-ystep));
					} else {
						points.push((x-xstep,y));
						points.push((x,y-ystep));
					}
				}
				points.push((x,y));
				error_prev = error;
			}
		}

		for coords in points.iter() {
			if !self.get_tile_los(*coords) {
				return false;
			}
		}

		true
	}

}

impl Screen for GameplayScreen {

	fn key_press(&mut self, game : &mut Game, window : &mut RenderWindow, key : Key) -> bool {
		// println!("pressed [{}]",key);
		match key {
			keyboard::Comma => {self.zoom_index -= 1;true}
			keyboard::Period => {self.zoom_index += 1;true}
			keyboard::BackSlash => {self.debug = !self.debug;true}
			_ => false
		}
	}

	fn update(&mut self, game : &mut Game, window : &mut RenderWindow, delta : f32) -> Option<~Screen> {
		self.logic(game,window,delta);
		self.draw(game,window);
		None
	}
}

/* Tile Sprite */

struct TileData {
	pub sprites: Vec<Sprite>,
	pub bounds: FloatRect,
	pub tile: Tile,
	pub seen: bool,
}

impl TileData {
	pub fn new(bounds: &FloatRect, tile: &Tile) -> TileData {
		TileData { sprites: Vec::new(), bounds: bounds.clone(), tile: tile.clone(), seen: false }
	}
	pub fn is_passable(&self) -> bool {
		match self.sprites.len() {
			0 => false,
			_ => true
		}
	}
	pub fn is_clear(&self) -> bool {
		self.is_passable()
	}
}

///////////////// utility stuff
// TODO oh god this is so messy

// closure for non-diagonal connections
fn connect_direct (ret: &mut GameplayScreen, x: int, y: int, offset: (int,int)) -> bool {
	let (ox,oy) = offset;
	let x2 = x+ox;
	let y2 = y+oy;
	let idx2_opt = ret.to_tile_idx( (x2, y2) );
	match idx2_opt {
		None => false,
		Some(idx2) => match ret.tiles.get(idx2).is_passable() {
			false => false,
			true => ret.graph.connect_nodes_at(x,y,x2,y2)
		}
	}
}

// closure for diagonal connections
fn connect_diag (ret: &mut GameplayScreen, x: int, y: int, offset: (int,int),
		check1: (int,int), check2: (int,int)) -> bool {
	let check1_idx_opt = ret.to_tile_idx(check1);
	let check2_idx_opt = ret.to_tile_idx(check2);
	match (check1_idx_opt,check2_idx_opt) {
		(Some(check1_idx),Some(check2_idx)) => match (
			ret.tiles.get(check1_idx).is_passable(),
			ret.tiles.get(check2_idx).is_passable()
		) {
			(true,true) => connect_direct(ret,x,y,offset),
			(_,_) => false
		},
		(_,_) => false
	}
}