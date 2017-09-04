#![feature(box_syntax, box_patterns)]
// quickcheck
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod static_dispatch;
mod dynamic_dispatch;

use static_dispatch::game::{Game as SdGame, PlayerDirector, RandProvider};
use dynamic_dispatch::game::Game as DdGame;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_cheating = args.len() > 1 && &args[1] == "cheat";
    let is_sd_game = args.len() > 2 && &args[2] == "dd";

    if is_sd_game {
        let game = DdGame;
        game.say_hi();
    } else {
        let mut director = &mut PlayerDirector;
        let provider = &RandProvider;
        let mut game = SdGame::new(director, provider);
        if is_cheating {
            game.enable_cheat_mode();
        }
        let run_result = game.run();

        print!("{}", run_result);
    }
}
