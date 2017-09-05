use map::{can_move, RoomNum};
use dynamic_dispatch::player::{Action, Player};
use dynamic_dispatch::pit::BottomlessPit;
use message::Message;
use util::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RunResult {
    UserQuit,
    DeathByBottomlessPit
}

impl fmt::Display for RunResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            RunResult::DeathByBottomlessPit => {
                format!("{}\n{}", Message::FELL_IN_PIT, Message::LOSE)
            }
            RunResult::UserQuit => "".into()
        };
        write!(f, "{}", msg)
    }
}

/// State of the game
#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub turn: usize,
    pub player: RoomNum,
    pub pit1: RoomNum,
    pub pit2: RoomNum
}

pub struct Game {
    turn: usize,
    pub player: Player,
    pub pit1: BottomlessPit,
    pub pit2: BottomlessPit,
    is_cheating: bool
}

impl Game {
    pub fn new() -> Self {
        let (player, pit1, pit2, ..) = gen_unique_rooms();

        Game {
            turn: 0,
            player: Player::new(player),
            pit1: BottomlessPit { room: pit1 },
            pit2: BottomlessPit { room: pit2 },
            is_cheating: false
        }
    }

    pub fn new_with_player(player: Player, pit1: RoomNum, pit2: RoomNum) -> Self {
        Game {
            turn: 0,
            player: player,
            pit1: BottomlessPit { room: pit1 },
            pit2: BottomlessPit { room: pit2 },
            is_cheating: false
        }
    }

    pub fn run(&mut self) -> (Vec<State>, RunResult) {
        let mut states: Vec<State> = Vec::new();

        let mut turn = 0;

        loop {
            if self.is_cheating {
                println!("{}", self);
            }
            let state = self.get_state();
            states.push(state.to_owned());
            let action = self.player.get_action(&state);

            if let Some(run_result) = self.process(&action) {
                return (states, run_result);
            }
            self.turn += 1;
        }
    }

    fn process(&mut self, action: &Action) -> Option<RunResult> {
        match *action {
            Action::Move(next_room) if can_move(self.player.room, next_room) => {
                self.move_player(next_room)
            }
            Action::Quit => Some(RunResult::UserQuit),
            _ => panic!("illegal action: {:?}", action)
        }
    }

    fn get_state(&self) -> State {
        State {
            turn: self.turn,
            player: self.player.room,
            pit1: self.pit1.room,
            pit2: self.pit2.room
        }
    }

    fn move_player(&mut self, next_room: RoomNum) -> Option<RunResult> {
        self.player.room = next_room;

        if self.player.room == self.pit1.room || self.player.room == self.pit2.room {
            Some(RunResult::DeathByBottomlessPit)
        } else {
            None
        }
    }

    pub fn enable_cheat_mode(&mut self) {
        self.is_cheating = true;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "player turn: {}, rooms: player {}, pit1: {}, pit2: {}.",
            self.turn,
            self.player.room,
            self.pit1.room,
            self.pit2.room,
        )
    }
}
