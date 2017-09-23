use game::{Game, RunResult, MAX_TRAVERSABLE};
use map::{rand_adj_room_to, rand_room};
use std::cell::RefCell;
use rand::{thread_rng, Rng};
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
        wumpus: 20,
        pit1: 19,
        pit2: 18,
        bat1: 17,
        bat2: 16
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
        wumpus: 17,
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
            wumpus: initial_state.wumpus,
            pit1: initial_state.pit1,
            pit2: initial_state.pit2,
            bat1: initial_state.bat1,
            bat2: initial_state.bat2
        });
    }
    result
}

/// A path is too crooked for an arrow if it follows an A-B-A path where A is
/// adjacent to B. Since the max number of traversable rooms is 5, paths of
/// length 3, 4, 5 are of interest because they can contain "too crooked" paths.
/// Consider the following cases where 'v' are valid paths and 'x' doesn't
/// matter:
///
/// len 5: A-B-A-x-x   len 4: A-B-A-x len 3: A-B-A
///        v-A-B-A-x          v-A-B-A
///        v-v-A-B-A
///
/// since the 'x' paths above don't matter and the function should return true
/// before reaching them, I'm going to test the following 3 cases:
///
/// A-B-A
/// v-A-B-A
/// v-v-A-B-A
#[test]
fn can_detect_crooked_arrow_paths() {
    // case1: A-B-A
    let case1 = || {
        let a = rand_room();
        let b = rand_adj_room_to(a);
        let path = [a, b, a];
        assert!(is_too_crooked(&path), "{:?}", &path);
    };

    // case2: v-A-B-A
    let case2 = || {
        let v = rand_room();
        let a = rand_adj_room_to(v);
        let b = rand_valid_adj_room_to(a, v);
        let path = [v, a, b, a];
        assert!(is_too_crooked(&path), "{:?}", &path);
    };

    // case3: v-v-A-B-A
    let case3 = || {
        let v1 = rand_room();
        let v2 = rand_adj_room_to(v1);
        let a = rand_valid_adj_room_to(v2, v1);
        let b = rand_valid_adj_room_to(a, v2);
        let path = [v1, v2, a, b, a];
        assert!(is_too_crooked(&path), "{:?}", &path);
    };
    perform_trial(10, &case1);
    perform_trial(10, &case2);
    perform_trial(10, &case3);
}

#[test]
fn valid_paths_are_not_too_crooked() {
    perform_trial(10, &|| {
        // any valid num of valid paths from [1, 5]
        let num_to_traverse = thread_rng().gen_range(1, MAX_TRAVERSABLE + 1);
        let mut valid_path = Vec::with_capacity(num_to_traverse);

        for i in 0..num_to_traverse {
            if i == 0 {
                valid_path.push(rand_room());
            } else if i == 1 {
                let prev = valid_path[i - 1];
                valid_path.push(rand_adj_room_to(prev));
            } else {
                let prev = valid_path[i - 1];
                let before_prev = valid_path[i - 2];
                valid_path.push(rand_valid_adj_room_to(prev, before_prev));
            }
        }
        assert!(!is_too_crooked(&valid_path), "{:?}", &valid_path);
    });
}

fn perform_trial(trial_count: u32, trial: &Fn()) {
    (0..trial_count).for_each(|_| trial());
}

/// Gets a random room adjacent to the given room, but not equal to the previous
/// room. Useful for avoiding "too crooked" paths.
fn rand_valid_adj_room_to(room: RoomNum, previous_room: RoomNum) -> RoomNum {
    loop {
        let r = rand_adj_room_to(room);
        if r != previous_room {
            return r;
        }
    }
}
