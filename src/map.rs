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

pub const MAPWIDTH: usize = 60;
pub const MAPHEIGHT: usize = 50;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

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

    pub fn draw_map(&self, ctx: &mut BTerm) {
        let bgd = RGB::from_f32(0.0, 0.0, 0.0);
        let mut y = 0;
        let mut x = 0;
    
        for (idx, tile) in self.tiles.iter().enumerate() {    
            // Render a tile depending upon the tile type
            if self.revealed_tiles[idx] {
                let glyph;
                let mut fg;

                match tile {
                    TileType::Mountain => {
                        fg = RGB::named(GREY);
                        glyph = to_cp437('A');
                        
                    }
                    TileType::Forest => {
                        fg = RGB::named(DARKGREEN);
                        glyph = to_cp437('t');
                    }
                    TileType::Grasslands => {
                        fg = RGB::named(GREEN);
                        glyph = to_cp437('w');
                    }
                    TileType::Coast => {
                        fg = RGB::named(YELLOW);
                        glyph = to_cp437('s');
                    }
                    TileType::Water => {
                        fg = RGB::named(BLUE);
                        glyph = to_cp437('~');
                    }
                    TileType::Ice => {
                        fg =  RGB::named(WHITE);
                        glyph = to_cp437('#');
                    }
                }
                if !self.visible_tiles[idx] { fg = fg.to_greyscale() }
                ctx.set(x, y, fg, bgd, glyph);
            }

            // move the coordinates
            x += 1;
            if x == self.width {
                x = 0;
                y += 1;
            }
        }
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
