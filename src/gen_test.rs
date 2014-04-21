extern crate rand;

use std::io::{stdin,BufferedReader};
use generator::generate_default;

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

	// create RNG seed
	// TODO prompt for seed?
	let seed = 0;

	let mut total_w: f32 = 0.;
	let mut total_h: f32 = 0.;
	let mut total: f32 = 0.;
	for i in range(0,count) {
		println!("Generating dungeon {}...",i);

		let mut d = generate_default(seed+i as u32);
		total_w += d.width() as f32;
		total_h += d.height() as f32;
		total += 1.0;
		d.shrink(); // this doesn't happen normally because of coordinate screwups
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