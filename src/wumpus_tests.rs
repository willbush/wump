use super::*;
use std::cell::RefCell;
use map::{adj_rooms_to, rand_room};
use rand::{thread_rng, Rng};

struct DummyDirector;

impl Director for DummyDirector {
    fn get_room(&self, _: &State) -> RoomNum {
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
    fn get_room(&self, _: &State) -> RoomNum {
        let mut rooms = self.rooms.borrow_mut();
        rooms.pop().unwrap()
    }

    fn get_action(&self) -> Action {
        self.action
    }
}

// if the player is adjacent to the wumpus,
// then we should get a warning despite if it is awake or not.
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
    let update_result = wumpus.try_update(&State {
        player: player_room,
        ..Default::default()
    });
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

    let update_result = wumpus.try_update(&State {
        player: player_room,
        ..Default::default()
    });
    assert_eq!(expected, update_result);
    assert_eq!(expected_room_after_bump, wumpus.room.get());
}

#[test]
fn awake_wumpus_can_avoid_pits_when_moving() {
    // when wumpus is in random room and there are bottomless pits in two adjacent
    // rooms, then the only place to go is room the final adjacent room that's not occupied.
    let wumpus_room = rand_room();
    let (a, b, c) = adj_rooms_to(wumpus_room);
    let mut shuffled_adj_rooms = [a, b, c];
    thread_rng().shuffle(&mut shuffled_adj_rooms);

    let pit1_room = shuffled_adj_rooms[0];
    let pit2_room = shuffled_adj_rooms[1];
    let expected_room = shuffled_adj_rooms[2];

    let wumpus = Wumpus {
        room: Cell::new(wumpus_room),
        is_awake: true,
        director: box WumpusDirector
    };
    let update_result = wumpus.try_update(&State {
        wumpus: wumpus_room,
        pit1: pit1_room,
        pit2: pit2_room,
        ..Default::default()
    });
    assert_eq!(
        expected_room,
        wumpus.room.get(),
        "original wumpus room: {}, expected room: {}, pit1: {}, pit2: {}",
        wumpus_room,
        expected_room,
        pit1_room,
        pit2_room
    );
    assert_eq!(None, update_result);
}
