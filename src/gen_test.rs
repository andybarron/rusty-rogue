extern crate rand;

use std::io::{stdin,BufferedReader};
use util::int_from_reader;
use generator::generate_default;
use rand::{XorShiftRng,SeedableRng,TaskRng,Rng,task_rng};

mod util;
mod generator;

fn int_from_reader<T: Reader>(reader: &mut BufferedReader<T>) -> Option<int> {
	let input = reader.read_line().ok().expect("Invalid input!");
	let opt = from_str::<int>(input.slice_to(input.len() - 1));
	opt
}

fn main() {

	let mut reader = BufferedReader::new(stdin());

	print!("How many dungeons to generate: ");
	let count = int_from_reader(&mut reader).expect("That's not a number!");
	let mut dungeons = Vec::new();

	// create the RNG
	// TODO prompt for seed?
	let seed = [1,2,3,4];

	let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
	// rng.reseed(seed); // to reset the RNG

	let mut total_w: f32 = 0.;
	let mut total_h: f32 = 0.;
	let mut total: f32 = 0.;
	for i in range(0,count) {
		println!("Generating dungeon {}...",i);

		let d = generate_default(&mut rng);
		total_w += d.width() as f32;
		total_h += d.height() as f32;
		total += 1.0;
		dungeons.push(d);
	}
	println!("Average size across {} dungeons was {}x{}.",count, (total_w/total).round(), (total_h/total).round());

	loop {
		print!("Enter a number to view that dungeon, or any non-number to exit: ");
		let idx = int_from_reader(&mut reader);
		match idx {
			None => {println!("Ooookaaaay byyyyye~"); break;}
			Some(number) => {
				let idx = number as uint;
				if idx < 0 || idx >= dungeons.len() {
					println!("That's out of range!");
					continue;
				} else {
					dungeons.get(idx).print();
				}
			}
		}
	}
}