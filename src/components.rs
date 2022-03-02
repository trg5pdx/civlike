//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}
// Came from roguelike tutorial chapter 7
#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerOrder {
    NoPlayer,
    PlayerOne,
    PlayerTwo,
}

#[derive(Component, Debug)]
pub struct Player {
    pub order: PlayerOrder,
}

/// Used for marking a unit as being movable
#[derive(Component, Debug)]
pub struct Moving;

#[derive(Component)]
pub struct Unit {
    pub owner: PlayerOrder,
    pub health: u8,
    pub strength: u8,
}
