#![feature(box_syntax, box_patterns)]
// quickcheck
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod game;

use game::*;

fn main() {
    let mut director = &mut PlayerDirector;
    let provider = &RandProvider;
    let mut game = Game::new(director, provider);
    let run_result = game.run();

    print!("{}", run_result);
}
