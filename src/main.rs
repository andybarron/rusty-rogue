extern crate rsfml;
extern crate rand;
extern crate collections;
extern crate native;

use rand::{XorShiftRng,SeedableRng};

use engine::launch;
use gameplay::GameplayScreen;
use generator::generate_default;

mod util;
mod generator;
mod graph;
mod search;
mod engine;
mod collision;
mod entities;
mod gameplay;

#[start]
fn start(argc: int, argv: **u8) -> int { native::start(argc, argv, main) }

fn main() {
	launch(~GameplayScreen::new( &generate_default( 123 ) ),"Hello Dungeon",800,600);
}