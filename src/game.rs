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

pub struct Game<T: ActionProvider> {
    positions: Pos,
    action_provider: T,
}

impl<T: ActionProvider> Game<T> {
    pub fn new(provider: T) -> Self {
        let (player, pit1, pit2) = gen_unique_rooms();

        let initial_positions = Pos::new(player, pit1, pit2);

        Game {
            positions: initial_positions,
            action_provider: provider,
        }
    }

    pub fn run(&mut self) -> RunResult {
        let mut player = self.positions.player;
        let mut turn = self.positions.turn;
        let pit1 = self.positions.pit1;
        let pit2 = self.positions.pit2;

        loop {
            let positions = Pos {
                turn: turn,
                player: player,
                pit1: pit1,
                pit2: pit2,
            };
            let action = self.action_provider.next(&positions);
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
            turn += 1;
        }
    }
}

impl<T: ActionProvider> PartialEq for Game<T> {
    fn eq(&self, other: &Game<T>) -> bool {
        self.positions == other.positions
    }
}

impl<T: ActionProvider> fmt::Debug for Game<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let turn = self.positions.turn;
        let player = self.positions.player;
        let pit1 = self.positions.pit1;
        let pit2 = self.positions.pit2;

        write!(
            f,
            "Pos {{ turn: {} player: {} pit1: {} pit2: {} }}",
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

/// Position of game entities
#[derive(Clone, PartialEq, Debug)]
pub struct Pos {
    // The game turn starting from 0. Each player turn increments this by one.
    turn: usize,
    pub player: RoomNum,
    pub pit1: RoomNum,
    pub pit2: RoomNum,
}

impl Pos {
    pub fn new(player: RoomNum, pit1: RoomNum, pit2: RoomNum) -> Self {
        Pos {
            turn: 0,
            player: player,
            pit1: pit1,
            pit2: pit2,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Move(RoomNum),
    Quit,
}

pub trait ActionProvider {
    fn next(&mut self, positions: &Pos) -> Action;
}

pub struct PlayerActionProvider;

impl ActionProvider for PlayerActionProvider {
    fn next(&mut self, positions: &Pos) -> Action {
        let room_num = positions.player;
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

struct SuperBat<T: ElseWhereVilleProvider> {
    room: RoomNum,
    ville_provider: T,
}

impl<T: ElseWhereVilleProvider> SuperBat<T> {
    fn snatch(&self, player: Player) -> Player {
        Player::new(self.ville_provider.get_ville())
    }
}

trait ElseWhereVilleProvider {
    fn get_ville(&self) -> RoomNum;
}

struct RandVille;

impl ElseWhereVilleProvider for RandVille {
    fn get_ville(&self) -> RoomNum {
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

fn gen_unique_rooms() -> (RoomNum, RoomNum, RoomNum) {
    let mut taken_rooms = HashSet::new();

    let player_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(player_room);
    let pit1_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(pit1_room);
    let pit2_room = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(pit2_room);

    (player_room, pit1_room, pit2_room)
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
    struct ActionProviderDummy;

    impl ActionProvider for ActionProviderDummy {
        fn next(&mut self, _: &Pos) -> Action {
            Action::Quit
        }
    }

    struct ActionProviderSpy {
        actions: Vec<Action>,
        expected_positions: Vec<Pos>,
    }

    impl ActionProviderSpy {
        fn new(actions: Vec<Action>, exptected_positions: Vec<Pos>) -> Self {
            ActionProviderSpy {
                actions: actions,
                expected_positions: exptected_positions,
            }
        }
    }

    struct MockVille {
        ville: RoomNum,
    }

    impl ElseWhereVilleProvider for MockVille {
        fn get_ville(&self) -> RoomNum {
            self.ville
        }
    }

    impl ActionProvider for ActionProviderSpy {
        fn next(&mut self, actual_positions: &Pos) -> Action {
            let turn = actual_positions.turn;
            if turn >= self.expected_positions.len() {
                panic!("unexpected number of turns taken! turn: {}", turn);
            }

            let expected_positions = &self.expected_positions[turn];

            assert_eq!(*expected_positions, *actual_positions);

            match self.actions.pop() {
                Some(action) => action,
                None => panic!("too many pops"),
            }
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
        let expected_positions = create_expected_game_positions(vec![1, 2, 3, 12], 19, 20);
        let initial_pos = expected_positions[0].clone();

        let provider = ActionProviderSpy::new(actions, expected_positions);

        let mut game = Game {
            positions: initial_pos,
            action_provider: provider,
        };

        assert_eq!(RunResult::UserQuit, game.run())
    }

    #[test]
    fn can_move_and_fall_in_pit() {
        let actions = vec![Action::Move(2)]; // move into bottomless pit
        let initial_pos = Pos::new(1, 2, 3);
        let provider = ActionProviderSpy::new(actions, vec![initial_pos.clone()]);
        let mut game = Game {
            positions: initial_pos,
            action_provider: provider,
        };

        assert_eq!(RunResult::DeathByBottomlessPit, game.run())
    }

    #[test]
    fn super_bat_can_snatch() {
        // snatch and send player to room 20
        let expected_room = 20;
        let ville_provider = MockVille { ville: expected_room };
        let bat = SuperBat { room: expected_room, ville_provider: ville_provider };

        let player = Player::new(1);
        let player = bat.snatch(player);

        assert_eq!(expected_room, player.room);
    }

    fn create_expected_game_positions(
        rooms: Vec<RoomNum>,
        pit1: RoomNum,
        pit2: RoomNum,
    ) -> Vec<Pos> {
        let mut result = Vec::new();
        for (i, room) in rooms.iter().enumerate() {
            result.push(Pos {
                turn: i,
                player: *room,
                pit1: pit1,
                pit2: pit2,
            });
        }
        result
    }
}
