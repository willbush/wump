#![feature(box_syntax, box_patterns)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod game;

use game::*;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::io;
use std::io::Write;

fn main() {
    let (player, pit1, pit2) = gen_unique_rooms();

    let initial_positions = Pos::new(player, pit1, pit2);

    match Game::new(initial_positions, box PlayerActionProvider) {
        Ok(mut game) => print!("{}", game.run()),
        Err(e) => panic!("{:?}", e),
    };
}

fn gen_unique_rooms() -> (RoomNum, RoomNum, RoomNum) {
    let mut taken_rooms = HashSet::new();

    let player_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(player_room);
    let pit1_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(pit1_room);
    let pit2_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(pit2_room);

    (player_room, pit1_room, pit2_room)
}

fn gen_unique_rand_room(taken_rooms: &HashSet<RoomNum>) -> RoomNum {
    let mut rng = thread_rng();

    loop {
        let room: RoomNum = rng.gen_range(1, MAP.len() + 1);

        if !taken_rooms.contains(&room) {
            return room;
        }
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
