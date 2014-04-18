use rsfml::graphics::RenderWindow;
use rsfml::graphics::Texture;
use rsfml::graphics::rc::Sprite;
use rsfml::graphics::Color;

use engine::{Game,Screen};
use util::get_gfx_path;
use util::get_rc_resource;

pub struct GameplayScreen {
	spr: Sprite
}

impl GameplayScreen  {

	pub fn new() -> GameplayScreen {

		let tex_path = get_gfx_path("all_tiles.png");
		let tex = Texture::new_from_file( tex_path ).expect("Failed to load all_tiles.png");

		let rc_tex = get_rc_resource(tex);

		let mut ret = GameplayScreen {
			spr: Sprite::new_with_texture(rc_tex).expect("Sprite failure")
		};

		ret
	}

	fn logic(&mut self, game : &mut Game, window : &mut RenderWindow, delta : f32) {

	}

	fn draw(&mut self, game : &mut Game, window : &mut RenderWindow) {
		window.clear(&Color::black());
		window.draw(&self.spr);
	}
}

impl Screen for GameplayScreen {

	fn update(&mut self, game : &mut Game, window : &mut RenderWindow, delta : f32) -> Option<~Screen> {
		self.logic(game,window,delta);
		self.draw(game,window);
		None
	}
}