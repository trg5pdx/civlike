//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use specs::prelude::*;
use specs_derive::*;

pub enum MessageType {
    Build,
    Claim,
    Move,
    Error,
    Other,
}

#[derive(Component)]
pub struct GameLog {
    pub entries: Vec<String>,
    pub message_type: Vec<MessageType>,
}
