use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::iter::FromIterator;

use gameplay::GameplayScreen;
use generator::generate_default;
use old_engine::launch;

mod animation;
mod collision;
mod entities;
mod gameplay;
mod generator;
mod graph;
mod old_engine;
mod search;
mod solver;
mod util;

mod test_gen;
mod test_search;

fn main() {
    let run_game = Cell::new(true);
    let mut fn_map: HashMap<String, Box<Fn()>> = HashMap::new();
    fn_map.insert(
        "--gen".into(),
        Box::new(|| {
            test_gen::main();
            run_game.set(false);
        }),
    );
    fn_map.insert(
        "--search".into(),
        Box::new(|| {
            test_search::main();
            run_game.set(false);
        }),
    );

    let unique_args: HashSet<String> = HashSet::from_iter(args());

    for arg in unique_args.iter() {
        fn_map.remove(arg).map(|ref mut f| f());
    }

    if !run_game.get() {
        return;
    }

    launch(
        GameplayScreen::new(
            &generate_default(Default::default()),
            &GameplayScreen::load_texture(),
        ),
        "Rusty Rogue",
        800,
        600,
    );
}
