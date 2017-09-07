use map::{is_adj, RoomNum};
use message::Message;
use dynamic_dispatch::player::{Action, Player};
use dynamic_dispatch::pit::BottomlessPit;
use dynamic_dispatch::bat::SuperBat;
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
    Death(Cow<'a, str>),
    SnatchTo(RoomNum)
}

#[derive(Debug, PartialEq)]
pub enum RunResult {
    UserQuit,
    PlayerDeath
}

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub turn: usize,
    pub player: RoomNum,
    pub pit1: RoomNum,
    pub pit2: RoomNum,
    pub bat1: RoomNum,
    pub bat2: RoomNum
}

pub struct Game {
    turn: usize,
    pub player: Rc<Player>,
    pub pit1: Rc<BottomlessPit>,
    pub pit2: Rc<BottomlessPit>,
    pub bat1: Rc<SuperBat>,
    pub bat2: Rc<SuperBat>,
    hazzards: Vec<Rc<Hazzard>>,
    is_cheating: bool
}

impl Game {
    pub fn new() -> Self {
        let (player, pit1, pit2, bat1, bat2) = gen_unique_rooms();

        let player = Rc::new(Player::new(player));
        let pit1 = Rc::new(BottomlessPit { room: pit1 });
        let pit2 = Rc::new(BottomlessPit { room: pit2 });
        let bat1 = Rc::new(SuperBat::new(bat1));
        let bat2 = Rc::new(SuperBat::new(bat2));

        let hazzards: Vec<Rc<Hazzard>> =
            vec![pit1.clone(), pit2.clone(), bat1.clone(), bat2.clone()];

        Game {
            turn: 0,
            player,
            pit1,
            pit2,
            bat1,
            bat2,
            hazzards,
            is_cheating: false
        }
    }

    #[allow(dead_code)]
    pub fn new_with_player(p: Player, s: State) -> Self {
        let player = Rc::new(p);
        let pit1 = Rc::new(BottomlessPit { room: s.pit1 });
        let pit2 = Rc::new(BottomlessPit { room: s.pit2 });
        let bat1 = Rc::new(SuperBat::new(s.bat1));
        let bat2 = Rc::new(SuperBat::new(s.bat2));
        let hazzards: Vec<Rc<Hazzard>> = vec![pit1.clone(), pit2.clone()];

        Game {
            turn: 0,
            player,
            pit1,
            pit2,
            bat1,
            bat2,
            hazzards,
            is_cheating: false
        }
    }

    pub fn run(&mut self) -> (Vec<State>, RunResult) {
        let mut states: Vec<State> = Vec::new();

        loop {
            if self.is_cheating {
                println!("{}", self);
            }
            let player_room = self.player.room.get();
            if let Some(run_result) = self.update(player_room) {
                return (states, run_result);
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
                self.player.room.replace(next_room);
                None
            }
            Action::Quit => Some(RunResult::UserQuit),
            _ => panic!("illegal action: {:?}", action)
        }
    }

    fn update(&mut self, player_room: RoomNum) -> Option<RunResult> {
        for h in &self.hazzards {
            if let Some(update_result) = h.update(player_room) {
                match update_result {
                    UpdateResult::Warning(msg) => {
                        println!("{}", msg);
                    }
                    UpdateResult::Death(msg) => {
                        println!("{}", msg);
                        return Some(RunResult::PlayerDeath);
                    }
                    UpdateResult::SnatchTo(new_room) => {
                        self.player.room.replace(new_room);
                        println!("{}", Message::BAT_SNATCH);
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
            pit2: self.pit2.room,
            bat1: self.bat1.room,
            bat2: self.bat2.room
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
            "player turn: {}, rooms: player {}, pit1: {}, pit2: {}, bat1: {}, bat2: {}.",
            self.turn,
            self.player.room.get(),
            self.pit1.room,
            self.pit2.room,
            self.bat1.room,
            self.bat2.room
        )
    }
}
