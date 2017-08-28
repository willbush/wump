#![feature(box_syntax, box_patterns)]
// quickcheck
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod game;

use game::Game;

fn main() {
    let mut game = Game::new();
    let run_result = game.run();

    print!("{}", run_result);
}
