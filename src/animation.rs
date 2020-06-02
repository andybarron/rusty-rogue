use sfml::graphics::IntRect;
use sfml::graphics::Sprite;
use std::rc::Rc;

#[derive(Clone)]
pub struct Animation<'a> {
	pub sprite: Rc<Sprite<'a>>,
	frame_set: usize,
	pub frame_sets: Vec<Vec<IntRect>>,
	pub timer: f32,
	pub length: f32,
	frame: usize,
}

// impl Clone for Animation {
// 	fn clone(&self) -> Animation {
// 		Animation {
// 			sprite: self.sprite.clone().expect("Anim sprite clone failed"),
// 			frame_set: self.frame_set,
// 			frame_sets: self.frame_sets.clone(),
// 			timer: self.timer,
// 			length: self.length,
// 			frame: self.frame,
// 		}
// 	}
// }

impl<'a> Animation<'a> {
	/* public */
	pub fn new(sprite: &Rc<Sprite<'a>>, frames: &Vec<IntRect>, length: f32) -> Animation<'a> {
		Animation {
			sprite: sprite.clone(),
			frame_sets: vec![frames.clone()],
			frame_set: 0,
			timer: 0.0,
			length: length,
			frame: 0,
		}
	}
	pub fn update(&mut self, delta: f32) {
		self.timer += delta;
		let frame_count = self.frame_sets[self.frame_set].len();
		let length = self.length / frame_count as f32;
		while self.timer >= length {
			self.timer -= length;
			self.frame = (self.frame + 1) % frame_count;
			self.update_rect();
		}
	}
	pub fn set_frame_set(&mut self, idx: usize) {
		self.frame_set = idx;
		self.update_rect();
	}
	/* private */
	fn update_rect(&mut self) {
		self.sprite
			.set_texture_rect(&self.frame_sets[self.frame_set][self.frame]);
	}
}
