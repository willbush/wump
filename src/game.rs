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

#[derive(Clone, PartialEq, Debug)]
pub struct GameState {
    // The game turn starting from 0. Each player turn increments this by one.
    turn: usize,
    pub player_room: RoomNum,
    pub pit1_room: RoomNum,
    pub pit2_room: RoomNum,
}

impl GameState {
    pub fn new(player_room: RoomNum, pit1: RoomNum, pit2: RoomNum) -> Self {
        GameState {
            turn: 0,
            player_room: player_room,
            pit1_room: pit1,
            pit2_room: pit2,
        }
    }
}

pub trait ActionProvider {
    fn next(&mut self, game_state: &GameState) -> Action;
}


#[derive(PartialEq, Debug)]
pub enum GameErr {
    NonUniqueSpawnLocations,
}

type GameResult = Result<Game, GameErr>;

pub struct Game {
    state: GameState,
    action_provider: Box<ActionProvider>,
}

impl Game {
    pub fn new(initial_state: GameState, ap: Box<ActionProvider>) -> GameResult {
        let mut rooms = HashSet::new();
        rooms.insert(initial_state.player_room);
        rooms.insert(initial_state.pit1_room);
        rooms.insert(initial_state.pit2_room);
        let expected_unique_room_count = 3;

        if rooms.len() != expected_unique_room_count {
            Err(GameErr::NonUniqueSpawnLocations)
        } else {
            Ok(Game {
                state: initial_state,
                action_provider: ap,
            })
        }
    }

    pub fn run(&mut self) {
        let mut room_num = self.state.player_room;
        let mut turn = self.state.turn;
        let pit1_room = self.state.pit1_room;
        let pit2_room = self.state.pit2_room;

        loop {
            let game_state = GameState {
                turn: turn,
                player_room: room_num,
                pit1_room: pit1_room,
                pit2_room: pit2_room,
            };
            let action = self.action_provider.next(&game_state);
            match action {
                Action::Move(next_room) if can_move(room_num, next_room) => room_num = next_room,
                Action::Quit => break,
                _ => panic!("illegal action state"),
            }
            turn += 1;
        }
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Game) -> bool {
        self.state == other.state
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let turn = self.state.turn;
        let player = self.state.player_room;
        let pit1 = self.state.pit1_room;
        let pit2 = self.state.pit2_room;

        write!(
            f,
            "GameState {{ turn: {} player: {} pit1: {} pit2: {} }}",
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
        fn next(&mut self, _: &GameState) -> Action {
            Action::Quit
        }
    }

    struct ActionProviderSpy {
        actions: Vec<Action>,
        expected_states: Vec<GameState>,
    }

    impl ActionProviderSpy {
        fn new(actions: Vec<Action>, exptected_states: Vec<GameState>) -> Self {
            ActionProviderSpy {
                actions: actions,
                expected_states: exptected_states,
            }
        }
    }

    impl ActionProvider for ActionProviderSpy {
        fn next(&mut self, actual_state: &GameState) -> Action {
            let expected_state = &self.expected_states[actual_state.turn];

            assert_eq!(*expected_state, *actual_state);

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
        let expected_states = create_expected_game_states(vec![1, 2, 3, 12], 19, 20);
        let initial_state = expected_states[0].clone();

        let provider = Box::new(ActionProviderSpy::new(actions, expected_states));

        match Game::new(initial_state, provider) {
            Ok(mut game) => game.run(),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn initial_state_with_non_unique_spawns_causes_err() {
        assert_state_has_game_result(GameState::new(1, 2, 2), Err(GameErr::NonUniqueSpawnLocations));
        assert_state_has_game_result(GameState::new(2, 2, 1), Err(GameErr::NonUniqueSpawnLocations));
        assert_state_has_game_result(GameState::new(2, 1, 2), Err(GameErr::NonUniqueSpawnLocations));
    }

    fn assert_state_has_game_result(initial_state: GameState, result: GameResult) {
        let game = Game::new(initial_state, Box::new(ActionProviderDummy));

        assert_eq!(result, game);
    }

    fn create_expected_game_states(
        rooms: Vec<RoomNum>,
        pit1: RoomNum,
        pit2: RoomNum,
    ) -> Vec<GameState> {
        let mut result = Vec::new();
        for (i, room) in rooms.iter().enumerate() {
            result.push(GameState {
                turn: i,
                player_room: *room,
                pit1_room: pit1,
                pit2_room: pit2,
            });
        }
        result
    }
}
