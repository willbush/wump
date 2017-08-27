#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod game;

use rand::{thread_rng, Rng};
use std::io;
use std::io::Write;
use game::*;

fn main() {
    let mut rng = thread_rng();
    let player_room: RoomNum = rng.gen_range(1, MAP.len() + 1);
    let pit1_room = 19;
    let pit2_room = 20;

    let initial_pos = Pos::new(player_room, pit1_room, pit2_room);

    match Game::new(initial_pos, Box::new(PlayerActionProvider)) {
        Ok(mut game) => game.run(),
        Err(e) => panic!("{:?}", e),
    }
}

struct PlayerActionProvider;

impl ActionProvider for PlayerActionProvider {
    fn next(&mut self, positions: &Pos) -> Action {
        let room_num = positions.player;
        loop {
            println!("You are in room {}", room_num);
            let (a, b, c) = game::adj_rooms_to(room_num);
            println!("Tunnel leads to {} {} {}", a, b, c);
            print("Shoot, Move, or Quit (S, M, Q) ");

            match read_sanitized_line().as_ref() {
                "M" => return Action::Move(get_adj_room_to(room_num)),
                "Q" => return Action::Quit,
                _ => continue,
            }
        }
    }
}

fn get_adj_room_to(room: RoomNum) -> RoomNum {
    print("Where to? ");

    loop {
        let input = read_sanitized_line();

        match input.parse::<RoomNum>() {
            Ok(next) if game::can_move(room, next) => return next,
            _ => print("Not Possible - Where to? "),
        }
    }
}

// Reads a line from stdin, trims it, and returns it as upper case.
fn read_sanitized_line() -> String {
    read_trimed_line().to_uppercase()
}

fn read_trimed_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}

// Print without new line and flush to force it to show up.
fn print(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}
