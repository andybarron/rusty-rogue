use std::iter::Range;
use collections::hashmap::HashMap;

use sync::{Arc,RWLock};

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

use engine::{Game,Screen};
use generator::{Tile,Dungeon,Floor,Corridor,Door,StairsUp,StairsDown,Monster};
use util::get_gfx_path;
use util::get_rc_resource;
use util::get_sprite_coords;
use util;

use collision::CollisionResolver;

use entities::Creature;
use animation::Animation;

use graph::Graph;
use solver::Solver;

static SOLVER_THREAD_COUNT : uint = 4;

pub struct GameplayScreen {
	tile_size: uint,
	tile_sizef: f32,
	dungeon: Dungeon,
	graph: Arc<RWLock<Graph>>,
	tiles: Vec<TileData>,
	view: View,
	zoom_index: int,
	zoom_levels: ~[f32],
	creatures: Vec<Creature>,
	debug: bool,
	debug_node_circle: CircleShape,
	solvers: Vec<Solver>,
	path_count: uint,
	vis_x: Range<int>,
	vis_y: Range<int>,
	player_idx: Option<uint>,
	collide: CollisionResolver,
}

impl GameplayScreen  {

	pub fn new(dungeon: &Dungeon) -> GameplayScreen {

		let mut dungeon = dungeon.clone();
		dungeon.shrink();
		// load tile texture file
		let tex_path = get_gfx_path("all_tiles.png");
		let tex = Texture::new_from_file( tex_path ).expect("Failed to load all_tiles.png");

		// get refcounted version for rc::Sprite
		let rc_tex = get_rc_resource(tex);

		let tsz_init = 16;
		let debug_node_radius = tsz_init as f32 / 4.0;

		// init screen
		let mut ret = GameplayScreen {
			tile_size: tsz_init,
			tile_sizef: tsz_init as f32,
			dungeon: dungeon.clone(),
			graph: Arc::new( RWLock::new( Graph::new() ) ),
			zoom_index: 1,
			zoom_levels: ~[1.,2.,3.,4.],
			tiles: Vec::new(),
			view: View::new().expect("Failed to create View"),
			creatures: Vec::new(),
			debug: false,
			debug_node_circle: CircleShape::new_init(debug_node_radius, 8).expect("Failed to make debug node circle"),
			solvers: Vec::new(),
			path_count: 0,
			vis_x: range(0,1),
			vis_y: range(0,1),
			player_idx: None,
			collide: CollisionResolver::new(),
		};
		ret.debug_node_circle.set_origin2f(debug_node_radius,debug_node_radius);
		ret.debug_node_circle.set_fill_color( &Color::new_RGBA(0,0,255,150) );

		for _ in range (0,SOLVER_THREAD_COUNT) {
			ret.solvers.push(Solver::new());
		}

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
				ret.graph.write().add_node_at(x,y);
			}
		}

		println!("Starting graph node loop...");
		// loop through the graph and
		// connect accessible nodes
		for y in range(0,dungeon.height) {
			for x in range(0,dungeon.width) {
				let idx_opt = ret.tile_idx_from_coords( (x, y) );
				let idx = idx_opt.expect("Shouldn't be negative");
				match ret.tiles.get(idx).is_passable() {
					false => {}
					true => {
						// only check R, DR, D, DL
						// yay undirected graphs!

						// check R and D
						ret.connect_direct(x,y,(1,0));
						ret.connect_direct(x,y,(0,1));

						// diagonal
						ret.connect_diag(x,y,(1,1),(x+1,y),(x,y+1));
						ret.connect_diag(x,y,(-1,1),(x-1,y),(x,y+1));
					}
				}
			}
		}
		println!("Done with graph!");

		let get_spr = |x: uint, y: uint| -> Sprite {
			let coords = grab_tile_rect(x,y);
			let mut spr = Sprite::new_with_texture(rc_tex.clone()).expect("erp");
			spr.set_texture_rect(&coords);
			spr
		};

		let get_walk_cycle = |x: uint, y: uint, length: f32| -> Animation {
			let spr_m = get_spr(x,y);
			let spr_l = get_spr(x-1,y);
			let spr_r = get_spr(x+1,y);
			let mut anim = Animation::new(length);
			anim.add_sprite(&spr_m);
			anim.add_sprite(&spr_l);
			anim.add_sprite(&spr_m);
			anim.add_sprite(&spr_r);
			anim
		};


		// load up player sprite
		let coords_hero = grab_tile_rect(4,8);
		let mut sprite_hero = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to create hero sprite");
		sprite_hero.set_texture_rect(&coords_hero);

		let mut anim_hero = Animation::new(1.0);
		anim_hero.add_sprite(&sprite_hero);

		// create player creature
		let mut hero = Creature::new(&get_walk_cycle(4,8,0.125),10);
		let (start_x, start_y) = dungeon.start_coords;
		hero.set_position2f( (start_x*t_sz as int) as f32, (start_y*t_sz as int) as f32 );
		hero.player = true;



		// a bunch of monsters
		let coords_slime = grab_tile_rect(10,8);
		let mut sprite_slime = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to load slime sprite");
		sprite_slime.set_texture_rect(&coords_slime);
		let mut anim_slime = Animation::new(1.0);
		anim_slime.add_sprite(&sprite_slime);

		let monster_cycles = [
			get_walk_cycle(10,8,0.5),
			get_walk_cycle(10,12,0.5),
			get_walk_cycle(7,12,0.5),
			get_walk_cycle(4,12,0.5),
			// get_walk_cycle(1,12,0.5), // slime
		];

		// find and create monsters
		for tile in dungeon.tiles.iter() {
			match tile.e {
				Some(Monster(num)) => {
					let idx = num % monster_cycles.len();
					let mut slime = Creature::new(&monster_cycles[idx],5);
					slime.set_position2f( (tile.x*t_sz as int) as f32, (tile.y*t_sz as int) as f32 );
					ret.creatures.push(slime);
				}
				_ => {}
			}
		}

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

		// depth sort
		self.sprite_depth_sort();

		// find player if necessary
		let player = match self.player_idx {
			Some(idx) => Some(idx),
			None => {
				let mut fnd = None;
				for i in range(0,self.creatures.len()) {
					if self.creatures.get(i).player {
						fnd = Some(i);
						break;
					}
				}
				fnd
			}
		};

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

				// get solutions
				let mut path_map: HashMap<uint,Vec<(int,int)>> = HashMap::new();
				for solver in self.solvers.mut_iter() {
					loop {
						match solver.poll() {
							None => break,
							Some(soln) => {
								let id = soln.id;
								let path = match soln.path {
									None => Vec::new(),
									Some(path) => path
								};
								path_map.insert(id,path);
							}
						}
					}
				}

				// chase player!
				let hero_pos = self.creatures.get(hero).get_position();
				for i in range(0, self.creatures.len()) {
					if i == hero { continue; }

					let monster_pos = self.creatures.get(i).get_position();

					let sees_player = self.los(&hero_pos,&monster_pos);

					let path_id = self.creatures.get(i).path_id;

					let searching_path = self.creatures.get(i).path_id.is_some();

					match path_id {
						None => {}
						Some(ref id) => {
							let path_opt = path_map.pop(id);
							match path_opt {
								None => {},
								Some(ref path) => {
									self.creatures.get_mut(i).path_id = None;
									//match path.len() {
										//0 => {},
										//_ => {
											self.creatures.get_mut(i).set_path(path);
											self.creatures.get_mut(i).pop_path_node();
										//}
									//}
								}
							}
						}
					}
					let has_path = self.creatures.get(i).has_path();

					let req_path = sees_player &&
						( (!has_path) || (!searching_path && self.creatures.get(i).path_age > 0.25) );

					if req_path {
						self.request_path_update(i,hero);
					}

					// TODO reconcile this with collision somehow
					if has_path {
						self.creatures.get_mut(i).path_age += delta;
						let tsz = self.tile_size as f32;
						let first_coords = self.creatures.get(i).get_target_node().expect("UGH");
						let (tx,ty) = first_coords;
						let (wx,wy) = (
							tx as f32 * tsz,
							ty as f32 * tsz
						);
						let wv = Vector2f::new(wx,wy);

						let chase_dist = 16.0 * delta;
						let mut dist_remaining = chase_dist;

						while dist_remaining > 0.0 && self.creatures.get(i).has_path() {
							let pos_dif = wv - monster_pos;
							let dif_len = (pos_dif.x*pos_dif.x + pos_dif.y*pos_dif.y).sqrt();
							if dif_len < dist_remaining {
								let cr = self.creatures.get_mut(i);
								cr.set_position(&wv);
								cr.pop_path_node();
								dist_remaining -= dif_len;
							} else {
								let (dx,dy) = (pos_dif.x,pos_dif.y);
								self.creatures.get_mut(i).move_polar_rad( chase_dist, dy.atan2(&dx) );
								dist_remaining = 0.0;
							}
						}
					}
				}
			}
		}


		// collision
		self.resolve_all_collisions();

		// updates
		for creature in self.creatures.mut_iter() {
			creature.update_anim(delta);
		}

		// set up screen view
		self.view.set_size2f( (window.get_size().x as f32)/mag,
			(window.get_size().y as f32)/mag);

		match player {
			None => {},
			Some(hero) => self.view.set_center( &self.creatures.get(hero).get_position() )
		}



		// figure out visible tiles

		let view_size = self.view.get_size();
		let view_center = self.view.get_center();
		let view_half = view_size / 2.0f32;

		let view_left = view_center.x - view_half.x;
		let view_right = view_center.x + view_half.x;
		let view_top = view_center.y - view_half.y;
		let view_bottom = view_center.y + view_half.y;

		let coord_left = self.tile_coord_from_position(view_left);
		let coord_right = self.tile_coord_from_position(view_right);
		let coord_top = self.tile_coord_from_position(view_top);
		let coord_bottom = self.tile_coord_from_position(view_bottom);

		self.vis_x = range(coord_left,coord_right + 1);
		self.vis_y = range(coord_top,coord_bottom + 1);

		for y in self.vis_y.clone() {
			for x in self.vis_x.clone() {
				match self.tile_idx_from_coords((x,y)) {
					None => {},
					Some(idx) => {
						// set visibility based on player
						match player {
							None => {
								let t = self.tiles.get_mut(idx);
								t.seen = true;
								t.visible = true;
							}
							Some(hero) => {
								let hero_pos = self.creatures.get(hero).get_position();
								let hero_coords = self.tile_coords_from_position((hero_pos.x,hero_pos.y));
								let (hero_x,hero_y) = hero_coords;
								let t = self.tiles.get(idx).tile;
								
								if self.los_coords(hero_x,hero_y,t.x,t.y) {
									let data = self.tiles.get_mut(idx);
									data.seen = true;
									data.visible = true;
								} else {
									let data = self.tiles.get_mut(idx);
									data.visible = false;
								}
							}
						}
						// color
						let t = self.tiles.get_mut(idx);
						let color = if t.visible || self.debug {
							Color::white()
						} else {
							Color::new_RGB(100,75,75)
						};
						for spr in t.sprites.mut_iter() {spr.set_color(&color);}
					}
				}
			}
		}
	}

	fn request_path_update(&mut self, i: uint, hero: uint) {
		let id = self.path_count;
		self.path_count += 1;
		self.creatures.get_mut(i).path_id = Some(id);
		self.creatures.get_mut(i).awake = true;
		let hero_coords = self.tile_coords_from_creature(self.creatures.get(hero));
		let rawr_coords = self.tile_coords_from_creature(self.creatures.get(i));
		let solver_idx = i % self.solvers.len();
		self.solvers.get_mut(solver_idx).queue_solve(
			id,
			self.graph.clone(),
			rawr_coords,
			hero_coords
		);
	}

	fn get_active_tiles(&self, bounds: &FloatRect) -> Vec<(int,int)> {
		let mut active_tiles = Vec::new();

		let top_left = (bounds.left,bounds.top);
		let bottom_right = (bounds.left + bounds.width, bounds.top + bounds.height);

		let top_left_tile = self.tile_coords_from_position(top_left);
		let bottom_right_tile = self.tile_coords_from_position(bottom_right);

		let (x1,y1) = top_left_tile;
		let (x2,y2) = bottom_right_tile;

		for y in range(y1,y2+1) {
			for x in range(x1,x2+1) {
				active_tiles.push( (x,y) );
			}
		}

		active_tiles
	}

	fn tile_coords_from_creature(&self, creature: &Creature) -> (int,int) {
		let pos = creature.get_position();
		self.tile_coords_from_position((pos.x,pos.y))
	}

	fn tile_coords_from_position(&self, pos: (f32, f32) ) -> (int,int) {
		let (x,y) = pos;
		(self.tile_coord_from_position(x), self.tile_coord_from_position(y))
	}

	fn tile_coord_from_position(&self, coord: f32) -> int {
		let t_size = self.tile_size as f32;
		let t_half = self.tile_size as f32 / 2.0;
		((coord + t_half)/t_size).floor() as int
	}

	fn tile_data_from_coords<'a>(&'a self, tile_coords: (int,int)) -> Option<&'a TileData> {
		match self.tile_idx_from_coords(tile_coords) {
			None => None,
			Some(idx) => Some(self.tiles.get(idx))
		}
	}

	fn tile_idx_from_coords(&self, tile_coords: (int,int) ) -> Option<uint> {
		let (x_idx,y_idx) = tile_coords;

		if x_idx < 0 || y_idx < 0 || x_idx >= self.dungeon.width
			|| y_idx >= self.dungeon.height
		{ return None; }

		let idx = (x_idx+y_idx*(self.dungeon.width)) as uint;

		if idx >= self.tiles.len() { return None; }

		Some( idx )
	}

	fn sprite_depth_sort(&mut self) {
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
				let apos = self.creatures.get(a).get_bounds();
				let bpos = self.creatures.get(b).get_bounds();
				let ay = apos.top + apos.height;
				let by = bpos.top + bpos.height;
				if by < ay {
					shuffled = true;
					let a_copy = self.creatures.get(a).clone();
					*self.creatures.get_mut(a) = self.creatures.get(b).clone();
					*self.creatures.get_mut(b) = a_copy;

				}
			}
		}
	}

	fn resolve_all_collisions(&mut self) {
		for i in range(0,self.creatures.len()) {

			let bounds = self.creatures.get(i).get_bounds();
			let active = self.get_active_tiles( &bounds );

			// creature-creature collision
			for j in range(0, self.creatures.len()) {
				if i == j { continue; }

				let i_box = &self.creatures.get(i).get_bounds();
				let j_box = &self.creatures.get(j).get_bounds();
				let offsets = self.collide.resolve_weighted(i_box,j_box,0.5);
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
				let idx = self.tile_idx_from_coords(coords.clone())
					.expect("Can't collide creature/wall for negative index");
				let data = self.tiles.get(idx);
				if !data.is_passable()	{
					let offset = self.collide.resolve_weighted(&data.bounds,&self.
							creatures.get(i).get_bounds(),1.0);
					match offset {
						None => {}
						Some(coords) => {
							let (_,offset) = coords;
							let (tx,ty) = (data.tile.x,data.tile.y);
							let check = if offset.x > 0.0 {
								Some((tx+1,ty))
							} else if offset.x < 0.0 {
								Some((tx-1,ty))
							} else if offset.y > 0.0 {
								Some((tx,ty+1))
							} else if offset.y < 0.0 {
								Some((tx,ty-1))
							} else {
								None
							};

							match check {
								None => {},
								Some(check_coords) => {
									if !active.contains(&check_coords) ||
											self.tile_data_from_coords(check_coords).
											expect("hunH?!").is_passable() {
										self.creatures.get_mut(i).move(&offset);
									}
								}
							}

						}
					}
				}
			}
		}
	}

	fn draw(&self, game : &mut Game, window : &mut RenderWindow) {

		window.set_view( get_rc_resource(self.view.new_copy().expect("Couldn't clone view")) );

		window.clear(&Color::black());

		let mut hero_pos = None;

		for creature in self.creatures.iter() {
			if creature.player {
				let pos = creature.get_position();
				hero_pos = Some(pos);
				break;
			}
		}

		for y in self.vis_y.clone() {
			for x in self.vis_x.clone() {
				match self.tile_idx_from_coords((x,y)) {

					None => {}

					Some(idx) => {
						let data = self.tiles.get(idx);
						// first draw sprites
						if data.visible || data.seen || self.debug {
							for sprite in data.sprites.iter() {
								window.draw(sprite);
							}
						}
						// then maybe nodes
						if data.is_passable() && self.debug {
							let mut circle = self.debug_node_circle.clone().expect("Couldn't clone debug circle");
							circle.set_position2f(
								data.tile.x as f32 * self.tile_size as f32,
								data.tile.y as f32 * self.tile_size as f32
							);
							window.draw(&circle);
						}
					}

				}
			}
		}


		for creature in self.creatures.iter() {
			match hero_pos {
				None => creature.draw(window),
				Some(ref pos) => {
					if self.debug || self.los(pos,&creature.get_position()) {
						creature.draw(window);
						if self.debug {
							match creature.get_path() {
								None => {}
								Some(path) => {
									// draw_line(&mut self, window: &mut RenderWindow,
									// start: &Vector2f, end: &Vector2f, color: &Color)

									// draw red line from creature to current target node
									let (start_tx,start_ty) = *path.get(0);
									let start_wx = start_tx as f32 * self.tile_sizef;
									let start_wy = start_ty as f32 * self.tile_sizef;
									let cpos = creature.get_position();
									let npos = Vector2f::new(start_wx,start_wy);
									game.draw_line(
										window,
										&cpos,
										&npos,
										&Color::red()
									);

									// if path len > 1, connect all nodes
									if path.len() > 1 {
										for i in range(0,path.len()-1) {
											let (atx,aty) = *path.get(i);
											let (btx,bty) = *path.get(i+1);
											let t = self.tile_sizef;
											let (awx,awy) = (atx as f32*t,aty as f32*t);
											let (bwx,bwy) = (btx as f32*t,bty as f32*t);
											let apos = Vector2f::new(awx,awy);
											let bpos = Vector2f::new(bwx,bwy);
											game.draw_line(
												window,
												&apos,
												&bpos,
												&Color::new_RGBA( 255,255,255,150 )
											);
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}

	fn los(&self, a: &Vector2f, b: &Vector2f) -> bool {
		let (ax,ay) = self.tile_coords_from_position((a.x,a.y));
		let (bx,by) = self.tile_coords_from_position((b.x,b.y));
		self.los_coords(ax,ay,bx,by)
	}

	fn get_tile_los(&self, coords: (int,int) ) -> bool {
		let idx = self.tile_idx_from_coords( coords ).expect( format!("FAILED getting tile LOS {}",coords));
		self.tiles.get(idx).is_clear()
	}

	// TODO better LOS algorithm
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
			for _ in range(0,dx) {
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
			for _ in range(0,dy) {
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
	pub visible: bool,
}

impl TileData {
	pub fn new(bounds: &FloatRect, tile: &Tile) -> TileData {
		TileData { sprites: Vec::new(), bounds: bounds.clone(),
			tile: tile.clone(), seen: false, visible: false }
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

impl GameplayScreen {

	// closure for non-diagonal connections
	fn connect_direct (&mut self, x: int, y: int, offset: (int,int)) -> bool {
		let (ox,oy) = offset;
		let x2 = x+ox;
		let y2 = y+oy;
		let idx2_opt = self.tile_idx_from_coords( (x2, y2) );
		match idx2_opt {
			None => false,
			Some(idx2) => match self.tiles.get(idx2).is_passable() {
				false => false,
				true => self.graph.write().connect_nodes_at(x,y,x2,y2)
			}
		}
	}

	// closure for diagonal connections
	fn connect_diag (&mut self, x: int, y: int, offset: (int,int),
			check1: (int,int), check2: (int,int)) -> bool {
		let check1_idx_opt = self.tile_idx_from_coords(check1);
		let check2_idx_opt = self.tile_idx_from_coords(check2);
		match (check1_idx_opt,check2_idx_opt) {
			(Some(check1_idx),Some(check2_idx)) => match (
				self.tiles.get(check1_idx).is_passable(),
				self.tiles.get(check2_idx).is_passable()
			) {
				(true,true) => self.connect_direct(x,y,offset),
				(_,_) => false
			},
			(_,_) => false
		}
	}

}