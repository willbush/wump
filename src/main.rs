#![feature(box_syntax, box_patterns)]
// quickcheck
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod static_dispatch;
mod dynamic_dispatch;
mod message;
mod util;
mod map;

use static_dispatch::game::{Game as SdGame, PlayerDirector, RandProvider};
// use dynamic_dispatch::game::Game as DdGame;
use message::Logo;
use std::env;
use std::{thread, time};

fn main() {
    print_logo();

    let args: Vec<String> = env::args().collect();
    let is_cheating = args.len() > 1 && &args[1] == "cheat";
    let is_sd_game = args.len() > 2 && &args[2] == "dd";

    if is_sd_game {
        // let mut game = DdGame;
        // game.run();
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

fn print_logo() {
    println!("{}", Logo::HUNT_ASCII);
    thread::sleep(time::Duration::from_millis(500));
    println!("{}", Logo::THE_ASCII);
    thread::sleep(time::Duration::from_millis(500));
    println!("{}", Logo::WUMPUS_ASCII);
}
