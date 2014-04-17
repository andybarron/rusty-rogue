use std::io::BufferedReader;
use rand::TaskRng;
use rand::Rng;

pub fn random_range(task: &mut TaskRng, lo: int, hi: int) -> int {
	if lo == hi {
		fail!("FFFF");
	} else {
		task.gen_range(lo,hi)
	}
}

pub fn map_range(n: f32, min1: f32, max1: f32, min2: f32, max2: f32, clamp: bool) -> f32 {
	let mut percent = (n - min1)/(max1-min1);
	if clamp {
		if percent < 0. { percent = 0.; }
		else if percent > 1. { percent = 1.; }
	}
	min2 + percent*(max2-min2)
}

pub fn int_from_reader<T: Reader>(reader: &mut BufferedReader<T>) -> Option<int> {
	let input = reader.read_line().ok().expect("Invalid input!");
	let opt = from_str::<int>(input.slice_to(input.len() - 1));
	opt
}