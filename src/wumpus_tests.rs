use super::*;
use std::cell::RefCell;
use map::{adj_rooms_to, rand_room};
use rand::{thread_rng, Rng};

struct DummyDirector;

impl Director for DummyDirector {
    fn get_room(&self, _: &State) -> RoomNum {
        1
    }

    fn feels_like_moving(&self) -> bool {
        true
    }
}

struct MockDirector {
    rooms: RefCell<Vec<RoomNum>>,
    feels_like_moving: bool
}

impl MockDirector {
    fn new(rooms: Vec<RoomNum>, feels_like_moving: bool) -> Self {
        MockDirector {
            rooms: RefCell::new(rooms),
            feels_like_moving
        }
    }
}

impl Director for MockDirector {
    fn get_room(&self, _: &State) -> RoomNum {
        let mut rooms = self.rooms.borrow_mut();
        rooms.pop().unwrap()
    }

    fn feels_like_moving(&self) -> bool {
        self.feels_like_moving
    }
}

struct MockFeelingOnly {
    director: Box<Director>,
    feels_like_moving: bool
}

impl Director for MockFeelingOnly {
    fn get_room(&self, s: &State) -> RoomNum {
        self.director.get_room(s)
    }

    fn feels_like_moving(&self) -> bool {
        self.feels_like_moving
    }
}

// if the player is adjacent to the wumpus,
// then we should get a warning despite if it is awake or not.
#[quickcheck]
fn warns_when_adj(player: RoomNum, wumpus: RoomNum, is_awake: bool) -> bool {
    let wumpus = Wumpus {
        room: Cell::new(wumpus),
        is_awake: Cell::new(is_awake),
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
fn awake_wumpus_kills_player() {
    let player_room = 1;
    let wumpus = Wumpus {
        room: Cell::new(player_room),
        is_awake: Cell::new(true),
        director: box DummyDirector
    };
    let update_result = wumpus.try_update(&State {
        player: player_room,
        ..Default::default()
    });
    let expected = Some(UpdateResult::Death(RunResult::KilledByWumpus));
    assert_eq!(expected, update_result);
}

#[test]
fn bumping_wumpus_while_asleep_kills_or_moves() {
    get_bumped_and(false, 1);
    get_bumped_and(true, 2);
}

fn get_bumped_and(feels_like_moving: bool, expected_room_after_bump: RoomNum) {
    let player_room = 1;

    let wumpus = Wumpus {
        room: Cell::new(player_room),
        is_awake: Cell::new(false),
        director: box MockDirector::new(vec![expected_room_after_bump], feels_like_moving)
    };

    let expected = if feels_like_moving {
        Some(UpdateResult::BumpAndLive)
    } else {
        Some(UpdateResult::BumpAndDie)
    };

    let update_result = wumpus.try_update(&State {
        player: player_room,
        ..Default::default()
    });
    assert_eq!(expected, update_result);
    assert!(wumpus.is_awake.get());
    assert_eq!(expected_room_after_bump, wumpus.room.get());
}

#[test]
fn awake_wumpus_avoids_pits_when_moving() {
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
        is_awake: Cell::new(true),
        director: box MockFeelingOnly {
            director: box WumpusDirector,
            feels_like_moving: true
        }
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

#[test]
fn player_with_arrows_below_max_capacity_wakes_wumpus() {
    let wumpus_room = 15;
    let wumpus = Wumpus::new(wumpus_room);
    let result = wumpus.try_update(&State {
        wumpus: wumpus_room,
        arrow_count: 4,
        ..Default::default()
    });
    assert!(wumpus.is_awake.get());
    assert_eq!(None, result);
}
