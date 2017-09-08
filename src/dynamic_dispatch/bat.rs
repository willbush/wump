use map::{is_adj, rand_room, RoomNum};
use message::Warning;
use dynamic_dispatch::game::{Hazzard, UpdateResult};

pub struct SuperBat {
    pub room: RoomNum,
    provider: Box<RoomProvider>
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

#[cfg(test)]
mod bat_tests {
    use super::*;
    use std::cell::RefCell;

    struct MockProvider {
        rooms: RefCell<Vec<RoomNum>>
    }

    impl RoomProvider for MockProvider {
        fn get_room(&self) -> RoomNum {
            let mut rooms = self.rooms.borrow_mut();

            match rooms.pop() {
                Some(room) => room,
                None => panic!("more pops than expected!")
            }
        }
    }

    #[test]
    fn can_warn_player() {
        let provider = box MockProvider { rooms: RefCell::new(vec![666]) };
        let bat = SuperBat { room: 1, provider };
        let player_room = 2;

        assert_eq!(Some(Warning::BAT), bat.try_warn(player_room));
    }

    #[test]
    fn can_snatch_player() {
        let first = 15;
        let second = 20;
        let provider = box MockProvider {
            rooms: RefCell::new(vec![second, first])
        };
        let room = 1;
        let bat = SuperBat { room, provider };

        assert_eq!(Some(UpdateResult::SnatchTo(first)), bat.try_update(room));
        assert_eq!(Some(UpdateResult::SnatchTo(second)), bat.try_update(room));
    }
}
