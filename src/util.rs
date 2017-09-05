use std::collections::HashSet;
use rand::{thread_rng, Rng};
use std::io;
use std::io::Write;
use map::{is_adj, RoomNum, MAP};

pub fn get_adj_room_to(room: RoomNum) -> RoomNum {
    print("Where to? ");

    loop {
        let input = read_sanitized_line();

        match input.parse::<RoomNum>() {
            Ok(next) if is_adj(room, next) => return next,
            _ => print("Not Possible - Where to? ")
        }
    }
}

// Reads a line from stdin, trims it, and returns it as upper case.
pub fn read_sanitized_line() -> String {
    read_trimed_line().to_uppercase()
}

pub fn read_trimed_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}

// Print without new line and flush to force it to show up.
pub fn print(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

pub fn gen_unique_rooms() -> (RoomNum, RoomNum, RoomNum, RoomNum, RoomNum) {
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

pub fn gen_unique_rand_room(taken_rooms: &HashSet<RoomNum>) -> RoomNum {
    let mut rng = thread_rng();

    loop {
        let room: RoomNum = rng.gen_range(1, MAP.len() + 1);

        if !taken_rooms.contains(&room) {
            return room;
        }
    }
}
