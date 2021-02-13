#![forbid(unsafe_code)]

//! This crate is not really used right now. The plan is to be able to run automated tests
//! with AIs playing the role of the player to prove that the player will or will not be able to
//! make certain jumps, etc.
//!
//! Being able to guarantee that a player cannot make a certain jump is of course very important in
//! a puzzle game, because these sorts of bugs could make puzzles trivially easy.
//!
//! This crate can also eventually be leveraged to create automated tests that prove that a certain
//! puzzle is solvable.

// TODO: Change this crate to a bin.

#[macro_use]
extern crate log;
