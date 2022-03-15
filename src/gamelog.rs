//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project. This code comes from section 2.7: User Interface
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_8.html#adding-a-message-log

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
