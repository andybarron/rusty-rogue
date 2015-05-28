// gen_test.rs
extern crate rand;

use std::io::{stdin,Read,BufReader,BufRead};
use generator::generate_default;
use std::str::FromStr;

mod util;
mod generator;

fn uint_from_reader<T: Read>(reader: &mut BufReader<T>) -> Option<usize> {
	let mut buf = String::new();
	reader.read_line(&mut buf).ok().expect("Invalid input!");
	let opt = usize::from_str(buf.trim());
	opt.ok()
}

fn main() {

	let mut reader = BufReader::new(stdin());

	println!("How many dungeons to generate: ");
	let count = uint_from_reader(&mut reader).expect("That's not a number!");
	let mut dungeons = Vec::new();

	// create RNG seed
	// TODO prompt for seed?
	let seed = 0;

	let mut total_w: f32 = 0.;
	let mut total_h: f32 = 0.;
	let mut total: f32 = 0.;
	for i in 0..count {
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
		println!("Enter a number to view that dungeon, or any non-number to exit: ");
		let idx = uint_from_reader(&mut reader);
		match idx {
			None => {println!("Ooookaaaay byyyyye~"); break;}
			Some(number) => {
				let idx = number;
				if idx >= dungeons.len() {
					println!("That's out of range!");
					continue;
				} else {
					dungeons.get(idx).map(|d| d.print());
				}
			}
		}
	}
}