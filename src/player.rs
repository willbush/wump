#[cfg(test)]
#[path = "./player_tests.rs"]
pub mod player_tests;

use message;
use map::{RoomNum, NUM_OF_ROOMS};
use game::{State, MAX_TRAVERSABLE};
use map::{adj_rooms_to, is_adj};
use util::{print, read_line, read_sanitized_line};
use std::cell::Cell;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Shoot(Vec<RoomNum>),
    Move(RoomNum),
    Quit
}

pub struct Player {
    pub room: Cell<RoomNum>,
    director: Box<Director>
}

impl Player {
    pub fn new(room: RoomNum) -> Self {
        Player {
            director: box PlayerDirector,
            room: Cell::new(room)
        }
    }

    pub fn get_action(&self, state: &State) -> Action {
        self.director.next(state)
    }
}

pub trait Director {
    fn next(&self, state: &State) -> Action;
}

pub struct PlayerDirector;

impl Director for PlayerDirector {
    fn next(&self, state: &State) -> Action {
        let room_num = state.player;
        loop {
            println!("You are in room {}", room_num);
            let (a, b, c) = adj_rooms_to(room_num);
            println!("Tunnel leads to {} {} {}", a, b, c);
            print(message::Prompt::ACTION);

            match read_sanitized_line().as_ref() {
                "M" => return Action::Move(get_adj_room_to(room_num)),
                "Q" => return Action::Quit,
                "S" => return Action::Shoot(get_rooms_to_shoot(room_num)),
                _ => continue
            }
        }
    }
}

pub fn get_adj_room_to(room: RoomNum) -> RoomNum {
    print("Where to? ");

    loop {
        let input = read_sanitized_line();

        match input.parse::<RoomNum>() {
            Ok(next) if is_adj(room, next) => return next,
            _ => print("Not Possible - Where to? ")
        }
    }
}

fn get_rooms_to_shoot(player: RoomNum) -> Vec<RoomNum> {
    loop {
        print("Enter up to 5 space separated rooms to shoot: ");

        if let Some(rooms) = try_parse_rooms_from_user(player) {
            if is_too_crooked(&rooms) {
                println!("{}", message::Message::TOO_CROOKED);
            } else {
                return rooms;
            }
        }
    }
}

fn try_parse_rooms_from_user(player: RoomNum) -> Option<Vec<RoomNum>> {
    let mut result = vec![player];
    let line = read_line();
    let rooms = line.split_whitespace()
        .take(MAX_TRAVERSABLE)
        .map(|r| r.parse::<RoomNum>());

    for r in rooms {
        match r {
            Ok(room_num) => if room_num > 0 && room_num <= NUM_OF_ROOMS {
                result.push(room_num);
            } else {
                println!("The room number {} is out of bounds.", room_num);
                return None;
            },
            Err(_) => {
                println!("The given list of rooms contains one or more invalid numbers.");
                return None;
            }
        }
    }
    Some(result)
}

/// A path is too crooked if it contains an A-B-A path where A is adjacent to B.
fn is_too_crooked(path: &[RoomNum]) -> bool {
    path.windows(3)
        .any(|x| x.len() == 3 && is_adj(x[0], x[1]) && x[0] == x[2])
}
