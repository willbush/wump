<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [wump - The game of Hunt the Wumpus](#wump---the-game-of-hunt-the-wumpus)
    - [Introduction](#introduction)
    - [Building and Running](#building-and-running)
    - [Why wump in Rust?](#why-wump-in-rust)
    - [How to Play](#how-to-play)
        - [Cheat Mode](#cheat-mode)
    - [Variations from the Original](#variations-from-the-original)

<!-- markdown-toc end -->

# wump - The game of Hunt the Wumpus

## Introduction

This is an implementation of Gregory Yob's [Hunt the
Wumpus](https://en.wikipedia.org/wiki/Hunt_the_Wumpus) text based 1972 game
using Rust.

## Building and Running

1. clone the repo
2. install [cargo](https://crates.io/) nightly
3. cd into the directory of the repo and `cargo run --release`

## Why wump in Rust?

I simply wanted to learn Rust. I especially wanted to learn how to write
testable code, test, and mock things out in Rust. I've implemented this game 3
times (text, 2D, 3D) in a intro to video game programming class I took in
college for fun, and I wanted to see how Rust "feels like" to program in
compared to C#.

## How to Play

The game has 20 rooms which form a dodecahedron. Rooms are numbered from 1 to 20
and each room is connected to 3 other rooms.

The game has 3 types hazards which are assigned to random, yet non-overlapping
room numbers:

- 2 bottomless pits
  - These are static hazards that do not move.
  - The player falls to their death when entered.
  - In the original game the Wumpus can go in these rooms because it has
    "suckers" on its feet. However, I the way I implemented it has the Wumpus
    avoiding going into rooms with Bottomless pits.

- 2 super bats
  - These are static hazards that do not move.
  - They snatch the player to another random room when entered.
    - keeps snatching the player until snatched to a non-bat room.
  - The Wumpus can go into this room, but won't be picked up because "it's too
    heavy!"

- 1 Wumpus
  - The Wumups is the monster you have to slay to win the game.
  - The Wumpus is immobile when the game starts since it is asleep.
  - Shooting an arrow on the map or entering the room of the wumpus will wake up
    the wumpus.
  - When awake it has a 75% chance of moving per turn.
  - The wumpus also has a 75% chance of moving out of the room when you "bump
    it" or first enter its room when it is asleep. Think of this as a fight or
    flight response. If the Wumpus stays to fight, then you lose.

You as the player:

- navigate the map in a sort of fog of war. You can only see the 3 adjacent
  rooms that you can move to, but you cannot see what they contain
- gain information about the map's layout via clues from adjacent rooms. These
  clues come in the form of warnings. When a hazard is adjacent to your room it
  will print its warning.
- have a bow with 5 magical crooked arrows that can navigate the crooked halls
  of the cave.
- If you run out of arrows, then you lose.

### Cheat Mode

I have enabled a cheat mode that displays where everything on the map is and
where the Wumpus moves everything it moves. Just pass the argument "cheat" to
the game.

For example:

```bash
cargo run cheat --release
```

## Variations from the Original

There are many variations on this game, and this one is no different. This
implementation was based on requirements as I understood them for the game and
my own opinions how things ought to work.

If you're curious about what might be different from other implementations, then
here are some differences I've noticed.

- The Wumpus avoids bottomless pits instead of moving through them.

- When the Wumpus is awakened by being bumped by the player its 75% of moving
  still applies, and the player will live or die depending on if the Wumpus
  moves or not.

- Super bats can snatch the player into its own room or the other bat room,
  which causes it to get snatched again. Snatching loops until the player is
  snatched into a non-super bat room. For each time the player is snatched it is
  treated as a normal game turn, printing out the hazard warnings and possibly
  having the Wumpus move (if awake).

- The player is asked for a space separated list of rooms given all at once
  instead of incrementally. Rooms are still validated that they don't contain a
  "too crooked" A-B-A path. The edge case of A-B-A path of player to adjacent
  room and back is not ignored.

- When the arrow traverses the given path and encounters a room that is not
  adjacent, the arrow then traverses randomly from then on. Some implementations
  will have the arrow go back to following the given path despite one of the
  rooms in the middle being disjoint.

- As the arrow randomly traverses it still avoids the too crooked A-B-A path.

- Its not possible for the arrow to kill the wumpus and the player. Arrows stop
  traversing if it hits the player or the Wumpus.

- When the game starts all game entities are placed in non-overlapping random
  rooms on the map.
  
- The game allows replaying with the same initial random game state (or setup).

- Has a [cheat mode](#cheat-mode).
