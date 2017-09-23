#[cfg(test)]
#[path = "./game_test.rs"]
mod game_test;

use std::fmt;
use std::rc::Rc;

use player::{Action, Player};
use wumpus::Wumpus;
use pit::BottomlessPit;
use bat::SuperBat;
use map::{is_adj, RoomNum};
use message::Message;
use util::*;

pub const MAX_TRAVERSABLE: usize = 5;

pub trait Hazzard {
    fn try_update(&self, state: &State) -> Option<UpdateResult>;
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

#[derive(Clone, PartialEq, Default, Debug)]
pub struct State {
    pub player: RoomNum,
    pub wumpus: RoomNum,
    pub pit1: RoomNum,
    pub pit2: RoomNum,
    pub bat1: RoomNum,
    pub bat2: RoomNum
}

pub struct Game {
    pub player: Box<Player>,
    pub wumpus: Rc<Wumpus>,
    pub pit1_room: RoomNum,
    pub pit2_room: RoomNum,
    pub bat1_room: RoomNum,
    pub bat2_room: RoomNum,
    hazzards: Vec<Rc<Hazzard>>,
    is_cheating: bool
}

impl Game {
    pub fn new() -> Self {
        let (player_room, wumpus_room, pit1_room, pit2_room, bat1_room, bat2_room) =
            gen_unique_rooms();

        let player = box Player::new(player_room);
        let wumpus = Rc::new(Wumpus::new(wumpus_room));

        let hazzards: Vec<Rc<Hazzard>> = vec![
            wumpus.clone(),
            Rc::new(BottomlessPit { room: pit1_room }),
            Rc::new(BottomlessPit { room: pit2_room }),
            Rc::new(SuperBat::new(bat1_room)),
            Rc::new(SuperBat::new(bat2_room)),
        ];

        Game {
            player,
            wumpus,
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
        let wumpus = Rc::new(Wumpus::new(s.wumpus));
        let hazzards: Vec<Rc<Hazzard>> = vec![
            wumpus.clone(),
            Rc::new(BottomlessPit { room: s.pit1 }),
            Rc::new(BottomlessPit { room: s.pit2 }),
            Rc::new(SuperBat::new(s.bat1)),
            Rc::new(SuperBat::new(s.bat2)),
        ];

        Game {
            player: box p,
            wumpus,
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
            if let Some(run_result) = self.update() {
                return (states, run_result);
            }

            self.print_any_hazzard_warnings();

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
        let state = self.get_state();
        self.hazzards
            .iter()
            .filter_map(|h| h.try_update(&state))
            .next()
    }

    fn get_state(&self) -> State {
        State {
            player: self.player.room.get(),
            wumpus: self.wumpus.room.get(),
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
            "r rooms: player {}, wumpus {}, pit1 {}, pit2 {}, bat1 {}, bat2 {}.",
            self.player.room.get(),
            self.wumpus.room.get(),
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

type NumToRandTraverse = usize;
type RoomToStartFrom = RoomNum;

#[derive(PartialEq, Debug)]
enum ShootResult {
    Miss,
    Hit,
    Remaining(NumToRandTraverse, RoomToStartFrom)
}

fn traverse(rooms: &[RoomNum], player: RoomNum, wumpus: RoomNum) -> ShootResult {
    if rooms.len() == 0 {
        return ShootResult::Miss;
    }
    let max = MAX_TRAVERSABLE;
    let num_to_traverse = if rooms.len() < max { rooms.len() } else { max };
    let mut traversed = 0;

    let mut previous_room = rooms[0];

    if !is_adj(previous_room, player) {
        return ShootResult::Remaining(1, player);
    }

    for room in rooms.iter().take(num_to_traverse) {
        if *room == wumpus {
            return ShootResult::Hit;
        }
        traversed += 1;
    }

    let remaining = num_to_traverse - traversed;
    if remaining == 0 {
        ShootResult::Miss
    } else {
        ShootResult::Remaining(remaining, player)
    }
}
