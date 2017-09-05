use map::{is_adj, RoomNum};
use dynamic_dispatch::player::{Action, Player};
use dynamic_dispatch::pit::BottomlessPit;
use util::*;
use std::fmt;
use std::rc::Rc;
use std::borrow::Cow;

pub trait Hazzard {
    fn update(&self, player_room: RoomNum) -> Option<UpdateResult>;
}

#[derive(Debug, PartialEq)]
pub enum UpdateResult<'a> {
    Warning(Cow<'a, str>),
    Death(Cow<'a, str>)
}

#[derive(Debug, PartialEq)]
pub enum RunResult {
    UserQuit,
    PlayerDeath
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
    pub player: Rc<Player>,
    pub pit1: Rc<BottomlessPit>,
    pub pit2: Rc<BottomlessPit>,
    hazzards: Vec<Rc<Hazzard>>,
    is_cheating: bool
}

impl Game {
    pub fn new() -> Self {
        let (player, pit1, pit2, ..) = gen_unique_rooms();

        let player = Rc::new(Player::new(player));
        let pit1 = Rc::new(BottomlessPit { room: pit1 });
        let pit2 = Rc::new(BottomlessPit { room: pit2 });

        let hazzards: Vec<Rc<Hazzard>> = vec![pit1.clone(), pit2.clone()];

        Game {
            turn: 0,
            player: player,
            pit1: pit1,
            pit2: pit2,
            is_cheating: false,
            hazzards: hazzards
        }
    }

    #[allow(dead_code)]
    pub fn new_with_player(player: Player, pit1: RoomNum, pit2: RoomNum) -> Self {
        let player = Rc::new(player);
        let pit1 = Rc::new(BottomlessPit { room: pit1 });
        let pit2 = Rc::new(BottomlessPit { room: pit2 });
        let hazzards: Vec<Rc<Hazzard>> = vec![pit1.clone(), pit2.clone()];
        Game {
            turn: 0,
            player: player,
            pit1: pit1,
            pit2: pit2,
            hazzards: hazzards,
            is_cheating: false
        }
    }

    pub fn run(&mut self) -> (Vec<State>, RunResult) {
        let mut states: Vec<State> = Vec::new();

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
            Action::Move(next_room) if is_adj(self.player.room.get(), next_room) => {
                self.move_player_and_update(next_room)
            }
            Action::Quit => Some(RunResult::UserQuit),
            _ => panic!("illegal action: {:?}", action)
        }
    }

    fn move_player_and_update(&self, next_room: RoomNum) -> Option<RunResult> {
        self.player.room.replace(next_room);

        for h in &self.hazzards {
            if let Some(update_result) = h.update(next_room) {
                match update_result {
                    UpdateResult::Warning(msg) => println!("{}", msg),
                    UpdateResult::Death(msg) => {
                        println!("{}", msg);
                        return Some(RunResult::PlayerDeath);
                    }
                }
            }
        }
        None
    }

    fn get_state(&self) -> State {
        State {
            turn: self.turn,
            player: self.player.room.get(),
            pit1: self.pit1.room,
            pit2: self.pit2.room
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
            self.player.room.get(),
            self.pit1.room,
            self.pit2.room,
        )
    }
}
