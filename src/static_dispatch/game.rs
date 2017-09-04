use std::fmt;
use rand::{thread_rng, Rng};
use message::{Message, Prompt};
use map::{adj_rooms_to, can_move, RoomNum, MAP};
use util::*;

pub struct Game<'a, D: Director + 'a, P: Provider + 'a> {
    player: Player,
    pit1: BottomlessPit,
    pit2: BottomlessPit,
    bat1: SuperBat<'a, P>,
    bat2: SuperBat<'a, P>,
    director: &'a mut D,
    turn: usize,
    is_cheating: bool
}

impl<'a, D, P> Game<'a, D, P>
    where D: Director,
          P: 'a + Provider {
    pub fn new(director: &'a mut D, provider: &'a P) -> Self {
        let (player, pit1, pit2, bat1, bat2) = gen_unique_rooms();

        Game {
            player: Player { room: player },
            pit1: BottomlessPit { room: pit1 },
            pit2: BottomlessPit { room: pit2 },
            bat1: SuperBat { room: bat1, provider: provider },
            bat2: SuperBat { room: bat2, provider: provider },
            director: director,
            turn: 0,
            is_cheating: false
        }
    }

    fn new_with_initial_state(director: &'a mut D, provider: &'a P, state: State) -> Self {
        Game {
            player: Player { room: state.player },
            pit1: BottomlessPit { room: state.pit1 },
            pit2: BottomlessPit { room: state.pit2 },
            bat1: SuperBat {
                room: state.bat1,
                provider: provider
            },
            bat2: SuperBat {
                room: state.bat2,
                provider: provider
            },
            director: director,
            turn: state.turn,
            is_cheating: false
        }
    }

    pub fn enable_cheat_mode(&mut self) { self.is_cheating = true; }

    pub fn run(&mut self) -> RunResult {
        loop {
            if self.is_cheating {
                println!("{}", self);
            }
            let state = self.get_state();
            let action = self.director.next(&state);

            match action {
                Action::Move(next_room) if can_move(self.player.room, next_room) => {
                    if let Some(run_result) = self.move_player(next_room) {
                        return run_result;
                    }
                }
                Action::Quit => return RunResult::UserQuit,
                _ => panic!("illegal action {:?}", action)
            }
            self.turn += 1;
        }
    }

    fn move_player(&mut self, next_room: RoomNum) -> Option<RunResult> {
        self.player.room = next_room;

        loop {
            let mut is_snatched = false;

            if self.player.room == self.pit1.room || self.player.room == self.pit2.room {
                return Some(RunResult::DeathByBottomlessPit);
            }
            if self.player.room == self.bat1.room {
                self.bat1.snatch(&mut self.player);
                is_snatched = true;
            } else if self.player.room == self.bat2.room {
                self.bat2.snatch(&mut self.player);
                is_snatched = true;
            }
            if !is_snatched {
                return None;
            }
        }
    }

    fn get_state(&self) -> State {
        State {
            turn: self.turn,
            player: self.player.room,
            pit1: self.pit1.room,
            pit2: self.pit2.room,
            bat1: self.bat1.room,
            bat2: self.bat2.room
        }
    }
}

impl<'a, D, P> PartialEq for Game<'a, D, P>
    where D: Director,
          P: 'a + Provider {
    fn eq(&self, other: &Game<D, P>) -> bool { self.get_state() == other.get_state() }
}

impl<'a, D, P> fmt::Debug for Game<'a, D, P>
    where D: Director,
          P: 'a + Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "State {{ turn: {} player: {} pit1: {} pit2: {} bat1: {}, bat2: {} }}",
            self.turn,
            self.player.room,
            self.pit1.room,
            self.pit2.room,
            self.bat1.room,
            self.bat2.room
        )
    }
}

impl<'a, D, P> fmt::Display for Game<'a, D, P>
    where D: Director,
          P: 'a + Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "player turn: {}, rooms: player {}, pit1: {}, pit2: {}, bat1: {}, bat2: {}.",
            self.turn,
            self.player.room,
            self.pit1.room,
            self.pit2.room,
            self.bat1.room,
            self.bat2.room
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum RunResult {
    DeathByBottomlessPit,
    UserQuit
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
    // The game turn starting from 0. Each player turn increments this by one.
    turn: usize,
    pub player: RoomNum,
    pub pit1: RoomNum,
    pub pit2: RoomNum,
    pub bat1: RoomNum,
    pub bat2: RoomNum
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Move(RoomNum),
    Quit
}

pub trait Director {
    fn next(&mut self, state: &State) -> Action;
}

pub struct PlayerDirector;

impl Director for PlayerDirector {
    fn next(&mut self, state: &State) -> Action {
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

struct Player {
    room: RoomNum
}

impl Player {
    fn new(room: RoomNum) -> Self { Player { room: room } }
}

struct BottomlessPit {
    room: RoomNum
}

pub trait Provider {
    fn get_room(&self) -> RoomNum;
}

struct SuperBat<'a, P: 'a + Provider> {
    room: RoomNum,
    provider: &'a P
}

impl<'a, P> SuperBat<'a, P>
    where P: Provider {
    fn snatch(&self, player: &mut Player) { player.room = self.provider.get_room(); }
}

pub struct RandProvider;

impl Provider for RandProvider {
    fn get_room(&self) -> RoomNum { thread_rng().gen_range(1, MAP.len() + 1) }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    struct DirectorMock {
        actions: Vec<Action>
    }

    impl Director for DirectorMock {
        fn next(&mut self, state: &State) -> Action {
            match self.actions.pop() {
                Some(action) => action,
                None => panic!("too many expected actions")
            }
        }
    }

    struct DirectorSpy {
        actions: Vec<Action>,
        expected_states: Vec<State>
    }

    impl DirectorSpy {
        fn new(actions: Vec<Action>, exptected_states: Vec<State>) -> Self {
            DirectorSpy {
                actions: actions,
                expected_states: exptected_states
            }
        }
    }

    impl Director for DirectorSpy {
        fn next(&mut self, actual_states: &State) -> Action {
            let turn = actual_states.turn;
            if turn >= self.expected_states.len() {
                panic!("unexpected number of turns taken! turn: {}", turn);
            }

            let expected_states = &self.expected_states[turn];

            assert_eq!(*expected_states, *actual_states);

            match self.actions.pop() {
                Some(action) => action,
                None => panic!("too many expected actions")
            }
        }
    }

    struct DummyProvider;

    impl Provider for DummyProvider {
        fn get_room(&self) -> RoomNum { 0 }
    }

    struct RandRoomProvider {
        snatch_to_room1: RoomNum,
        snatch_to_room2: RoomNum
    }

    impl Provider for RandRoomProvider {
        fn get_room(&self) -> RoomNum {
            let is_rand_true = thread_rng().gen::<bool>();
            if is_rand_true {
                self.snatch_to_room1
            } else {
                self.snatch_to_room2
            }
        }
    }

    struct MockProvider {
        snatch_to_room: RoomNum
    }

    impl Provider for MockProvider {
        fn get_room(&self) -> RoomNum { self.snatch_to_room }
    }

    #[test]
    fn can_move_and_quit() {
        let actions = vec![
            Action::Quit,
            Action::Move(12),
            Action::Move(3),
            Action::Move(2),
        ];
        let initial_state = State {
            turn: 0,
            player: 1,
            pit1: 17,
            pit2: 18,
            bat1: 19,
            bat2: 20
        };
        let expected_states = create_player_state_trans_from(&initial_state, &vec![2, 3, 12]);

        let director = &mut DirectorSpy::new(actions, expected_states);
        let provider = &DummyProvider;

        let mut game = Game::new_with_initial_state(director, provider, initial_state);

        assert_eq!(RunResult::UserQuit, game.run())
    }

    #[test]
    fn can_move_and_fall_in_pit() {
        let actions = vec![Action::Move(2)]; // move into bottomless pit
        let initial_state = State {
            turn: 0,
            player: 1,
            pit1: 2,
            pit2: 3,
            bat1: 4,
            bat2: 5
        };
        let director = &mut DirectorSpy::new(actions, vec![initial_state.clone()]);
        let provider = &DummyProvider;
        let mut game = Game::new_with_initial_state(director, provider, initial_state);

        assert_eq!(RunResult::DeathByBottomlessPit, game.run())
    }

    #[test]
    fn super_bat_can_snatch() {
        // snatch and send player to room 20
        let expected_room = 20;

        let provider = &MockProvider { snatch_to_room: expected_room };
        let bat = SuperBat {
            room: expected_room,
            provider: provider
        };

        let mut player = &mut Player::new(1);
        bat.snatch(player);

        assert_eq!(expected_room, player.room);
    }

    #[test]
    fn super_bat_can_snatch_player_to_pit() {
        let adj_room_to_player = 2;
        let other_room = 5;
        let is_bat1_adj = thread_rng().gen::<bool>();
        // move into super bats which are set to snatch into a pit
        let initial_state = State {
            turn: 0,
            player: 1,
            bat1: if is_bat1_adj { adj_room_to_player } else { other_room },
            pit1: 3,
            pit2: 4,
            bat2: if !is_bat1_adj { adj_room_to_player } else { other_room }
        };
        let mut director = &mut DirectorMock { actions: vec![Action::Move(2)] };
        // snatch back to the same room as the bat or the pit randomly
        let provider = &RandRoomProvider {
            snatch_to_room1: 3,
            snatch_to_room2: 2
        };
        let mut game = Game::new_with_initial_state(director, provider, initial_state);

        assert_eq!(RunResult::DeathByBottomlessPit, game.run())
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
                turn: i + 1, // assumes initial state starts at 0
                player: *room,
                pit1: initial_state.pit1,
                pit2: initial_state.pit2,
                bat1: initial_state.bat1,
                bat2: initial_state.bat2
            });
        }
        result
    }
}
