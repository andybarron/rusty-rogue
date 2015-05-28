extern crate sfml;
extern crate rand;

use std::env::{args, Args};
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

mod test_gen;
mod test_search;

fn main() {
    let mut done_gen = false;
    let mut done_search = false;
    for arg in args() {
        if arg == "--test-gen" && !done_gen {
            test_gen::main();
            done_gen = true;
        } else if arg == "--test-search" && !done_search {
            test_search::main();
            done_search = true;
        }
    }

    if done_gen || done_search { return; }

	launch(GameplayScreen::new( &generate_default( 123 ) ),"Rusty Rogue",800,600);
}