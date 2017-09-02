#![feature(box_syntax, box_patterns)]
// quickcheck
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod game;

use game::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_cheating = args.len() > 0 && &args[1] == "cheat";

    let mut director = &mut PlayerDirector;
    let provider = &RandProvider;
    let mut game = Game::new(director, provider);
    if is_cheating {
        game.enable_cheat_mode();
    }
    let run_result = game.run();

    print!("{}", run_result);
}
