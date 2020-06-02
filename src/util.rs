use std::cell::RefCell;
use std::cmp::PartialOrd;
use std::rc::Rc;

pub static PI: f32 = ::std::f32::consts::PI;

pub trait AngleHelper {
	fn to_deg(self) -> Self;
	fn to_rad(self) -> Self;
}

impl AngleHelper for f32 {
	fn to_deg(self) -> Self {
		self * 180. / PI
	}
	fn to_rad(self) -> Self {
		self * PI / 180.
	}
}

static RES_LOC: &'static str = "./res/";
static GFX_DIR: &'static str = "gfx/";
static SND_DIR: &'static str = "snd/";

/* Resource management */

pub fn get_gfx_path(fname: &str) -> String {
	RES_LOC.to_string() + GFX_DIR.into() + fname.into()
}

pub fn get_snd_path(fname: &str) -> String {
	RES_LOC.to_string() + SND_DIR.into() + fname.into()
}

pub fn get_rc_resource<T>(resource: T) -> Rc<RefCell<T>> {
	Rc::new(RefCell::new(resource))
}

// TODO add gutter/separation option
pub fn get_sprite_coords(x: usize, y: usize, tile_w: usize, tile_h: usize) -> (usize, usize) {
	(x * tile_w, y * tile_h)
}

/* Angular math */

pub fn circle(radians: bool) -> f32 {
	if (radians) {
		2. * ::std::f32::consts::PI
	} else {
		360.0
	}
}

pub fn normalize_angle(angle: f32, radians: bool) -> f32 {
	let circle = circle(radians);
	let mut norm = angle;
	while norm >= circle {
		norm -= circle;
	}
	while norm < 0.0 {
		norm += circle;
	}
	norm
}

pub fn get_angle_diff(current: f32, target: f32) -> f32 {
	let mut diff = target - current;

	while diff < 0. {
		diff += 360.;
	}
	while diff >= 360. {
		diff -= 360.
	}

	if diff > 180. {
		diff -= 360.;
	}

	diff
}

/* Random stuff */

pub fn map_range_f32(n: f32, min1: f32, max1: f32, min2: f32, max2: f32, clamp_val: bool) -> f32 {
	let raw_percent = (n - min1) / (max1 - min1);
	let percent = if clamp_val {
		clamp(raw_percent, 0., 1.)
	} else {
		raw_percent
	};
	min2 + percent * (max2 - min2)
}

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
	if max < min {
		panic!("Clamp value mismatch");
	} else if max < x {
		max
	} else if x < min {
		min
	} else {
		x
	}
}
