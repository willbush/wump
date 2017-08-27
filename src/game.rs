use std::fmt;
use std::collections::HashSet;

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

pub fn can_move(next: RoomNum, current: RoomNum) -> bool {
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

pub fn adj_rooms_to(room: RoomNum) -> (RoomNum, RoomNum, RoomNum) {
    let adj_rooms = MAP[room - 1];
    (adj_rooms[0], adj_rooms[1], adj_rooms[2])
}

pub type RoomNum = usize;

#[derive(Debug, PartialEq)]
pub enum Action {
    Move(RoomNum),
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum RunResult {
    DeathByBottomlessPit,
    UserQuit,
}

impl fmt::Display for RunResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            RunResult::DeathByBottomlessPit => "YYYIIIIEEEE... fell in a pit!\n\
                                                Ha ha ha - you lose!\n",
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

pub trait ActionProvider {
    fn next(&mut self, positions: &Pos) -> Action;
}


#[derive(PartialEq, Debug)]
pub enum GameErr {
    NonUniqueSpawnLocations,
}

type GameResult = Result<Game, GameErr>;

pub struct Game {
    positions: Pos,
    action_provider: Box<ActionProvider>,
}

impl Game {
    pub fn new(positions: Pos, ap: Box<ActionProvider>) -> GameResult {
        let mut rooms = HashSet::new();
        rooms.insert(positions.player);
        rooms.insert(positions.pit1);
        rooms.insert(positions.pit2);
        let expected_unique_room_count = 3;

        if rooms.len() != expected_unique_room_count {
            Err(GameErr::NonUniqueSpawnLocations)
        } else {
            Ok(Game {
                positions: positions,
                action_provider: ap,
            })
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

impl PartialEq for Game {
    fn eq(&self, other: &Game) -> bool {
        self.positions == other.positions
    }
}

impl fmt::Debug for Game {
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

        let provider = box ActionProviderSpy::new(actions, expected_positions);

        let mut game = Game::new(initial_pos, provider).unwrap();

        assert_eq!(RunResult::UserQuit, game.run())
    }

    #[test]
    fn can_move_and_fall_in_pit() {
        let actions = vec![Action::Move(2)]; // move into bottomless pit
        let initial_pos = Pos::new(1, 2, 3);
        let provider = box ActionProviderSpy::new(actions, vec![initial_pos.clone()]);
        let mut game = Game::new(initial_pos, provider).unwrap();

        assert_eq!(RunResult::DeathByBottomlessPit, game.run())
    }

    #[test]
    fn initial_pos_with_non_unique_spawns_causes_err() {
        assert_pos_has_game_result(Pos::new(1, 2, 2), Err(GameErr::NonUniqueSpawnLocations));
        assert_pos_has_game_result(Pos::new(2, 2, 1), Err(GameErr::NonUniqueSpawnLocations));
        assert_pos_has_game_result(Pos::new(2, 1, 2), Err(GameErr::NonUniqueSpawnLocations));
    }

    fn assert_pos_has_game_result(positions: Pos, result: GameResult) {
        let game = Game::new(positions, box ActionProviderDummy);

        assert_eq!(result, game);
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
