use dynamic_dispatch::player::player_tests::create_mock_directed_player;
use dynamic_dispatch::bat::bat_tests::create_mock_provided_bat;
use dynamic_dispatch::bat::SuperBat;
use dynamic_dispatch::pit::BottomlessPit;
use dynamic_dispatch::player::Action;
use std::rc::Rc;
use super::*;

#[test]
fn player_can_get_multi_snatched_into_pit() {
    // player moves into bat room, gets snatched back into the bat room,
    // then snatched to pit room.
    let player_room = 1;
    let bat_room = 2;
    let pit_room = 3;

    let player = Rc::new(create_mock_directed_player(
        player_room,
        vec![Action::Move(bat_room)]
    ));

    let super_bat = create_mock_provided_bat(bat_room, vec![bat_room, pit_room]);

    let pit1 = Rc::new(BottomlessPit { room: pit_room });
    let pit2 = Rc::new(BottomlessPit { room: 19 });
    let bat1 = Rc::new(super_bat);
    let bat2 = Rc::new(SuperBat::new(20));

    let hazzards: Vec<Rc<Hazzard>> = vec![pit1.clone(), pit2.clone(), bat1.clone(), bat2.clone()];

    let mut game = Game {
        player,
        pit1,
        pit2,
        bat1,
        bat2,
        hazzards,
        is_cheating: false
    };
    let (_, result) = game.run();

    assert_eq!(RunResult::DeathByBottomlessPit, result);
}
