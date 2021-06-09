//! Example for Dungeon Generator - Draws a dungeon to the terminal
//!
//! Christian Schmid - June 2021
//! CS510 - Rust Programming

use dungeon_generator::inst::Dungeon;

fn main() {
    let dun = Dungeon::new(20, 20);
    println!("{:?}", dun);
}
