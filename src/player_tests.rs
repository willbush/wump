use rand::{thread_rng, Rng};
use std::cell::RefCell;

use super::*;
use game::game_test::new_game_from;
use game::{RunResult, MAX_TRAVERSABLE};
use map::map_tests::gen_rand_valid_path_of_len;
use map::{rand_adj_room_to, rand_room, rand_valid_adj_room_to};

struct MockDirector {
    pub actions: RefCell<Vec<Action>>
}

impl Director for MockDirector {
    fn next(&self, _: &State) -> Action {
        let mut actions = self.actions.borrow_mut();
        actions.pop().expect("Ran out of actions to pop!")
    }
}

#[test]
fn move_player_and_quit() {
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
        bat2: 16,
        arrow_count: 5,
        is_cheating: false
    };

    let mut game = new_game_from(player, initial_state);

    assert_eq!(RunResult::Quit, game.run());
}

#[test]
fn move_and_fall_in_pit() {
    // move into bottomless pit.
    let player = create_mock_directed_player(1, vec![Action::Move(2)]);

    let initial_state = State {
        player: 1,
        wumpus: 17,
        pit1: 2,
        pit2: 18,
        bat1: 19,
        bat2: 20,
        arrow_count: 5,
        is_cheating: false
    };

    let mut game = new_game_from(player, initial_state.to_owned());

    assert_eq!(RunResult::KilledByPit, game.run());
}

#[test]
fn running_out_of_arrows_causes_loss() {
    let player_room = 1;
    let actions = vec![Action::Shoot(vec![player_room, 2])];
    let player = Player {
        room: Cell::new(player_room),
        arrow_count: Cell::new(1),
        director: box MockDirector {
            actions: RefCell::new(actions)
        }
    };

    let initial_state = State {
        player: player_room,
        wumpus: 17,
        pit1: 2,
        pit2: 18,
        bat1: 19,
        bat2: 20,
        arrow_count: 1,
        is_cheating: false
    };

    let mut game = new_game_from(player, initial_state.to_owned());

    assert_eq!(RunResult::RanOutOfArrows, game.run());
}

pub fn create_mock_directed_player(room: RoomNum, actions: Vec<Action>) -> Player {
    Player {
        room: Cell::new(room),
        arrow_count: Cell::new(5),
        director: box MockDirector {
            actions: RefCell::new(actions)
        }
    }
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
        assert!(is_too_crooked(&path));
    };

    // case2: v-A-B-A
    let case2 = || {
        let v = rand_room();
        let a = rand_adj_room_to(v);
        let b = rand_valid_adj_room_to(a, v);
        let path = [v, a, b, a];
        assert!(is_too_crooked(&path));
    };

    // case3: v-v-A-B-A
    let case3 = || {
        let v1 = rand_room();
        let v2 = rand_adj_room_to(v1);
        let a = rand_valid_adj_room_to(v2, v1);
        let b = rand_valid_adj_room_to(a, v2);
        let path = [v1, v2, a, b, a];
        assert!(is_too_crooked(&path));
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
        let valid_path = gen_rand_valid_path_of_len(num_to_traverse);
        assert!(!is_too_crooked(&valid_path), "{:?}", &valid_path);
    });
}

fn perform_trial(trial_count: u32, trial: &dyn Fn()) {
    (0..trial_count).for_each(|_| trial());
}
