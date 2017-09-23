#[cfg(test)]
#[path = "./player_tests.rs"]
pub mod player_tests;

use message::Prompt;
use map::RoomNum;
use game::State;
use map::adj_rooms_to;
use util::{get_adj_room_to, print, read_sanitized_line};
use std::cell::Cell;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Shoot(RoomNum),
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
            print(Prompt::ACTION);

            match read_sanitized_line().as_ref() {
                "M" => return Action::Move(get_adj_room_to(room_num)),
                "Q" => return Action::Quit,
                _ => continue
            }
        }
    }
}
