use std::io;

// The game map in Hunt the Wumpus is laid out as a dodecahedron. The vertices
// of the dodecahedron are considered rooms, and each room has 3 adjacent rooms.
// A room is adjacent if it has a line segment directly from one vertex to
// another. Here we have a 2D array where the first dimension represents the 20
// rooms (index + 1 == room number). the second dimension is an array of the
// adjacent rooms. I just hard coded some valid room values here for ease, but
// there is a formula that could be used to derive instead.
static MAP: [[usize; 3]; 20] = [
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

fn move_player(room_num: usize) -> usize {
    let adj_rooms = MAP[room_num - 1];
    let room_a = adj_rooms[0];
    let room_b = adj_rooms[1];
    let room_c = adj_rooms[2];
    println!("You are in room {}", room_num);
    println!("Tunnels leads to {} {} {}", room_a, room_b, room_c);

    loop {
        let input = read_sanitized_line();
        match input.parse::<usize>() {
            Ok(n) => if n == room_a || n == room_b || n == room_c {
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
