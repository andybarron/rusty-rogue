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
use generator::{generate_default,Tile,TileType,Dungeon,Floor,Corridor,Door,StairsUp,StairsDown};
use util::get_gfx_path;
use util::get_rc_resource;
use util::get_sprite_coords;
use util;

pub struct GameplayScreen {
	tiles: Vec<Sprite>,
	view: View,
	mag: f32
}

impl GameplayScreen  {

	pub fn new(dungeon: &Dungeon) -> GameplayScreen {

		let tex_path = get_gfx_path("all_tiles.png");
		let tex = Texture::new_from_file( tex_path ).expect("Failed to load all_tiles.png");

		let rc_tex = get_rc_resource(tex);

		let mut ret = GameplayScreen {
			mag: 2.0,
			tiles: Vec::new(),
			view: View::new().expect("Failed to create View")
		};

		let t_sz = 16;
		let grab_tile_rect = |x: uint, y: uint| -> IntRect {
			let (tx,ty) = get_sprite_coords(x,y,t_sz,t_sz);
			IntRect{ left: tx as i32, top: ty as i32, width: t_sz as i32, height: t_sz as i32 }
		};


		let coords_floor = grab_tile_rect(8,6);
		let coords_door = grab_tile_rect(3,0);
		let coords_hall = grab_tile_rect(7,4);
		let coords_up = grab_tile_rect(9,7);
		let coords_dn = grab_tile_rect(10,7);

		for tile in dungeon.get_tile_vector().iter() {
			let x = tile.x * t_sz as int;
			let y = tile.y * t_sz as int;

			let tile_coords = match tile.t {
				Floor => ~[coords_floor],
				Door => ~[coords_hall,coords_door],
				Corridor => ~[coords_hall],
				StairsUp => ~[coords_floor,coords_up],
				StairsDown => ~[coords_floor,coords_dn],
				_ => ~[]
			};


			for coords in tile_coords.iter() {
				let mut spr = Sprite::new_with_texture(rc_tex.clone()).expect("Failed to create sprite");
				spr.set_texture_rect(coords);
				spr.set_position( &Vector2f::new(x as f32,y as f32) );
				ret.tiles.push(spr);
			}
		}


		ret
	}

	fn logic(&mut self, game : &mut Game, window : &mut RenderWindow, delta : f32) {
		self.mag = util::clamp(self.mag,0.25,4.0);
		let pan_spd = 16.*16.*delta/self.mag;

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
		self.view.set_size2f( (window.get_size().x as f32)/self.mag,
			(window.get_size().y as f32)/self.mag);

	}

	fn draw(&self, game : &mut Game, window : &mut RenderWindow) {

		window.set_view( get_rc_resource(self.view.new_copy().expect("Couldn't clone view")) );

		window.clear(&Color::black());
		for tile in self.tiles.iter() {
			window.draw(tile);
		}
	}
}

impl Screen for GameplayScreen {

	fn key_press(&mut self, game : &mut Game, window : &mut RenderWindow, key : Key) -> bool {
		println!("pressed [{}]",key);
		match key {
			keyboard::Comma => {self.mag /= 2.0;true}
			keyboard::Period => {self.mag *= 2.0;true}
			_ => false
		}
	}

	fn update(&mut self, game : &mut Game, window : &mut RenderWindow, delta : f32) -> Option<~Screen> {
		self.logic(game,window,delta);
		self.draw(game,window);
		None
	}
}