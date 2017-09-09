#[cfg(test)]
#[path = "./bat_tests.rs"]
pub mod bat_tests;

use map::{is_adj, rand_room, RoomNum};
use message::Warning;
use dynamic_dispatch::game::{Hazzard, UpdateResult};

pub struct SuperBat {
    pub room: RoomNum,
    pub provider: Box<RoomProvider>
}

impl SuperBat {
    pub fn new(room: RoomNum) -> Self {
        SuperBat {
            room,
            provider: box BatRoomProvider
        }
    }
}

impl Hazzard for SuperBat {
    fn try_update(&self, player_room: RoomNum) -> Option<UpdateResult> {
        if self.room == player_room {
            let else_where = self.provider.get_room();
            Some(UpdateResult::SnatchTo(else_where))
        } else {
            None
        }
    }

    fn try_warn(&self, player_room: RoomNum) -> Option<&str> {
        if is_adj(player_room, self.room) {
            Some(Warning::BAT)
        } else {
            None
        }
    }
}

pub trait RoomProvider {
    fn get_room(&self) -> RoomNum;
}

pub struct BatRoomProvider;

impl RoomProvider for BatRoomProvider {
    fn get_room(&self) -> RoomNum {
        rand_room()
    }
}
