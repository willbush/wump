#![feature(box_syntax, box_patterns)]
#![feature(iterator_for_each)]
// quickcheck
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod game;
mod message;
mod util;
mod map;
mod player;
mod wumpus;
mod bat;
mod pit;

use game::Game;
use message::Logo;
use std::env;
use std::{thread, time};

fn main() {
    print_logo();

    let args: Vec<String> = env::args().collect();
    let is_cheating = args.len() > 1 && &args[1] == "cheat";

    let mut game = Game::new();
    if is_cheating {
        game.enable_cheat_mode();
    }
    let (_, run_result) = game.run();

    print!("{}", run_result);
}

fn print_logo() {
    println!("{}", Logo::HUNT_ASCII);
    thread::sleep(time::Duration::from_millis(500));
    println!("{}", Logo::THE_ASCII);
    thread::sleep(time::Duration::from_millis(500));
    println!("{}", Logo::WUMPUS_ASCII);
}
