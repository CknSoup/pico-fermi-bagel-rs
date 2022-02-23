# pico-pico-fermi-rs
A console based version of the game Pico Pico Fermi, or Pico Fermi bagels written in Rust.

## Installation

The most straightforward way to run this game is with Rust directly:

* Clone the project
* In the top level directory run: `cargo run`

## Options

### *ToDo*

## How to play

The goal of the game is to guess a series of numbers ranging from 0-9 correctly.
The length of the series of numbers is by default 4.

By default, you have 8 chances to guess the series of numbers correctly.
After a guess you are told how many numbers are present and in the correct place, and how many numbers are correct but not in the right place.
You are not told which numbers are correct, but you must deduce which ones they are through subsequent guesses.

Example play-through:

```
$ cargo run
4 Numbers : 0-9 Range : 8 Guesses

(1) 8 5 4 2  -==-  F - - - 
(2) 1 3 5 7  -==-  F F - -
(3) 5 7 3 0  -==-  P - - -
(4) 5 1 7 6  -==-  F - - -  
(5) 4 7 1 9  -==-  F F F -
(6) 1 4 3 9  -==-  P P F F
(7) 9 4 3 1  -==-  P P P P 

(A) 9 4 3 1
You Win!
```

The program first starts by stating the game settings.

The guess number is labeled on the left-hand side with parentheses.
The user's guess inputs the guess next to it; these can be immediately adjacent, or delimited by spaces.
After the user submits their guess with the Enter key, their guess's Pico-Fermi result is given.
For guess 1, the game states 1 Fermi, meaning that one of the numbers is correct, but not in the proper place (in this example, 4).
While for guess 3, the game states 1 Pico, meaning that one number is correct and in its proper place (here, 3).
Notice that the Picos and Fermis fill up from left to right; therefore, for guess 3, the game isn't stating that 5 is in its proper place, rather that one of those numbers is in the correct place.

The game will give you the correct result in the end regardless of whether you guess correctly or not.

The guesses can be whitespace delimited or not, but one should use whitespace and be consistent for readability.

## ToDo List:

* (1.0) Implement the program startup, including the random generation of the result match.
* (2.0) Implement user input; implement error checking
* (3.0) Implement guess parsing and checking
* (4.0) Implement Pico/Fermi display (include special unicode characters like green/yellow/red circles) 
* (5.0) Implement custom game settings
