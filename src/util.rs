use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ord;
use std::num::Float;

static RES_LOC: &'static str = "./res/";
static GFX_DIR: &'static str = "gfx/";
static SND_DIR: &'static str = "snd/";

/* Resource management */

pub fn get_gfx_path(fname: &str) -> ~str {
	RES_LOC.clone() + GFX_DIR.clone() + fname.clone()
}

pub fn get_snd_path(fname: &str) -> ~str {
	RES_LOC.clone() + SND_DIR.clone() + fname.clone()
}

pub fn get_rc_resource<T>(resource : T) -> Rc<RefCell<T>> {
	Rc::new(RefCell::new(resource))
}

// TODO add gutter/separation option
pub fn get_sprite_coords(x: uint, y: uint, tile_w: uint, tile_h: uint) -> (uint,uint) {
	( x*tile_w, y*tile_h )
}

/* Angular math */

pub fn circle(radians: bool) -> f32 {
	if (radians) { Float::two_pi() } else { 360.0 }
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

pub fn get_angle_diff( current: f32, target: f32 ) -> f32 {
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
	let raw_percent = (n - min1)/(max1-min1);
	let percent = if clamp_val { clamp(raw_percent, 0., 1.) } else { raw_percent };
	min2 + percent*(max2-min2)
}

pub fn clamp<T:Ord>(x : T, min : T, max : T) -> T {
	if max < min {
		fail!("Clamp value mismatch");
	} else if max < x {
		max
	} else if x < min {
		min
	} else {
		x
	}
}