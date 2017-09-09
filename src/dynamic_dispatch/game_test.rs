use dynamic_dispatch::player::player_tests::MockDirector;
use dynamic_dispatch::bat::bat_tests::MockProvider;
use dynamic_dispatch::bat::SuperBat;
use dynamic_dispatch::pit::BottomlessPit;
use dynamic_dispatch::player::{Action, Player};
use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use super::*;

#[test]
fn player_can_get_multi_snatched_into_pit() {
    let bat_room = 2;
    let pit_room = 3;

    // move into super bats.
    let actions = vec![Action::Move(bat_room)];
    let mock = box MockDirector { actions: RefCell::new(actions) };

    // setup super bat provider to tell the bat to snatch to room 2 and then 3.
    let first_snatch = bat_room;
    let second_snatch = pit_room;
    let provider = box MockProvider {
        rooms: RefCell::new(vec![second_snatch, first_snatch])
    };

    let player = Rc::new(Player {
        room: Cell::new(1),
        director: mock
    });
    let pit1 = Rc::new(BottomlessPit { room: pit_room });
    let pit2 = Rc::new(BottomlessPit { room: 19 });
    let bat1 = Rc::new(SuperBat { room: bat_room, provider });
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
