use rsfml::graphics::rc::Sprite;
use rsfml::system::Vector2f;

#[deriving(Clone)]
pub struct AnimationSet {
	anims: Vec<Animation>,
}

impl AnimationSet {
	pub fn new(anim: &Animation) -> AnimationSet {
		AnimationSet { anims: Vec::new() }
	}
}

pub struct Animation {
	sprites: Vec<Sprite>,
	position: Vector2f,
	origin: Vector2f,
	scale: Vector2f,
	timer: f32,
	length: f32,
	frame: uint,
}

impl Clone for Animation {
	fn clone(&self) -> Animation {
		let mut cl = Animation {
			sprites: Vec::new(),
			position: self.position,
			origin: self.origin,
			scale: self.scale,
			timer: self.timer,
			length: self.length,
			frame: self.frame,
		};
		for sprite in self.sprites.iter() {
			cl.sprites.push(sprite.clone().expect("Anim sprite clone failed"));
		}
		cl
	}
}

impl Animation {
	/* public */
	pub fn new(length: f32) -> Animation {
		let this = Animation {
			sprites: Vec::new(),
			position: Vector2f::new(0.0,0.0),
			origin: Vector2f::new(0.0,0.0),
			scale: Vector2f::new(1.0,1.0),
			timer: 0.0,
			length: length,
			frame: 0,
		};
		this
	}
	pub fn each_sprite(&mut self, func: |&mut Sprite|) {
		for spr in self.sprites.mut_iter() {
			func(spr);
		}
	}
	pub fn update(&mut self, delta: f32) {
		self.timer += delta;
		while self.timer >= self.length {
			self.timer -= self.length;
			self.frame = self.frame + 1;
		}
		self.frame %= self.sprites.len();
	}
	pub fn add_sprite(&mut self, sprite: &Sprite) {
		self.sprites.push(sprite.clone().expect("Couldn't clone sprite for anim"));
		let idx = self.sprites.len()-1;
		self.update_sprite( idx );
	}
	pub fn set_scale2f(&mut self, x: f32, y: f32) {
		self.scale.x = x;
		self.scale.y = y;
		self.update_all_sprites();
	}
	pub fn set_origin2f(&mut self, x: f32, y: f32) {
		self.origin.x = x;
		self.origin.y = y;
		self.update_all_sprites();
	}
	pub fn get_position(&self) -> Vector2f {
		self.position
	}
	pub fn set_position2f(&mut self, x: f32, y: f32) {
		self.position.x = x;
		self.position.y = y;
		self.update_all_sprites();
	}
	pub fn get_current_sprite<'a>(&'a self) -> &'a Sprite {
		self.sprites.get(self.frame)
	}
	/* private */
	fn update_sprite(&mut self, idx: uint) {
		let sprite = self.sprites.get_mut(idx);
		sprite.set_position(&self.position);
		sprite.set_origin(&self.origin);
		sprite.set_scale(&self.scale);
	}
	fn update_all_sprites(&mut self) {
		for i in range(0,self.sprites.len()) {
			self.update_sprite(i);
		}
	}
}