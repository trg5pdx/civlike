//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

// use std::cmp::{max, min};
use bracket_lib::prelude::*;
// use specs::prelude::*;
use crate::heightmap::generate_heightmap;
// use crate::{Viewshed, Player, World};

pub const MAPWIDTH: usize = 120;
pub const MAPHEIGHT: usize = 120;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

pub const VIEW_WIDTH: usize = 60;
pub const VIEW_HEIGHT: usize = 50;
pub const VIEW_COUNT: usize = VIEW_WIDTH * VIEW_HEIGHT;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Mountain,
    Forest,
    Grasslands,
    Coast,
    Water,
    Ice,
}

// Kept this outside of the Map impl so heightmap can still make use of it
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAPWIDTH) + x as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    // Idea for code generation came from: https://gillesleblanc.wordpress.com/2012/10/16/creating-a-random-2d-game-world-map/
    pub fn new_map() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Water; MAPCOUNT], 
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
        };
        
        let perlin = generate_heightmap();

        for y in 0..map.height {
            for x in 0..map.width {
                    let idx = xy_idx(x, y); 
                    
                    if perlin[idx] > (1.0/6.0) {
                        map.tiles[idx] = TileType::Mountain;
                    }
                    else if perlin[idx] > 0.0 {
                        map.tiles[idx] = TileType::Forest;
                    }
                    else if perlin[idx] > -(1.0/10.0) {
                        map.tiles[idx] = TileType::Grasslands;
                    }
                    else if perlin[idx] > -(1.0/8.0) {
                        map.tiles[idx] = TileType::Coast;
                    }
                    else {
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
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx as usize] == TileType::Ice
    }
}
