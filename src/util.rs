use map::{RoomNum, MAP};
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::io;
use std::io::Write;

// Reads a line from stdin, trims it, and returns it as upper case.
pub fn read_sanitized_line() -> String {
    read_line().trim().to_uppercase()
}

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input
}

// Print without new line and flush to force it to show up.
pub fn print(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

pub fn gen_unique_rooms() -> (RoomNum, RoomNum, RoomNum, RoomNum, RoomNum, RoomNum) {
    let mut taken_rooms = HashSet::new();

    let player = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(player);
    let pit1 = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(pit1);
    let pit2 = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(pit2);
    let bat1 = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(bat1);
    let bat2 = gen_unique_rand_room(&taken_rooms);
    taken_rooms.insert(bat2);
    let wumpus = gen_unique_rand_room(&taken_rooms);

    (player, wumpus, pit1, pit2, bat1, bat2)
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
