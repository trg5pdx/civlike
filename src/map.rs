//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

// use std::cmp::{max, min};
use bracket_lib::prelude::*;
use crate::heightmap::generate_heightmap;

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

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAPWIDTH) + x as usize
}

// Idea for code geenration came from: https://gillesleblanc.wordpress.com/2012/10/16/creating-a-random-2d-game-world-map/
pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Water; MAPCOUNT];
    
    let map_height: i32 = MAPHEIGHT as i32;
    let map_width: i32 = MAPWIDTH as i32;
    
    let perlin = generate_heightmap();

    for y in 0..map_height {
        for x in 0..map_width {
                let idx = xy_idx(x, y); 
                
                if perlin[idx] > (1.0/6.0) {
                    map[idx] = TileType::Mountain;
                }
                else if perlin[idx] > 0.0 {
                    map[idx] = TileType::Forest;
                }
                else if perlin[idx] > -(1.0/10.0) {
                    map[idx] = TileType::Grasslands;
                }
                else if perlin[idx] > -(1.0/8.0) {
                    map[idx] = TileType::Coast;
                }
                else {
                    map[idx] = TileType::Water; 
                }
        }
    }

    // Make the boundaries walls
    for x in 0..map_width {
        map[xy_idx(x, 0)] = TileType::Ice;
        map[xy_idx(x, map_height - 1)] = TileType::Ice;
    }
    for y in 0..map_height {
        map[xy_idx(0, y)] = TileType::Ice;
        map[xy_idx(map_width - 1, y)] = TileType::Ice;
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    // let map_height: i32 = MAPHEIGHT as i32 - 1;
    let map_width: i32 = MAPWIDTH as i32 - 1;
    
    let bgd = RGB::from_f32(0.0, 0.0, 0.0);

    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Mountain => {
                ctx.set(x, y, RGB::named(GREY), bgd, to_cp437('A'));
            }
            TileType::Forest => {
                ctx.set(x, y, RGB::named(DARKGREEN), bgd, to_cp437('t'));
            }
            TileType::Grasslands => {
                ctx.set(x, y, RGB::named(GREEN), bgd, to_cp437('w'));
            }
            TileType::Coast => {
                ctx.set(x, y, RGB::named(YELLOW), bgd, to_cp437('s'));
            }
            TileType::Water => {
                ctx.set(x, y, RGB::named(BLUE), bgd, to_cp437('~'));
            }
            TileType::Ice => {
                ctx.set(x, y, RGB::named(WHITE), bgd, to_cp437('#'));
            }
        }

        // move the coordinates
        x += 1;
        if x > map_width {
            x = 0;
            y += 1;
        }
    }
}
