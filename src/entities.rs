use rsfml::system::Vector2f;
use rsfml::graphics::rc::Sprite;

pub struct Creature {
	max_health: int,
	health: int,
	pub pos: Vector2f,
	pub sprite: Sprite, // TODO some kind of Animation class
	pub player: bool,
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
		};

		// TODO better sprite origin calculation?
		let bounds = c.sprite.get_local_bounds();
		c.sprite.set_origin2f(bounds.width/2.0,bounds.height/2.0);

		c
	}

	pub fn update_sprite(&mut self) {
		self.sprite.set_position2f( self.pos.x, self.pos.y );
	}

	pub fn move_polar(&mut self, distance: f32, angle: f32) { // degrees, i guess 
		self.pos.x += distance*angle.to_radians().cos();
		self.pos.y += distance*angle.to_radians().sin();
	}
}

impl Clone for Creature {
	fn clone(&self) -> Creature {
		Creature {
			max_health: self.max_health,
			health: self.health,
			pos: self.pos,
			sprite: self.sprite.clone().expect("Creature.clone() failed to clone sprite"),
			player: self.player
		}
	}
}