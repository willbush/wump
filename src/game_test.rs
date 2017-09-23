use map::{rand_adj_rooms_to, rand_room, NUM_OF_ROOMS};
use player::player_tests::create_mock_directed_player;
use bat::bat_tests::create_mock_provided_bat;
use bat::SuperBat;
use pit::BottomlessPit;
use wumpus::Wumpus;
use player::Action;
use quickcheck::TestResult;
use rand::{thread_rng, Rng};
use super::*;

/// player moves into bat room, gets snatched back into the bat room, then
/// snatched to pit room.
#[test]
fn player_can_get_multi_snatched_into_pit() {
    let player_room = 1;
    let bat1_room = 2;
    let bat2_room = 20;
    let pit1_room = 3;
    let pit2_room = 19;
    let wumpus_room = 18;

    let player = box create_mock_directed_player(player_room, vec![Action::Move(bat1_room)]);

    let wumpus = Rc::new(Wumpus::new(wumpus_room));

    let hazzards: Vec<Rc<Hazzard>> = vec![
        Rc::new(BottomlessPit { room: pit1_room }),
        Rc::new(BottomlessPit { room: pit2_room }),
        Rc::new(create_mock_provided_bat(
            bat1_room,
            vec![bat1_room, pit1_room]
        )),
        Rc::new(SuperBat::new(bat2_room)),
    ];

    let mut game = Game {
        player,
        wumpus,
        pit1_room,
        pit2_room,
        bat1_room,
        bat2_room,
        hazzards,
        is_cheating: false
    };
    let (_, result) = game.run();

    assert_eq!(RunResult::DeathByBottomlessPit, result);
}

#[test]
fn can_detect_crooked_arrow_paths() {
    let a = rand_room();
    let b = rand_adj_rooms_to(a);
    let path = [a, b, a];
    assert!(is_too_crooked(&path));
}

#[test]
fn non_adj_A_B_A_path_is_not_crooked() {
    let a = rand_room();

    let b = loop {
        let b = rand_room();
        if !is_adj(a, b) {
            break b;
        }
    };
    let path = [a, b, a];
    assert!(!is_too_crooked(&path));
}

/// loop up to the max where we can shoot through up to 5 rooms in line and
/// still miss the Wumpus by one.
#[test]
fn can_miss_by_one() {
    let max = NUM_OF_ROOMS - MAX_TRAVERSABLE + 1;

    for room_num in 2..max {
        let num_to_traverse = thread_rng().gen_range(1, MAX_TRAVERSABLE + 1);
        let rooms_to_traverse: Vec<_> = (room_num..(room_num + num_to_traverse)).collect();
        let player_room = room_num - 1;
        let wumpus_room = player_room + num_to_traverse + 1;

        let shoot_result = traverse(&rooms_to_traverse, player_room, wumpus_room);

        assert_eq!(ShootResult::Miss, shoot_result);
    }
}

/// loop up to the max where we can shoot through up to 5 rooms in line and hit
/// the Wumpus.
#[test]
fn can_hit() {
    let max = NUM_OF_ROOMS - MAX_TRAVERSABLE + 1;

    for room_num in 2..max {
        let num_to_traverse = thread_rng().gen_range(1, MAX_TRAVERSABLE + 1);
        let rooms_to_traverse: Vec<_> = (room_num..(room_num + num_to_traverse)).collect();
        let player_room = room_num - 1;
        let wumpus_room = player_room + num_to_traverse;

        let shoot_result = traverse(&rooms_to_traverse, player_room, wumpus_room);

        assert_eq!(ShootResult::Hit, shoot_result);
    }
}

#[quickcheck]
fn invalid_first_room_causes_random_traversal(room_to_shoot: RoomNum) -> TestResult {
    let player = 1;

    if !is_adj(player, room_to_shoot) {
        let wumpus = 20;
        // cannot shoot from a room not adjacent to the player.
        let shoot_result = traverse(&[room_to_shoot], player, wumpus);
        let expected_remaining = 1;
        TestResult::from_bool(
            ShootResult::Remaining(expected_remaining, player) == shoot_result
        )
    } else {
        TestResult::discard()
    }
}
