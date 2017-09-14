#[cfg(test)]
#[path = "./game_test.rs"]
mod game_test;

use map::{is_adj, RoomNum};
use message::Message;
use dynamic_dispatch::player::{Action, Player};
use dynamic_dispatch::pit::BottomlessPit;
use dynamic_dispatch::bat::SuperBat;
use util::*;
use std::fmt;

pub trait Hazzard {
    fn try_update(&self, player_room: RoomNum) -> Option<UpdateResult>;
    fn try_warn(&self, player_room: RoomNum) -> Option<&str>;
}

pub trait RoomProvider {
    fn get_room(&self) -> RoomNum;
}

#[derive(Debug, PartialEq)]
pub enum UpdateResult {
    Death(RunResult),
    SnatchTo(RoomNum),
    BumpAndLive,
    BumpAndDie
}

#[derive(Debug, PartialEq)]
pub enum RunResult {
    UserQuit,
    DeathByBottomlessPit,
    DeathByWumpus
}

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub player: RoomNum,
    pub pit1: RoomNum,
    pub pit2: RoomNum,
    pub bat1: RoomNum,
    pub bat2: RoomNum
}

pub struct Game {
    pub player: Box<Player>,
    pub pit1_room: RoomNum,
    pub pit2_room: RoomNum,
    pub bat1_room: RoomNum,
    pub bat2_room: RoomNum,
    hazzards: Vec<Box<Hazzard>>,
    is_cheating: bool
}

impl Game {
    pub fn new() -> Self {
        let (player_room, pit1_room, pit2_room, bat1_room, bat2_room) =
            gen_unique_rooms();

        let player = box Player::new(player_room);

        let hazzards: Vec<Box<Hazzard>> = vec![
            box BottomlessPit { room: pit1_room },
            box BottomlessPit { room: pit2_room },
            box SuperBat::new(bat1_room),
            box SuperBat::new(bat2_room),
        ];

        Game {
            player,
            pit1_room,
            pit2_room,
            bat1_room,
            bat2_room,
            hazzards,
            is_cheating: false
        }
    }

    #[allow(dead_code)]
    pub fn new_with_player(p: Player, s: State) -> Self {
        let hazzards: Vec<Box<Hazzard>> = vec![
            box BottomlessPit { room: s.pit1 },
            box BottomlessPit { room: s.pit2 },
            box SuperBat::new(s.bat1),
            box SuperBat::new(s.bat2),
        ];

        Game {
            player: box p,
            pit1_room: s.pit1,
            pit2_room: s.pit2,
            bat1_room: s.bat1,
            bat2_room: s.bat2,
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
            self.print_any_hazzard_warnings();

            if let Some(run_result) = self.update() {
                return (states, run_result);
            }
            let state = self.get_state();
            states.push(state.to_owned());
            let action = self.player.get_action(&state);

            if let Some(run_result) = self.process(&action) {
                return (states, run_result);
            }
        }
    }

    fn print_any_hazzard_warnings(&self) {
        self.hazzards
            .iter()
            .filter_map(|h| h.try_warn(self.player.room.get()))
            .for_each(|warning| println!("{}", warning));
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

    fn update(&mut self) -> Option<RunResult> {
        loop {
            let mut is_snatched = false;

            if let Some(update_result) = self.try_update() {
                match update_result {
                    UpdateResult::Death(run_result) => {
                        return Some(run_result);
                    }
                    UpdateResult::SnatchTo(new_room) => {
                        self.player.room.replace(new_room);
                        is_snatched = true;
                        println!("{}", Message::BAT_SNATCH);
                    }
                    UpdateResult::BumpAndLive => {
                        println!("{}", Message::WUMPUS_BUMP);
                    }
                    UpdateResult::BumpAndDie => {
                        println!("{}", Message::WUMPUS_BUMP);
                        return Some(RunResult::DeathByWumpus);
                    }
                }
            }

            if !is_snatched {
                break;
            }
        }
        None
    }

    fn try_update(&mut self) -> Option<UpdateResult> {
        self.hazzards
            .iter()
            .filter_map(|h| h.try_update(self.player.room.get()))
            .next()
    }

    fn get_state(&self) -> State {
        State {
            player: self.player.room.get(),
            pit1: self.pit1_room,
            pit2: self.pit2_room,
            bat1: self.bat1_room,
            bat2: self.bat2_room
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
            "player rooms: player {}, pit1: {}, pit2: {}, bat1: {}, bat2: {}.",
            self.player.room.get(),
            self.pit1_room,
            self.pit2_room,
            self.bat1_room,
            self.bat2_room
        )
    }
}

impl fmt::Display for RunResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            RunResult::DeathByBottomlessPit => {
                format!("{}\n{}\n", Message::FELL_IN_PIT, Message::LOSE)
            }
            RunResult::DeathByWumpus => format!("{}\n{}\n", Message::WUMPUS_GOT_YOU, Message::LOSE),
            RunResult::UserQuit => String::from("")
        };
        write!(f, "{}", msg)
    }
}
