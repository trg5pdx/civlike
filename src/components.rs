//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project.
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::*;

/// Stores the tiles currently visible, and stores how many tiles out an entity can see
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

/// Stores an x/y position for an entity
#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// For recording how to render an entity and its color and background along with render order
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

/// Used to keep track of which player owns/claims what
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerOrder {
    NoPlayer,
    PlayerOne,
    PlayerTwo,
}

/// Used for the cursor, keeps track of the player order, and how many units and forts that player has
#[derive(Component, Debug)]
pub struct Player {
    pub order: PlayerOrder,
    pub unit_count: u16,
    pub fort_count: u16,
}

/// Stores the health and strength of a unit and keeps track of who owns that unit
#[derive(Component)]
pub struct Unit {
    pub owner: PlayerOrder,
    pub health: u8,
    pub strength: u8,
}

/// Used for marking a unit as being movable
#[derive(Component)]
pub struct Moving;

/// Stores the forts defense and stores who owns that fort
#[derive(Component)]
pub struct Fort {
    pub owner: PlayerOrder,
    pub defense: u8,
}

/// Used for marking which fort is currently selected by the player
#[derive(Component)]
pub struct Selected;
