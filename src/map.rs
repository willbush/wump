use rand::{thread_rng, Rng};

/// room number as usize so it can index into the map.
pub type RoomNum = usize;

pub const NUM_OF_ROOMS: RoomNum = 20;

/// The game map in Hunt the Wumpus is laid out as a dodecahedron. The vertices
/// of the dodecahedron are considered rooms, and each room has 3 adjacent rooms.
/// A room is adjacent if it has a line segment directly from one vertex to
/// another. Here we have a 2D array where the first dimension represents the 20
/// rooms (index + 1 == room number). the second dimension is an array of the
/// adjacent rooms. I just hard coded some valid room values here for ease, but
/// there is a formula that could be used to derive instead.
pub static MAP: [[RoomNum; 3]; NUM_OF_ROOMS] = [
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

/// returns true if the two given rooms are adjacent.
pub fn is_adj(next: RoomNum, current: RoomNum) -> bool {
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

/// Get a random room on the map.
pub fn rand_room() -> RoomNum {
    thread_rng().gen_range(1, MAP.len() + 1)
}

/// Get a tuple of adjacent rooms.
pub fn adj_rooms_to(room: RoomNum) -> (RoomNum, RoomNum, RoomNum) {
    let adj_rooms = MAP[room - 1];
    (adj_rooms[0], adj_rooms[1], adj_rooms[2])
}

/// Get a random room adjacent to the given room.
pub fn rand_adj_room_to(room: RoomNum) -> RoomNum {
    let adj_rooms = MAP[room - 1];
    let i = thread_rng().gen_range(0, adj_rooms.len());
    adj_rooms[i]
}

/// Generate a valid arrow path of given length.
pub fn gen_rand_valid_path_of_len(n: usize) -> Vec<RoomNum> {
    gen_rand_valid_path_from(n, rand_room())
}

pub fn gen_rand_valid_path_from(len: usize, starting: RoomNum) -> Vec<RoomNum> {
    let mut valid_path = Vec::with_capacity(len);

    for i in 0..len {
        if i == 0 {
            valid_path.push(starting);
        } else if i == 1 {
            let prev = valid_path[i - 1];
            valid_path.push(rand_adj_room_to(prev));
        } else {
            let prev = valid_path[i - 1];
            let before_prev = valid_path[i - 2];
            valid_path.push(rand_valid_adj_room_to(prev, before_prev));
        }
    }
    valid_path
}

/// Gets a random room adjacent to the given room, but not equal to the previous
/// room. Useful for avoiding "too crooked" paths.
pub fn rand_valid_adj_room_to(room: RoomNum, previous_room: RoomNum) -> RoomNum {
    loop {
        let r = rand_adj_room_to(room);
        if r != previous_room {
            return r;
        }
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    /// One property that exists for the map is if current room is in bounds of
    /// the map and strictly less than the map length, then we should always be
    /// able to move to the room (current + 1).
    #[quickcheck]
    fn can_move_to_next_room_num_property(current: RoomNum) -> bool {
        let is_adj = is_adj(current, current + 1);

        if current > 0 && current < MAP.len() {
            is_adj
        } else {
            !is_adj
        }
    }

    #[test]
    fn adj_rooms_to_test() {
        let expected = (13, 16, 19);
        let actual = adj_rooms_to(20);
        assert_eq!(expected, actual);
    }
}
