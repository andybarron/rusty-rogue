use rsfml::system::Vector2f;
use rsfml::graphics::FloatRect;
use rsfml::graphics::RenderWindow;
use animation::Animation;

#[deriving(Clone)]
pub enum Facing {
	North = 2,
	South = 0,
	East = 3,
	West = 1,
}

#[deriving(Clone)]
pub struct Creature {
	max_health: int,
	health: int,
	pos: Vector2f,
	anim: Animation,
	pub player: bool,
	path: Vec<(int,int)>,
	pub path_age: f32,
	pub path_id: Option<uint>,
	pub awake: bool,
	pub facing: Facing,
}

impl Creature {

	pub fn new(anim: &Animation, max_health: int) -> Creature {
		// TODO position sprite differently
		let mut c = Creature {
			max_health: max_health,
			health: max_health,
			pos: Vector2f::new(0.0,0.0),
			anim: anim.clone(),
			player: false,
			path: Vec::new(),
			path_age: 0.0,
			path_id: None,
			awake: false,
			facing: South,
		};

		// TODO better sprite origin calculation?
		let bounds = c.anim.get_current_sprite().get_local_bounds();
		c.anim.set_origin2f(bounds.width/2.0,bounds.height/2.0);
		c.update_sprite();

		c
	}

	pub fn update_anim(&mut self, delta: f32) {
		self.anim.update(delta);
	}

	fn update_sprite(&mut self) {
		self.anim.set_position2f( self.pos.x, self.pos.y );
	}

	#[inline]
	pub fn get_bounds(&self) -> FloatRect {
		self.get_bounds_trimmed(0.0)
	}

	// TODO make better
	pub fn get_bounds_trimmed(&self, trim: f32) -> FloatRect {
		let mut bounds = self.anim.get_current_sprite().get_global_bounds();
		let reduce_h = bounds.height / 2.0;
		let reduce_w = bounds.width / 4.0;

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
		window.draw(self.anim.get_current_sprite());
	}

	pub fn set_scale2f(&mut self, x: f32, y: f32) {
		self.anim.set_scale2f(x,y);
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

	pub fn has_path(&self) -> bool {
		self.path.len() > 0
	}

	pub fn pop_path_node(&mut self) -> bool {
		self.path.remove(0).is_some()
	}

	pub fn set_path(&mut self, path: &Vec<(int,int)>) {
		self.path.clear();
		self.path = path.clone();
		self.path_age = 0.0;
	}

	pub fn get_target_node(&self) -> Option<(int,int)> {
		match self.has_path() {
			false => None,
			true => Some(self.path.get(0).clone())
		}
	}

	pub fn get_path(&self) -> Option<Vec<(int,int)>> {
		match self.path.len() {
			0 => None,
			_ => Some(self.path.clone())
		}
	}
	
}

// impl Clone for Creature {
// 	fn clone(&self) -> Creature {
// 		Creature {
// 			max_health: self.max_health,
// 			health: self.health,
// 			pos: self.pos,
// 			anim: self.anim.clone(),
// 			player: self.player,
// 			path: self.path.clone(),
// 			path_age: self.path_age,
// 			path_id: self.path_id.clone(),
// 			awake: self.awake,
// 			facing: self.facing,
// 		}
// 	}
// }