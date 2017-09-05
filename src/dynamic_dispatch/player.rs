use message::Prompt;
use map::RoomNum;
use dynamic_dispatch::game::State;
use map::adj_rooms_to;
use util::{get_adj_room_to, print, read_sanitized_line};
use std::cell::Cell;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Move(RoomNum),
    Quit
}

pub struct Player {
    pub room: Cell<RoomNum>,
    director: Box<Director>
}

impl Player {
    pub fn new(room: RoomNum) -> Self {
        Player {
            director: box PlayerDirector,
            room: Cell::new(room)
        }
    }

    pub fn get_action(&self, state: &State) -> Action {
        self.director.next(state)
    }
}

pub trait Director {
    fn next(&self, state: &State) -> Action;
}

pub struct PlayerDirector;

impl Director for PlayerDirector {
    fn next(&self, state: &State) -> Action {
        let room_num = state.player;
        loop {
            println!("You are in room {}", room_num);
            let (a, b, c) = adj_rooms_to(room_num);
            println!("Tunnel leads to {} {} {}", a, b, c);
            print(Prompt::ACTION);

            match read_sanitized_line().as_ref() {
                "M" => return Action::Move(get_adj_room_to(room_num)),
                "Q" => return Action::Quit,
                _ => continue
            }
        }
    }
}

#[cfg(test)]
mod player_tests {
    use dynamic_dispatch::game::{Game, RunResult};
    use super::*;

    struct DirectorMock {
        actions: Vec<Action>
    }

    impl Director for DirectorMock {
        fn next(&self, state: &State) -> Action {
            if state.turn >= self.actions.len() {
                panic!("turn many turns taken!")
            }
            self.actions[state.turn].to_owned()
        }
    }

    #[test]
    fn can_move_player_and_quit() {
        // start in room 1, move until in room 12, and quit.
        let actions = vec![
            Action::Move(2),
            Action::Move(3),
            Action::Move(12),
            Action::Quit,
        ];
        let initial_state = State {
            turn: 0,
            player: 1,
            pit1: 19,
            pit2: 20
        };
        let expected_states = create_player_state_trans_from(&initial_state, &vec![2, 3, 12]);

        let mock = box DirectorMock { actions: actions };

        let player = Player {
            room: Cell::new(1),
            director: mock
        };
        let mut game = Game::new_with_player(player, 19, 20);
        let (actual_states, result) = game.run();

        assert_eq!(RunResult::UserQuit, result);
        assert_eq!(expected_states, actual_states);
    }

    #[test]
    fn can_move_and_fall_in_pit() {
        // move into bottomless pit.
        let actions = vec![Action::Move(2)];
        let mock = box DirectorMock { actions: actions };

        let player = Player {
            room: Cell::new(1),
            director: mock
        };

        let mut game = Game::new_with_player(player, 2, 20);

        let expected_states = vec![
            State {
                turn: 0,
                player: 1,
                pit1: 2,
                pit2: 20
            },
        ];
        let (actual_states, result) = game.run();

        assert_eq!(RunResult::PlayerDeath, result);
        assert_eq!(expected_states, actual_states);
    }

    /// Create state transitions starting from the given initial state
    fn create_player_state_trans_from(
        initial_state: &State,
        room_trans: &Vec<RoomNum>
    ) -> Vec<State> {
        let mut result = Vec::new();
        result.push(initial_state.clone());

        for (i, room) in room_trans.iter().enumerate() {
            result.push(State {
                turn: i + 1,
                player: *room,
                pit1: initial_state.pit1,
                pit2: initial_state.pit2
            });
        }
        result
    }
}
