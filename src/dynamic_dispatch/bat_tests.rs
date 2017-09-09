use super::*;
use std::cell::RefCell;

pub struct MockProvider {
    pub rooms: RefCell<Vec<RoomNum>>
}

impl RoomProvider for MockProvider {
    fn get_room(&self) -> RoomNum {
        let mut rooms = self.rooms.borrow_mut();
        rooms.pop().unwrap()
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
