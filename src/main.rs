extern crate rsfml;
extern crate rand;
extern crate collections;

use rand::{XorShiftRng,SeedableRng};

use engine::start;
use gameplay::GameplayScreen;
use generator::generate_default;

mod util;
mod generator;
mod engine;
mod entities;
mod gameplay;

fn main() {
	start(~GameplayScreen::new( &generate_default( -1 ) ),"Hello Dungeon",800,600);
}