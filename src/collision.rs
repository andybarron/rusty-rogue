use rsfml::graphics::FloatRect;
use rsfml::system::Vector2f;

pub struct CollisionResolver {
	overlap: FloatRect,
}

impl CollisionResolver {
	pub fn new() -> CollisionResolver {
		CollisionResolver {
			overlap: FloatRect::new(0.0,0.0,0.0,0.0),
		}
	}

	/// 0.0 moves only a, 0.5 moves both equally, 1.0 moves only b
	pub fn resolve_weighted(&mut self, a: &FloatRect, b: &FloatRect, weight: f32) -> Option<(Vector2f, Vector2f)> {
		match FloatRect::intersects(a,b,&self.overlap) {
			false => None,
			true => {
				let mult_a = 1.0-weight;
				let mult_b = weight;

				let overlap = &self.overlap;
				// println!("Overlap: {}",overlap);
				let ow = overlap.width;
				let oh = overlap.height;

				if oh > ow {

					let mut offset_a = ow*mult_a;
					let mut offset_b = ow*mult_b;

					let center_x_a = a.left + a.width/2.0;
					let center_x_b = b.left + b.width/2.0;

					if center_x_a < center_x_b {
						offset_a *= -1.0;
					} else {
						offset_b *= -1.0;
					}


					Some((Vector2f::new(offset_a,0.0),
						Vector2f::new(offset_b,0.0)))



				} else {


					let mut offset_a = oh*mult_a;
					let mut offset_b = oh*mult_b;

					let center_y_a = a.top + a.height/2.0;
					let center_y_b = b.top + b.height/2.0;

					if center_y_a < center_y_b {
						offset_a *= -1.0;
					} else {
						offset_b *= -1.0;
					}


					Some((Vector2f::new(0.0,offset_a),
						Vector2f::new(0.0,offset_b)))
				}

			}
		}
	}
}