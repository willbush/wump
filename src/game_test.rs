use player::player_tests::create_mock_directed_player;
use bat::bat_tests::create_mock_provided_bat;
use bat::SuperBat;
use pit::BottomlessPit;
use player::Action;
use super::*;

#[test]
fn player_can_get_multi_snatched_into_pit() {
    // player moves into bat room, gets snatched back into the bat room,
    // then snatched to pit room.
    let player_room = 1;
    let bat1_room = 2;
    let bat2_room = 20;
    let pit1_room = 3;
    let pit2_room = 19;

    let player = box create_mock_directed_player(player_room, vec![Action::Move(bat1_room)]);

    let hazzards: Vec<Box<Hazzard>> = vec![
        box BottomlessPit { room: pit1_room },
        box BottomlessPit { room: pit2_room },
        box create_mock_provided_bat(bat1_room, vec![bat1_room, pit1_room]),
        box SuperBat::new(bat2_room),
    ];

    let mut game = Game {
        player,
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
