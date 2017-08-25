#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#[cfg(test)]
extern crate quickcheck;

mod game;

use std::io;
use game::*;

fn main() {
    let mut room_num = 1;
    loop {
        println!("You are in room {}", room_num);
        println!("Shoot, Move, or Quit (S, M, Q)");

        match read_sanitized_line().as_ref() {
            "M" => {
                room_num = move_player(room_num);
            }
            "Q" => break,
            _ => continue,
        }
    }
}

fn move_player(room_num: RoomNum) -> RoomNum {
    let adj_rooms = MAP[room_num - 1];
    let adj1 = adj_rooms[0];
    let adj2 = adj_rooms[1];
    let adj3 = adj_rooms[2];
    println!("You are in room {}", room_num);
    println!("Tunnels leads to {} {} {}", adj1, adj2, adj3);

    loop {
        let input = read_sanitized_line();
        match input.parse::<RoomNum>() {
            Ok(n) => if n == adj1 || n == adj2 || n == adj3 {
                return n;
            } else {
                println!("Not Possible - Where to?");
            },
            Err(_) => println!("Not Possible - Where to?"),
        }
    }
}

// Reads a line from stdin, trims it, and returns it as upper case.
fn read_sanitized_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_uppercase()
}
