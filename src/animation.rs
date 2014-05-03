use rsfml::graphics::rc::Sprite;
use rsfml::system::Vector2f;
use rsfml::graphics::IntRect;

pub struct Animation {
	pub sprite: Sprite,
	frame_set: uint,
	pub frame_sets: Vec<Vec<IntRect>>,
	pub timer: f32,
	pub length: f32,
	frame: uint,
}

impl Clone for Animation {
	fn clone(&self) -> Animation {
		Animation {
			sprite: self.sprite.clone().expect("Anim sprite clone failed"),
			frame_set: self.frame_set,
			frame_sets: self.frame_sets.clone(),
			timer: self.timer,
			length: self.length,
			frame: self.frame,
		}
	}
}

impl Animation {
	/* public */
	pub fn new(sprite: &Sprite, frames: &Vec<IntRect>, length: f32) -> Animation {
		Animation {
			sprite: sprite.clone().expect("Anim sprite init failed"),
			frame_sets: vec!(frames.clone()),
			frame_set: 0,
			timer: 0.0,
			length: length,
			frame: 0,
		}
	}
	pub fn update(&mut self, delta: f32) {
		self.timer += delta;
		let frame_count = self.frame_sets.get(self.frame_set).len();
		let length = self.length / frame_count as f32;
		while self.timer >= length {
			self.timer -= length;
			self.frame = (self.frame + 1) % frame_count;
			self.update_rect();
		}
	}
	pub fn set_frame_set(&mut self, idx: uint) {
		self.frame_set = idx;
		self.update_rect();
	}
	/* private */
	fn update_rect(&mut self) {
		self.sprite.set_texture_rect(
			self.frame_sets.get(self.frame_set).get(
				self.frame
			)
		);
	}
}