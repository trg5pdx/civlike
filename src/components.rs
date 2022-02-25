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

#[derive(Component, Debug)]
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

#[derive(Component, Debug)]
pub struct Player {}

pub enum UnitState {
    Selected,
    Idle,
}

#[derive(Component)]
pub struct Unit {
    pub health: u8,
    pub strength: u8,
    pub state: UnitState,
}

/// For denoting what player owns a specific unit
#[derive(Component, Debug, Clone)]
pub struct OwnedBy {
    pub owner: Entity
}

/// For keeping track of all the units owned by a player
#[derive(Component, Debug, Clone)]
pub struct UnitControl {
    pub owned_by: Entity,
    pub unit: Entity,
}
