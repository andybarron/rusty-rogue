extern crate rsfml;
extern crate rand;

use engine::start;
use gameplay::GameplayScreen;

mod util;
mod engine;
mod gameplay;

fn main() {
	start(~GameplayScreen::new(),"Hello Dungeon",800,600);
}