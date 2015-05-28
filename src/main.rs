extern crate sfml;
extern crate rand;

use engine::launch;
use gameplay::GameplayScreen;
use generator::generate_default;

mod util;
mod generator;
mod graph;
mod search;
mod solver;
mod engine;
mod collision;
mod animation;
mod entities;
mod gameplay;

fn main() {
	launch(GameplayScreen::new( &generate_default( 123 ) ),"Rusty Rogue",800,600);
}