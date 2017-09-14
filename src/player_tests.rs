use game::{Game, RunResult};
use std::cell::RefCell;
use super::*;

struct MockDirector {
    pub actions: RefCell<Vec<Action>>
}

impl Director for MockDirector {
    fn next(&self, _: &State) -> Action {
        let mut actions = self.actions.borrow_mut();
        actions.pop().unwrap()
    }
}

#[test]
fn can_move_player_and_quit() {
    let player_room = 1;
    // start in room 1, move until in room 12, and quit.
    // actions are in reverse order because they are popped to get the next.
    let actions = vec![
        Action::Quit,
        Action::Move(12),
        Action::Move(3),
        Action::Move(2),
    ];

    let player = create_mock_directed_player(player_room, actions);
    let initial_state = State {
        player: player_room,
        pit1: 19,
        pit2: 18,
        bat1: 19,
        bat2: 20
    };
    let expected_states = create_player_state_trans_from(&initial_state, &vec![2, 3, 12]);

    let mut game = Game::new_with_player(player, initial_state);
    let (actual_states, result) = game.run();

    assert_eq!(RunResult::UserQuit, result);
    assert_eq!(expected_states, actual_states);
}

#[test]
fn can_move_and_fall_in_pit() {
    // move into bottomless pit.
    let player = create_mock_directed_player(1, vec![Action::Move(2)]);

    let initial_state = State {
        player: 1,
        pit1: 2,
        pit2: 18,
        bat1: 19,
        bat2: 20
    };

    let mut game = Game::new_with_player(player, initial_state.to_owned());

    let expected_states = vec![initial_state];
    let (actual_states, result) = game.run();

    assert_eq!(RunResult::DeathByBottomlessPit, result);
    assert_eq!(expected_states, actual_states);
}

pub fn create_mock_directed_player(room: RoomNum, actions: Vec<Action>) -> Player {
    Player {
        room: Cell::new(room),
        director: box MockDirector { actions: RefCell::new(actions) }
    }
}

/// Create state transitions starting from the given initial state
fn create_player_state_trans_from(initial_state: &State, room_trans: &Vec<RoomNum>) -> Vec<State> {
    let mut result = Vec::new();
    result.push(initial_state.clone());

    for room in room_trans.iter() {
        result.push(State {
            player: *room,
            pit1: initial_state.pit1,
            pit2: initial_state.pit2,
            bat1: initial_state.bat1,
            bat2: initial_state.bat2
        });
    }
    result
}
