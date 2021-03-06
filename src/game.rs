#[cfg(test)]
#[path = "./game_test.rs"]
pub mod game_test;

use std::fmt;
use std::rc::Rc;

use bat::SuperBat;
use map::{gen_rand_valid_path_from, is_adj, RoomNum};
use message::Message;
use pit::BottomlessPit;
use player::{Action, Player};
use util::*;
use wumpus::Wumpus;

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
    Quit,
    Win,
    Suicide,
    RanOutOfArrows,
    KilledByPit,
    KilledByWumpus
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct State {
    pub player: RoomNum,
    pub wumpus: RoomNum,
    pub pit1: RoomNum,
    pub pit2: RoomNum,
    pub bat1: RoomNum,
    pub bat2: RoomNum,
    pub arrow_count: u8,
    pub is_cheating: bool
}

pub struct Game {
    pub player: Box<Player>,
    pub wumpus: Rc<Wumpus>,
    pub pit1_room: RoomNum,
    pub pit2_room: RoomNum,
    pub bat1_room: RoomNum,
    pub bat2_room: RoomNum,
    hazzards: Vec<Rc<dyn Hazzard>>,
    is_cheating: bool
}

impl Game {
    pub fn new(is_cheating: bool) -> Self {
        let (player, wumpus, pit1, pit2, bat1, bat2) = gen_unique_rooms();

        Game::new_using(&State {
            player,
            wumpus,
            pit1,
            pit2,
            bat1,
            bat2,
            is_cheating,
            ..Default::default()
        })
    }

    pub fn new_using(s: &State) -> Self {
        let player = box Player::new(s.player);
        let wumpus = Rc::new(Wumpus::new(s.wumpus));
        let wumpus_clone = Rc::clone(&wumpus);

        let hazzards: Vec<Rc<dyn Hazzard>> = vec![
            wumpus_clone,
            Rc::new(BottomlessPit { room: s.pit1 }),
            Rc::new(BottomlessPit { room: s.pit2 }),
            Rc::new(SuperBat::new(s.bat1)),
            Rc::new(SuperBat::new(s.bat2)),
        ];

        Game {
            player,
            wumpus,
            hazzards,
            pit1_room: s.pit1,
            pit2_room: s.pit2,
            bat1_room: s.bat1,
            bat2_room: s.bat2,
            is_cheating: s.is_cheating
        }
    }

    pub fn run(&mut self) -> RunResult {
        loop {
            if self.is_cheating {
                println!("{}", self);
            }
            if let Some(run_result) = self.update() {
                return run_result;
            }
            self.print_any_hazzard_warnings();

            let action = self.player.get_action(&self.get_state());

            if let Some(run_result) = self.process(&action) {
                return run_result;
            }
        }
    }

    pub fn get_state(&self) -> State {
        State {
            player: self.player.room.get(),
            wumpus: self.wumpus.room.get(),
            pit1: self.pit1_room,
            pit2: self.pit2_room,
            bat1: self.bat1_room,
            bat2: self.bat2_room,
            arrow_count: self.player.arrow_count.get(),
            is_cheating: self.is_cheating
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
                self.player.room.set(next_room);
                None
            }
            Action::Quit => Some(RunResult::Quit),
            Action::Shoot(ref rooms) => shoot(rooms, &self.get_state()),
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
                        self.player.room.set(new_room);
                        is_snatched = true;
                        println!("{}", Message::BAT_SNATCH);
                    }
                    UpdateResult::BumpAndLive => {
                        println!("{}", Message::WUMPUS_BUMP);
                    }
                    UpdateResult::BumpAndDie => {
                        println!("{}", Message::WUMPUS_BUMP);
                        return Some(RunResult::KilledByWumpus);
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
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rooms: player {}, wumpus {}, pit1 {}, pit2 {}, bat1 {}, bat2 {}.",
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
            RunResult::KilledByPit => format!("{}\n{}\n", Message::FELL_IN_PIT, Message::LOSE),
            RunResult::KilledByWumpus => {
                format!("{}\n{}\n", Message::WUMPUS_GOT_YOU, Message::LOSE)
            }
            RunResult::Suicide => format!("{}\n{}\n", Message::ARROW_GOT_YOU, Message::LOSE),
            RunResult::RanOutOfArrows => format!("{}\n{}\n", Message::OUT_OF_ARROWS, Message::LOSE),
            RunResult::Win => format!("{}\n", Message::WIN),
            RunResult::Quit => String::from("")
        };
        write!(f, "{}", msg)
    }
}

type NumRemaining = usize;
type LastTraversed = usize;

#[derive(PartialEq, Debug)]
enum ShootResult {
    Miss,
    Hit,
    Suicide,
    Remaining(NumRemaining, LastTraversed)
}

fn shoot(rooms: &[RoomNum], s: &State) -> Option<RunResult> {
    // rooms length must contain the player and at least one other room to
    // traverse.
    if rooms.len() < 2 || rooms.len() > MAX_TRAVERSABLE + 1 {
        panic!(
            "shoot function called with a length out of bounds: {}",
            rooms.len()
        );
    }
    match traverse(rooms, s) {
        ShootResult::Hit => Some(RunResult::Win),
        ShootResult::Suicide => Some(RunResult::Suicide),
        ShootResult::Miss => {
            println!("{}", String::from(Message::MISSED));
            if s.arrow_count == 0 {
                Some(RunResult::RanOutOfArrows)
            } else {
                None
            }
        }
        ShootResult::Remaining(remaining, last_traversed) => {
            let remaining_rooms = gen_rand_valid_path_from(remaining, last_traversed);
            shoot(&remaining_rooms, s) // recursive call at most once.
        }
    }
}

/// Traverse crooked arrow across rooms starting from the player's room.
fn traverse(rooms: &[RoomNum], s: &State) -> ShootResult {
    for (num_traversed, w) in rooms.windows(2).enumerate() {
        let a = w[0];
        let b = w[1];

        if !is_adj(a, b) {
            return ShootResult::Remaining(rooms.len() - num_traversed, a);
        }
        println!("{}", b);
        if b == s.player {
            return ShootResult::Suicide;
        }
        if b == s.wumpus {
            return ShootResult::Hit;
        }
    }
    ShootResult::Miss
}
