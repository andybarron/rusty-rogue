// TODO custom tile culling so we don't draw the entire level every frame

use rsfml::graphics::RenderWindow;
use rsfml::graphics::View;
use rsfml::graphics::Texture;
use rsfml::graphics::rc::Sprite;
use rsfml::graphics::Color;
use rsfml::graphics::IntRect;
use rsfml::system::Vector2f;
use rsfml::window::keyboard;
use rsfml::window::keyboard::Key;

use engine::{Game,Screen};
use generator::{generate_default,Tile,TileType,Dungeon,Floor,Corridor,Door,StairsUp,StairsDown,Monster};
use util::get_gfx_path;
use util::get_rc_resource;
use util::get_sprite_coords;
use util;

use collision::CollisionResolver;

use entities::Creature;

pub struct GameplayScreen {
	tiles: Vec<Sprite>,
	view: View,
	zoom_index: int,
	zoom_levels: ~[f32],
	creatures: Vec<Creature>,
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
			zoom_index: 1,
			zoom_levels: ~[1.,2.,3.,4.],
			tiles: Vec::new(),
			view: View::new().expect("Failed to create View"),
			creatures: Vec::new(),
		};

		// closure to get tile coordinates from tile x/y index
		// i.e. top left tile is (0,0)
		let t_sz = 16;
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
			for coords in tile_coords.iter() {
				let mut spr = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to create sprite");
				spr.set_texture_rect(coords);
				spr.set_origin2f(t_sz as f32/2.0, t_sz as f32/2.0);
				spr.set_position( &Vector2f::new(x as f32,y as f32) );
				ret.tiles.push(spr);
			}
		}

		// load up player sprite
		let coords_hero = grab_tile_rect(4,8);
		let mut sprite_hero = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to create hero sprite");
		sprite_hero.set_texture_rect(&coords_hero);

		// create player creature
		let mut hero = Creature::new(sprite_hero,10);
		let (start_x, start_y) = dungeon.start_coords;
		hero.pos.x = (start_x*t_sz as int) as f32;
		hero.pos.y = (start_y*t_sz as int) as f32;
		hero.player = true;
		ret.creatures.push(hero);

		// a bunch of monsters
		let coords_slime = grab_tile_rect(1,12);
		let mut sprite_slime = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to load slime sprite");
		sprite_slime.set_texture_rect(&coords_slime);

		for tile in dungeon.tiles.iter() {
			match tile.e {
				Some(Monster) => {
					let mut slime = Creature::new(sprite_slime.clone().expect("Couldn't clone slime sprite"),5);
					slime.pos.x = (tile.x*t_sz as int) as f32;
					slime.pos.y = (tile.y*t_sz as int) as f32;
					ret.creatures.push(slime);
				}
				_ => {}
			}
		}

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
		let mut shuffled = true;
		while shuffled {
			shuffled = false;
			for i in range(0,self.creatures.len()-1) {
				let a = i;
				let b = i+1;
				let ay = self.creatures.get(a).pos.y;
				let by = self.creatures.get(b).pos.y;
				if by < ay {
					shuffled = true;
					let a_copy = self.creatures.get(a).clone();
					*self.creatures.get_mut(a) = self.creatures.get(b).clone();
					*self.creatures.get_mut(b) = a_copy;

				}
			}
		}

		// update creatures
		let mut i = 0;
		for creature in self.creatures.mut_iter() {
			creature.update_sprite();

			if creature.player {
				self.view.set_center(&creature.pos);
				player = Some(i);
			}
			i += 1;
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
					None => { /* hero.pos.x = hero.pos.x.round(); hero.pos.y = hero.pos.y.round(); */ }
					Some(deg) => { self.creatures.get_mut(hero).move_polar(dist,deg); }
				}

				// then do collision
				let mut hit = CollisionResolver::new();
				let hero_box = &self.creatures.get(hero).get_bounds();
				for i in range(0, self.creatures.len()) {
					if i == hero { continue; }

					let creature_box = &self.creatures.get(i).get_bounds();
					let offsets = hit.resolve_weighted(hero_box,creature_box,0.0);
					match offsets {
						None => {},
						Some(vectors) => {
							let (a,b) = vectors;
							println!("{}",offsets);
							self.creatures.get_mut(hero).pos = self.creatures.get(hero).pos + a;
							self.creatures.get_mut(i).pos = self.creatures.get(i).pos + b;
						}
					}
				}
			}
		}

		// set up screen view
		self.view.set_size2f( (window.get_size().x as f32)/mag,
			(window.get_size().y as f32)/mag);
	}

	fn draw(&self, game : &mut Game, window : &mut RenderWindow) {

		window.set_view( get_rc_resource(self.view.new_copy().expect("Couldn't clone view")) );

		window.clear(&Color::black());
		for tile in self.tiles.iter() {
			window.draw(tile);
		}

		for creature in self.creatures.iter() {
			creature.draw(window);
		}
	}
}

impl Screen for GameplayScreen {

	fn key_press(&mut self, game : &mut Game, window : &mut RenderWindow, key : Key) -> bool {
		// println!("pressed [{}]",key);
		match key {
			keyboard::Comma => {self.zoom_index -= 1;true}
			keyboard::Period => {self.zoom_index += 1;true}
			_ => false
		}
	}

	fn update(&mut self, game : &mut Game, window : &mut RenderWindow, delta : f32) -> Option<~Screen> {
		self.logic(game,window,delta);
		self.draw(game,window);
		None
	}
}