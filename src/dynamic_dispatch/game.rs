use map::{can_move, RoomNum};
use dynamic_dispatch::player::{Action, Player};

#[derive(Debug, PartialEq)]
pub enum RunResult {
    UserQuit
}

/// State of the game
#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub turn: usize,
    pub player: RoomNum
}

pub struct Game<'a> {
    pub player: Player<'a>
}

impl<'a> Game<'a> {
    pub fn run(&mut self) -> (Vec<State>, RunResult) {
        let mut states: Vec<State> = Vec::new();

        let mut turn = 0;

        loop {
            let state = self.get_state(turn);
            states.push(state.to_owned());
            let action = self.player.get_action(&state);

            if let Some(run_result) = self.process(&action) {
                return (states, run_result);
            }
            turn += 1;
        }
    }

    fn process(&mut self, action: &Action) -> Option<RunResult> {
        match *action {
            Action::Move(next_room) if can_move(self.player.room, next_room) => {
                self.player.move_to(next_room)
            }
            Action::Quit => Some(RunResult::UserQuit),
            _ => panic!("illegal action: {:?}", action)
        }
    }

    fn get_state(&self, turn: usize) -> State {
        State {
            turn: turn,
            player: self.player.room
        }
    }
}
