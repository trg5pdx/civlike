//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::heightmap::generate_heightmap;
use crate::PlayerOrder;
use crate::PlayerOrder::*;
use bracket_lib::prelude::*;
use specs::Entity;

pub const MAPWIDTH: usize = 400;
pub const MAPHEIGHT: usize = 300;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

//! The work I did for decoupling the screen from the map came from here:
//! https://bfnightly.bracketproductions.com/rustbook/chapter_41.html
pub const VIEW_WIDTH: usize = 60;
pub const VIEW_HEIGHT: usize = 40;
pub const VIEW_COUNT: usize = VIEW_WIDTH * VIEW_HEIGHT;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Mountain,
    Forest,
    Grasslands,
    Coast,
    Water,
    Ice,
}

/// Uses the x/y coordinates to get the location of a tile in a 1 dimensional array; used for the tile map and the heightmap
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAPWIDTH) + x as usize
}

/// Contains all tiles of the map and includes tiles that are revealed, visible, blocked, or claimed by a player
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
    pub claimed_tiles: Vec<PlayerOrder>,
}

impl Map {
    pub fn new_map() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Water; MAPCOUNT],
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            tile_content: vec![Vec::new(); MAPCOUNT],
            claimed_tiles: vec![NoPlayer; MAPCOUNT],
        };

        // Idea for map generation came from: https://gillesleblanc.wordpress.com/2012/10/16/creating-a-random-2d-game-world-map/
        let perlin = generate_heightmap();

        for y in 0..map.height {
            for x in 0..map.width {
                let idx = xy_idx(x, y);

                if perlin[idx] > (1.0 / 6.0) {
                    map.tiles[idx] = TileType::Mountain;
                } else if perlin[idx] > 0.0 {
                    map.tiles[idx] = TileType::Forest;
                } else if perlin[idx] > -(1.0 / 10.0) {
                    map.tiles[idx] = TileType::Grasslands;
                } else if perlin[idx] > -(1.0 / 8.0) {
                    map.tiles[idx] = TileType::Coast;
                } else {
                    map.tiles[idx] = TileType::Water;
                }
            }
        }
        // Make the boundaries walls
        for x in 0..map.width {
            map.tiles[xy_idx(x, 0)] = TileType::Ice;
            map.tiles[xy_idx(x, map.height - 1)] = TileType::Ice;
        }
        for y in 0..map.height {
            map.tiles[xy_idx(0, y)] = TileType::Ice;
            map.tiles[xy_idx(map.width - 1, y)] = TileType::Ice;
        }

        map
    }

    // Both populate_blocked and clear_content_index came from chapter 7 of the roguelike tutorial
    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            if *tile == TileType::Ice || *tile == TileType::Mountain || *tile == TileType::Water {
                self.blocked[i] = true;
            } 
        }
    }
    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
    /// Clears the list of currently blocked tiles to refresh what tiles are currently blocked
    pub fn clear_blocked(&mut self) {
        for tile in self.blocked.iter_mut() {
            *tile = false;
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Ice
    }
}
