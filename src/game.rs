use std::fmt;
use std::collections::HashSet;
use rand::{thread_rng, Rng};
use std::io;
use std::io::Write;

// The game map in Hunt the Wumpus is laid out as a dodecahedron. The vertices
// of the dodecahedron are considered rooms, and each room has 3 adjacent rooms.
// A room is adjacent if it has a line segment directly from one vertex to
// another. Here we have a 2D array where the first dimension represents the 20
// rooms (index + 1 == room number). the second dimension is an array of the
// adjacent rooms. I just hard coded some valid room values here for ease, but
// there is a formula that could be used to derive instead.
pub static MAP: [[RoomNum; 3]; 20] = [
    [2, 5, 8],
    [1, 3, 10],
    [2, 4, 12],
    [3, 5, 14],
    [1, 4, 6],
    [5, 7, 15],
    [6, 8, 17],
    [1, 7, 9],
    [8, 10, 18],
    [2, 9, 11],
    [10, 12, 19],
    [3, 11, 13],
    [12, 14, 20],
    [4, 13, 15],
    [6, 14, 16],
    [15, 17, 20],
    [7, 16, 18],
    [9, 17, 19],
    [11, 18, 20],
    [13, 16, 19],
];

type RoomNum = usize;

pub struct Game<'a, D: Director + 'a, P: Provider + 'a> {
    player: Player,
    pit1: BottomlessPit,
    pit2: BottomlessPit,
    bat1: SuperBat<'a, P>,
    bat2: SuperBat<'a, P>,
    director: &'a mut D,
    turn: usize,
}

impl<'a, D, P> Game<'a, D, P>
where
    D: Director,
    P: 'a + Provider,
{
    pub fn new(director: &'a mut D, provider: &'a P) -> Self {
        let (player, pit1, pit2, bat1, bat2) = gen_unique_rooms();

        Game {
            turn: 0,
            player: Player { room: player },
            pit1: BottomlessPit { room: pit1 },
            pit2: BottomlessPit { room: pit2 },
            bat1: SuperBat {
                room: bat1,
                provider: provider,
            },
            bat2: SuperBat {
                room: bat2,
                provider: provider,
            },
            director: director,
        }
    }

    fn new_with_initial_state(director: &'a mut D, provider: &'a P, state: State) -> Self {
        Game {
            turn: state.turn,
            player: Player { room: state.player },
            pit1: BottomlessPit { room: state.pit1 },
            pit2: BottomlessPit { room: state.pit2 },
            bat1: SuperBat {
                room: state.bat1,
                provider: provider,
            },
            bat2: SuperBat {
                room: state.bat2,
                provider: provider,
            },
            director: director,
        }
    }

    pub fn run(&mut self) -> RunResult {
        let mut player = self.player.room;
        let pit1 = self.pit1.room;
        let pit2 = self.pit2.room;

        loop {
            let state = self.get_state();
            let action = self.director.next(&state);

            match action {
                Action::Move(next_room) if can_move(player, next_room) => {
                    player = next_room;

                    if player == pit1 || player == pit2 {
                        return RunResult::DeathByBottomlessPit;
                    }
                }
                Action::Quit => return RunResult::UserQuit,
                _ => panic!("illegal action {:?}", action),
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
            bat2: self.bat2.room,
        }
    }
}

impl<'a, D, P> PartialEq for Game<'a, D, P>
where
    D: Director,
    P: 'a + Provider,
{
    fn eq(&self, other: &Game<D, P>) -> bool {
        self.get_state() == other.get_state()
    }
}

impl<'a, D, P> fmt::Debug for Game<'a, D, P>
where
    D: Director,
    P: 'a + Provider,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let turn = self.turn;
        let player = self.player.room;
        let pit1 = self.pit1.room;
        let pit2 = self.pit2.room;

        write!(
            f,
            "State {{ turn: {} player: {} pit1: {} pit2: {} }}",
            turn,
            player,
            pit1,
            pit2
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum RunResult {
    DeathByBottomlessPit,
    UserQuit,
}

impl fmt::Display for RunResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            RunResult::DeathByBottomlessPit => {
                "YYYIIIIEEEE... fell in a pit!\n\
                 Ha ha ha - you lose!\n"
            }
            RunResult::UserQuit => "",
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
    pub bat2: RoomNum,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Move(RoomNum),
    Quit,
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
            print("Shoot, Move, or Quit (S, M, Q) ");

            match read_sanitized_line().as_ref() {
                "M" => return Action::Move(get_adj_room_to(room_num)),
                "Q" => return Action::Quit,
                _ => continue,
            }
        }
    }
}

struct Player {
    room: RoomNum,
}

impl Player {
    fn new(room: RoomNum) -> Self {
        Player { room: room }
    }
}

struct BottomlessPit {
    room: RoomNum,
}

pub trait Provider {
    fn get_room(&self) -> RoomNum;
}

struct SuperBat<'a, P: 'a + Provider> {
    room: RoomNum,
    provider: &'a P,
}

impl<'a, P> SuperBat<'a, P>
where
    P: Provider,
{
    fn snatch(&self, player: &mut Player) {
        player.room = self.provider.get_room();
    }
}

pub struct RandProvider;

impl Provider for RandProvider {
    fn get_room(&self) -> RoomNum {
        thread_rng().gen_range(1, MAP.len() + 1)
    }
}

fn get_adj_room_to(room: RoomNum) -> RoomNum {
    print("Where to? ");

    loop {
        let input = read_sanitized_line();

        match input.parse::<RoomNum>() {
            Ok(next) if can_move(room, next) => return next,
            _ => print("Not Possible - Where to? "),
        }
    }
}

// Reads a line from stdin, trims it, and returns it as upper case.
fn read_sanitized_line() -> String {
    read_trimed_line().to_uppercase()
}

fn read_trimed_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}

fn can_move(next: RoomNum, current: RoomNum) -> bool {
    if current > 0 && current <= MAP.len() {
        let adj_rooms = MAP[current - 1];
        let adj1 = adj_rooms[0];
        let adj2 = adj_rooms[1];
        let adj3 = adj_rooms[2];

        next == adj1 || next == adj2 || next == adj3
    } else {
        false
    }
}

// Print without new line and flush to force it to show up.
fn print(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

fn gen_unique_rooms() -> (RoomNum, RoomNum, RoomNum, RoomNum, RoomNum) {
    let mut taken_rooms = HashSet::new();

    let player_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(player_room);
    let pit1_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(pit1_room);
    let pit2_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(pit2_room);
    let bat1_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(bat1_room);
    let bat2_room = gen_unique_rand_room(&taken_rooms);

    (player_room, pit1_room, pit2_room, bat1_room, bat2_room)
}

fn gen_unique_rand_room(taken_rooms: &HashSet<RoomNum>) -> RoomNum {
    let mut rng = thread_rng();

    loop {
        let room: RoomNum = rng.gen_range(1, MAP.len() + 1);

        if !taken_rooms.contains(&room) {
            return room;
        }
    }
}

fn adj_rooms_to(room: RoomNum) -> (RoomNum, RoomNum, RoomNum) {
    let adj_rooms = MAP[room - 1];
    (adj_rooms[0], adj_rooms[1], adj_rooms[2])
}

#[cfg(test)]
mod game_tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct DirectorDummy;

    impl Director for DirectorDummy {
        fn next(&mut self, _: &State) -> Action {
            Action::Quit
        }
    }

    struct DirectorSpy {
        actions: Vec<Action>,
        expected_states: Vec<State>,
    }

    impl DirectorSpy {
        fn new(actions: Vec<Action>, exptected_states: Vec<State>) -> Self {
            DirectorSpy {
                actions: actions,
                expected_states: exptected_states,
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
                None => panic!("too many pops"),
            }
        }
    }

    struct DummyProvider;

    impl Provider for DummyProvider {
        fn get_room(&self) -> RoomNum {
            0
        }
    }

    struct MockProvider {
        room: RoomNum,
    }

    impl Provider for MockProvider {
        fn get_room(&self) -> RoomNum {
            self.room
        }
    }

    // One property that exists for the map is if current room is in bounds of
    // the map and strictly less than the map length, then we should always be
    // able to move to the room (current + 1).
    #[quickcheck]
    fn can_move_to_next_room_num_property(current: RoomNum) -> bool {
        let can_move = can_move(current, current + 1);

        if current > 0 && current < MAP.len() {
            can_move
        } else {
            !can_move
        }
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
            bat2: 20,
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
            bat2: 5,
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

        let provider = &MockProvider {
            room: expected_room,
        };
        let bat = SuperBat {
            room: expected_room,
            provider: provider,
        };

        let mut player = &mut Player::new(1);
        bat.snatch(player);

        assert_eq!(expected_room, player.room);
    }

    #[test]
    fn super_bat_can_snatch_player_to_pit() {
        // move into super bats which are set to snatch into a pit
        let actions = vec![Action::Move(2)];
        let initial_state = State {
            turn: 0,
            player: 1,
            pit1: 2,
            pit2: 3,
            bat1: 4,
            bat2: 5,
        };
        let mut director = &mut DirectorSpy::new(actions, vec![initial_state.clone()]);
        let provider = &DummyProvider;
        let mut game = Game::new_with_initial_state(director, provider, initial_state);

        assert_eq!(RunResult::DeathByBottomlessPit, game.run())
    }

    /// Create state transitions starting from the given initial state
    fn create_player_state_trans_from(
        initial_state: &State,
        room_trans: &Vec<RoomNum>,
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
                bat2: initial_state.bat2,
            });
        }
        result
    }
}
