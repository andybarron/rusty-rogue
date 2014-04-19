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
	start(~GameplayScreen::new( &generate_default( 0 ) ),"Hello Dungeon",800,600);
}