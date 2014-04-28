use rsfml::system::Vector2f;
use rsfml::graphics::rc::Sprite;
use rsfml::graphics::FloatRect;
use rsfml::graphics::RenderWindow;

pub struct Creature {
	max_health: int,
	health: int,
	pos: Vector2f,
	sprite: Sprite, // TODO some kind of Animation class
	pub player: bool,
	pub path: Option<Vec<(int,int)>>
}

impl Creature {

	pub fn new(sprite: Sprite, max_health: int) -> Creature {
		// TODO position sprite differently
		let mut c = Creature {
			max_health: max_health,
			health: max_health,
			pos: Vector2f::new(0.0,0.0),
			sprite: sprite,
			player: false,
			path: None,
		};

		// TODO better sprite origin calculation?
		let bounds = c.sprite.get_local_bounds();
		c.sprite.set_origin2f(bounds.width/2.0,bounds.height/2.0);
		c.update_sprite();

		c
	}

	fn update_sprite(&mut self) {
		self.sprite.set_position2f( self.pos.x, self.pos.y );
	}

	#[inline]
	pub fn get_bounds(&self) -> FloatRect {
		self.get_bounds_trimmed(0.0)
	}

	pub fn get_bounds_trimmed(&self, trim: f32) -> FloatRect {
		let mut bounds = self.sprite.get_global_bounds();
		let mut reduce_h = bounds.height / 2.0;
		let mut reduce_w = bounds.width / 4.0;

		bounds.height -= reduce_h;
		bounds.top += reduce_h;
		bounds.width -= reduce_w;
		bounds.left += reduce_w/2.0;

		if trim != 0.0 {
			bounds.left += trim;
			bounds.top += trim;
			bounds.width -= trim;
			bounds.height -= trim;
		}

		bounds
	}

	pub fn move_polar_deg(&mut self, distance: f32, degrees: f32) {
		self.move_polar_rad(distance, degrees.to_radians())
	}

	pub fn move_polar_rad(&mut self, distance: f32, radians: f32) {
		self.pos.x += distance*radians.cos();
		self.pos.y += distance*radians.sin();
		self.update_sprite();
	}

	pub fn draw(&self, window: &mut RenderWindow) {
		window.draw(&self.sprite);
	}

	pub fn set_position2f(&mut self, x: f32, y: f32) {
		self.pos.x = x;
		self.pos.y = y;
		self.update_sprite();
	}

	pub fn set_position(&mut self, position: &Vector2f) {
		self.pos = position.clone();
		self.update_sprite();
	}

	pub fn move(&mut self, dist: &Vector2f) {
		self.pos = self.pos + *dist;
		self.update_sprite();
	}

	pub fn get_position(&self) -> Vector2f {
		self.pos
	}

	pub fn pop_path_node(&mut self) -> bool {
		match self.path {
			None => false,
			Some(ref mut path) => {
				path.remove(0).is_some()
			}
		}
	}
}

impl Clone for Creature {
	fn clone(&self) -> Creature {
		Creature {
			max_health: self.max_health,
			health: self.health,
			pos: self.pos,
			sprite: self.sprite.clone().expect("Creature.clone() failed to clone sprite"),
			player: self.player,
			path: self.path.clone()
		}
	}
}