use map::RoomNum;
use dynamic_dispatch::game::{Game, RunResult, State};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Move(RoomNum),
    Quit
}

pub struct Player<'a> {
    pub room: RoomNum,
    director: &'a Director
}

impl<'a> Player<'a> {
    pub fn new(director: &'a Director, room: RoomNum) -> Self {
        Player { director: director, room: room }
    }

    pub fn get_action(&mut self, state: &State) -> Action {
        self.director.next(state)
    }

    pub fn move_to(&mut self, room: RoomNum) -> Option<RunResult> {
        self.room = room;
        None
    }
}

pub trait Director {
    fn next(&self, state: &State) -> Action;
}

#[cfg(test)]
mod player_tests {
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
        // start in room 1, move until room 12, and quit.
        let actions = vec![
            Action::Move(2),
            Action::Move(3),
            Action::Move(12),
            Action::Quit,
        ];
        let initial_state = State { turn: 0, player: 1 };
        let expected_states = create_player_state_trans_from(&initial_state, &vec![2, 3, 12]);

        let mock = &DirectorMock { actions: actions };

        let player = Player::new(mock as &Director, 1);
        let mut game = Game { player: player };
        let (actual_states, result) = game.run();

        assert_eq!(RunResult::UserQuit, result);
        assert_eq!(expected_states.len(), actual_states.len());
    }

    /// Create state transitions starting from the given initial state
    fn create_player_state_trans_from(
        initial_state: &State,
        room_trans: &Vec<RoomNum>
    ) -> Vec<State> {
        let mut result = Vec::new();
        result.push(initial_state.clone());

        for (i, room) in room_trans.iter().enumerate() {
            result.push(State { turn: i + 1, player: *room });
        }
        result
    }
}
