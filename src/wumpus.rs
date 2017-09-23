#[cfg(test)]
#[path = "./wumpus_tests.rs"]
pub mod wumpus_tests;

use message::Warning;
use game::{Hazzard, RunResult, State, UpdateResult};
use map::{adj_rooms_to, is_adj, RoomNum};
use rand::{thread_rng, Rng};
use std::cell::Cell;

trait Director {
    fn get_room(&self, state: &State) -> RoomNum;
    fn feels_like_moving(&self) -> bool;
}

pub struct Wumpus {
    pub room: Cell<RoomNum>,
    director: Box<Director>,
    is_awake: Cell<bool>
}

impl Wumpus {
    pub fn new(room: RoomNum) -> Self {
        Wumpus {
            room: Cell::new(room),
            is_awake: Cell::new(false),
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
        let is_bumped = !self.is_awake.get() && s.player == self.room.get();

        if is_bumped {
            self.is_awake.replace(true);
        }
        if self.is_awake.get() && self.director.feels_like_moving() {
            let next_room = self.director.get_room(s);
            self.room.replace(next_room);
        }
        if self.is_awake.get() && s.player == self.room.get() {
            if is_bumped {
                Some(UpdateResult::BumpAndDie)
            } else {
                Some(UpdateResult::Death(RunResult::DeathByWumpus))
            }
        } else if is_bumped {
            Some(UpdateResult::BumpAndLive)
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

    /// Wumpus feels like moving with a 75% chance.
    fn feels_like_moving(&self) -> bool {
        let n = thread_rng().gen_range(1, 5);
        n > 1
    }
}
