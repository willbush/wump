#[cfg(test)]
#[path = "./wumpus_tests.rs"]
pub mod wumpus_tests;

use game::{Hazzard, RunResult, State, UpdateResult};
use map::{adj_rooms_to, is_adj, RoomNum};
use message::Warning;
use player;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::cell::Cell;

trait Director {
    fn get_room(&self, state: &State) -> RoomNum;
    fn feels_like_moving(&self) -> bool;
}

pub struct Wumpus {
    pub room: Cell<RoomNum>,
    director: Box<dyn Director>,
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
        let arrow_shot_awakes_wumpus =
            !self.is_awake.get() && s.arrow_count < player::ARROW_CAPACITY;

        if is_bumped || arrow_shot_awakes_wumpus {
            self.is_awake.set(true);
        }
        if self.is_awake.get() && self.director.feels_like_moving() {
            let next_room = self.director.get_room(s);
            self.room.set(next_room);

            if s.is_cheating {
                println!("Wumpus moved to: {}", next_room);
            }
        }
        if self.is_awake.get() && s.player == self.room.get() {
            if is_bumped {
                Some(UpdateResult::BumpAndDie)
            } else {
                Some(UpdateResult::Death(RunResult::KilledByWumpus))
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

        adj_rooms.shuffle(&mut thread_rng());
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
