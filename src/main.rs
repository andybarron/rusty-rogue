extern crate rsfml;
extern crate rand;

use rand::{XorShiftRng,SeedableRng};

use engine::start;
use gameplay::GameplayScreen;
use generator::generate_default;

mod util;
mod generator;
mod engine;
mod gameplay;

fn main() {
	let seed = [1,2,3,4];
	let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
	start(~GameplayScreen::new( &generate_default( &mut rng ) ),"Hello Dungeon",800,600);
}