extern crate sfml;
extern crate rand;

use std::env::args;
use old_engine::launch;
use gameplay::GameplayScreen;
use generator::generate_default;

mod engine;

mod util;
mod components;
mod generator;
mod graph;
mod search;
mod solver;
mod old_engine;
mod collision;
mod animation;
mod entities;
mod gameplay;

mod test_gen;
mod test_search;

fn main() {
    let mut done_gen = false;
    let mut done_search = false;
    let mut use_new = false;
    for arg in args() {
        if arg == "--test-gen" && !done_gen {
            test_gen::main();
            done_gen = true;
        } else if arg == "--test-search" && !done_search {
            test_search::main();
            done_search = true;
        } else if arg == "--new" && !use_new {
            use_new = true;
        }
    }

    if done_gen || done_search { return; }

    if use_new {
        engine::testing::launch_test();
    } else {
        launch(GameplayScreen::new( &generate_default( 123 ) ), "Rusty Rogue",
                800, 600);
    }
}