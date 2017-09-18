use super::*;
use std::cell::RefCell;
use map::RoomNum;

struct MockProvider {
    rooms: RefCell<Vec<RoomNum>>
}

impl MockProvider {
    fn new(rooms: Vec<RoomNum>) -> Self {
        MockProvider { rooms: RefCell::new(rooms) }
    }
}

impl RoomProvider for MockProvider {
    fn get_room(&self) -> RoomNum {
        let mut rooms = self.rooms.borrow_mut();
        rooms.pop().unwrap()
    }
}

#[test]
fn can_warn_player() {
    let provider = box MockProvider::new(vec![666]);
    let bat = SuperBat { room: 1, provider };
    let player_room = 2;

    assert_eq!(Some(Warning::BAT), bat.try_warn(player_room));
}

#[test]
fn can_snatch_player() {
    let first = 15;
    let second = 20;
    let provider = box MockProvider::new(vec![second, first]);
    let room = 1;
    let bat = SuperBat { room, provider };

    let s = &State { player: room, ..Default::default()};
    assert_eq!(Some(UpdateResult::SnatchTo(first)), bat.try_update(s));
    assert_eq!(Some(UpdateResult::SnatchTo(second)), bat.try_update(s));
}

pub fn create_mock_provided_bat(room: RoomNum, mut snatch_order: Vec<RoomNum>) -> SuperBat {
    // reverse snatch order because they are popped to get the next room to snatch to.
    snatch_order.reverse();
    SuperBat {
        room,
        provider: box MockProvider::new(snatch_order)
    }
}
