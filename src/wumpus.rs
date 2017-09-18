#[cfg(test)]
#[path = "./wumpus_tests.rs"]
pub mod wumpus_tests;

use message::Warning;
use game::{Hazzard, RunResult, State, UpdateResult};
use map::{adj_rooms_to, is_adj, RoomNum};
use rand::{thread_rng, Rng};
use std::cell::Cell;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Action {
    Fight,
    Flight
}

trait Director {
    fn get_room(&self, state: &State) -> RoomNum;
    fn get_action(&self) -> Action;
}

pub struct Wumpus {
    pub room: Cell<RoomNum>,
    director: Box<Director>,
    is_awake: bool
}

impl Wumpus {
    pub fn new(room: RoomNum) -> Self {
        Wumpus {
            room: Cell::new(room),
            is_awake: false,
            director: box WumpusDirector
        }
    }
}

impl Hazzard for Wumpus {
    fn try_warn(&self, player_room: RoomNum) -> Option<&str> {
        if is_adj(player_room, self.room.get()) {
            Some(Warning::WUMPUS)
        } else {
            None
        }
    }

    fn try_update(&self, s: &State) -> Option<UpdateResult> {
        if self.is_awake {
            if s.player == self.room.get() {
                Some(UpdateResult::Death(RunResult::DeathByWumpus))
            } else {
                self.room.replace(self.director.get_room(s));
                None
            }
        } else if s.player == self.room.get() {
            match self.director.get_action() {
                Action::Flight => {
                    self.room.replace(self.director.get_room(s));
                    Some(UpdateResult::BumpAndLive)
                }
                Action::Fight => Some(UpdateResult::BumpAndDie)
            }
        } else {
            None
        }
    }
}

struct WumpusDirector;
impl Director for WumpusDirector {
    fn get_room(&self, s: &State) -> RoomNum {
        let (a, b, c) = adj_rooms_to(s.wumpus);
        let mut adj_rooms = [a, b, c];

        thread_rng().shuffle(&mut adj_rooms);
        *adj_rooms
            .iter()
            .find(|room| **room != s.pit1 && **room != s.pit2)
            .unwrap()
    }

    fn get_action(&self) -> Action {
        let n = thread_rng().gen_range(1, 5);

        // fight with 25% chance and flight with 75% chance.
        if n == 1 {
            Action::Fight
        } else {
            Action::Flight
        }
    }
}
