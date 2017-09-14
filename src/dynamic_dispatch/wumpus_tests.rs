use super::*;
use std::cell::RefCell;

struct DummyDirector;

impl Director for DummyDirector {
    fn get_room(&self) -> RoomNum {
        1
    }

    fn get_action(&self) -> Action {
        Action::Fight
    }
}

struct MockDirector {
    rooms: RefCell<Vec<RoomNum>>,
    action: Action
}

impl MockDirector {
    fn new(rooms: Vec<RoomNum>, action: Action) -> Self {
        MockDirector {
            rooms: RefCell::new(rooms),
            action
        }
    }
}

impl Director for MockDirector {
    fn get_room(&self) -> RoomNum {
        let mut rooms = self.rooms.borrow_mut();
        rooms.pop().unwrap()
    }

    fn get_action(&self) -> Action {
        self.action
    }
}

// if the player is adjacent to the wumpus,
// then we should get a warning despite if awake or not.
#[quickcheck]
fn can_warn_property(player: RoomNum, wumpus: RoomNum, is_awake: bool) -> bool {
    let wumpus = Wumpus {
        room: Cell::new(wumpus),
        is_awake,
        director: box DummyDirector
    };

    let warn_result = wumpus.try_warn(player);

    if is_adj(player, wumpus.room.get()) {
        warn_result == Some(Warning::WUMPUS)
    } else {
        warn_result == None
    }
}

#[test]
fn awake_wumpus_can_kill_player() {
    let player_room = 1;
    let wumpus = Wumpus {
        room: Cell::new(player_room),
        is_awake: true,
        director: box DummyDirector
    };
    let update_result = wumpus.try_update(player_room);
    let expected = Some(UpdateResult::Death(RunResult::DeathByWumpus));
    assert_eq!(expected, update_result);
}

#[test]
fn asleep_wumpus_can_get_bumped_and_kill_or_move() {
    get_bumped_and_perform(Action::Fight, 1);
    get_bumped_and_perform(Action::Flight, 2);
}

fn get_bumped_and_perform(action: Action, expected_room_after_bump: RoomNum) {
    let player_room = 1;

    let wumpus = Wumpus {
        room: Cell::new(player_room),
        is_awake: false,
        director: box MockDirector::new(vec![expected_room_after_bump], action)
    };

    let expected = if action == Action::Fight {
        Some(UpdateResult::BumpAndDie)
    } else {
        Some(UpdateResult::BumpAndLive)
    };

    let update_result = wumpus.try_update(player_room);
    assert_eq!(expected, update_result);
    assert_eq!(expected_room_after_bump, wumpus.room.get());
}
