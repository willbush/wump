#![feature(box_syntax, box_patterns)]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod bat;
mod game;
mod map;
mod message;
mod pit;
mod player;
mod util;
mod wumpus;

use std::env;
use std::{thread, time};

use game::{Game, State};
use message::{Logo, Prompt};
use util::{print, read_sanitized_line};

fn main() {
    print_logo();

    let args: Vec<String> = env::args().collect();
    let is_cheating = args.len() > 1 && &args[1] == "cheat";

    let mut game = Game::new(is_cheating);
    let initial_state = game.get_state();

    loop {
        print!("{}", game.run());

        if player_responds_yes_to(Prompt::PLAY) {
            game = setup_new_game(&initial_state, is_cheating);
        } else {
            break;
        }
    }
}

fn print_logo() {
    println!("{}", Logo::HUNT_ASCII);
    thread::sleep(time::Duration::from_millis(500));
    println!("{}", Logo::THE_ASCII);
    thread::sleep(time::Duration::from_millis(500));
    println!("{}", Logo::WUMPUS_ASCII);
}

fn setup_new_game(initial_state: &State, is_cheating: bool) -> Game {
    if player_responds_yes_to(Prompt::SETUP) {
        Game::new_using(initial_state)
    } else {
        Game::new(is_cheating)
    }
}

fn player_responds_yes_to(prompt: &str) -> bool {
    loop {
        print(prompt);

        match read_sanitized_line().as_ref() {
            "Y" => return true,
            "N" => return false,
            _ => println!("Invalid input.")
        }
    }
}
