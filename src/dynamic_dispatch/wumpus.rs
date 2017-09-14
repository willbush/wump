#[cfg(test)]
#[path = "./wumpus_tests.rs"]
pub mod wumpus_tests;

use message::Warning;
use dynamic_dispatch::game::{Hazzard, RunResult, UpdateResult};
use map::{is_adj, RoomNum};
use std::cell::Cell;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Action {
    Fight,
    Flight
}

trait Director {
    fn get_room(&self) -> RoomNum;
    fn get_action(&self) -> Action;
}

pub struct Wumpus {
    room: Cell<RoomNum>,
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

    fn try_update(&self, player_room: RoomNum) -> Option<UpdateResult> {
        if player_room == self.room.get() {
            if self.is_awake {
                Some(UpdateResult::Death(RunResult::DeathByWumpus))
            } else {
                match self.director.get_action() {
                    Action::Flight => {
                        self.room.replace(self.director.get_room());
                        Some(UpdateResult::BumpAndLive)
                    }
                    Action::Fight => Some(UpdateResult::BumpAndDie)
                }
            }
        } else {
            None
        }
    }
}

struct WumpusDirector;
impl Director for WumpusDirector {
    fn get_room(&self) -> RoomNum {
        1
    }

    fn get_action(&self) -> Action {
        Action::Fight
    }
}
