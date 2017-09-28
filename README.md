<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [wump - The game of Hunt the Wumpus](#wump---the-game-of-hunt-the-wumpus)
- [Introduction](#introduction)
- [Building and Running](#building-and-running)
- [Why wump in Rust?](#why-wump-in-rust)
- [How to Play](#how-to-play)
    - [Moving](#moving)
    - [Hazards](#hazards)
    - [Warnings](#warnings)
    - [Shooting](#shooting)
    - [Winning the game](#winning-the-game)
    - [Cheat Mode](#cheat-mode)
- [Opinionated How?](#opinionated-how)
    - [Possible Differences](#possible-differences)

<!-- markdown-toc end -->

# wump - The game of Hunt the Wumpus

# Introduction

This is an opinionated implementation of Gregory Yob's [Hunt the
Wumpus](https://en.wikipedia.org/wiki/Hunt_the_Wumpus) text based 1972 game
using Rust.

# Building and Running

1. clone the repo
2. install [cargo](https://crates.io/) nightly
3. cd into the directory of the repo and `cargo run --release`

# Why wump in Rust?

I originally became aware of this game in an intro to video programming class I
took in college for fun. Throughout the course we implemented this game 3 times
in C# (text based, 2D, and 3D). I was interested to do it again in Rust in part
because I wanted get a feel for programming in a non-object oriented language.

One of the things that stood out to me about this game when implementing it in
rust was just how tricky it can be to implement the requirements without bugs. I
was therefore interested in implementing this in a more TDD style as bug free as
possible where before I had only done a lot of manually testing. 

# How to Play

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
    it" or first enter its room when it is asleep. Think of this as a fight of
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

## Moving

For moving, the game displays a list of the room numbers of connected rooms,
then asks for the room number of the destination room. If the entered room is
not connected to the current room, the game displays, "Not possible" and re-asks
for the destination room. On a move, the game moves the player to the new room,
then checks for whether the player has hit a hazard in that room.

## Hazards

A superbat attack causes the player to be randomly placed in a new room. When
this happens, the game displays "Zap--Super Bat snatch! Elsewhereville for you!"
This is the equivalent to a normal move, in that hazard checks are performed
after determining the new room.

A bottomless pit kills the player, and ends the game. When this happens, the
game displays, "YYYIIIIEEEE . . . fell in a pit"

The Wumpus begins the game asleep. If the player shoots an arrow anywhere in the
map, the Wumpus wakes up. For every turn after the Wumpus is awoken (including
the turn that woke him up), the Wumpus has a 75% chance of moving to another
room.

The first time the player enters the room containing the Wumpus, the Wumpus
wakes up, and the game displays, "... Ooops! Bumped a Wumpus." Once you bump the
Wumpus the player will die if the Wumpus decides to not leave the room of the
player with a 25% chance.

## Warnings

- If superbats are in an adjoining room, display the warning message, “Bats
  nearby”.
- If a bottomless pit is in an adjoining room, display the warning message, "I
  feel a draft".
- If the Wumpus is in an adjoining room, display the warning message, "I smell a
  Wumpus". Shooting

## Shooting

If the player chooses to shoot, the game then asks for up to five rooms the
arrow should visit. The game asks the player to enters a space separated list of
rooms, but does not check if each room correctly follows another. The game does,
however, check to ensure the arrow does not go from room A to B and back to A
again. If the player attempts to enter an A-B-A path, the game replies, "Arrows
aren't that crooked" and the player must re-enter the list of rooms. If the
entered list of rooms are not correctly connected from one room to the next
starting with the player's room, then it's path is determined at randomly from
then on (while still preventing a "too crooked" path).

If the arrow enters the room containing the Wumpus, the game displays, "Aha! You
got the Wumpus!" (and the player wins the game). If the arrow enters the room
containing the player, the game displays, "Ouch! Arrow got you!" If the arrow
goes through its entire path and does not hit the Wumpus (or the player),
display "Missed!" The player only has 5 arrows, and loses one each time an arrow
is shot. The game ends when the player runs out of arrows.

## Winning the game

When the player wins the game, display, "Hee hee hee - the Wumpus'll getcha next
time!!"

Once the game is won or lost the player is given the option to re-play the game
using the same randomly chosen locations for the Wumpus, bats, and pits, or to
have new random locations chosen for them.

## Cheat Mode

I have enabled a cheat mode that displays where everything on the map is and
where the Wumpus moves everything it moves. Just pass the argument "cheat" to
the game.

For example:

```bash
cargo run cheat --release
```

# Opinionated How?

There are many variations on this game, and this one is no different. This
implementation was based on requirements as I understood them for the game and
my own opinions how things ought to work. I honestly didn't spend any time
playing the original game or trying to deconstruct exactly how the original did
things. However, now that I am finished and spent some time to look at other
implementations, I'm surprised how simplified many of them are.

In fact, this might be one of the highest lines of code implementations of the
game. Perhaps part of that is me attempting to model the game entities in an
near object oriented way, which might have been a mistake. However, I'd like to
think part of that has to do with me not ignoring edge cases and not having lazy
implementations of different features as I see so many other implementations do.

## Possible Differences

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
