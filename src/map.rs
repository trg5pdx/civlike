//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

// use std::cmp::{max, min};
use bracket_lib::prelude::*;

pub const MAPWIDTH: usize = 60;
pub const MAPHEIGHT: usize = 50;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Grasslands,
    Forest,
    Water,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAPWIDTH) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Grasslands; MAPCOUNT];
    
    let map_height: i32 = MAPHEIGHT as i32 - 1;
    let map_width: i32 = MAPWIDTH as i32 - 1;

    for x in 0..=map_width {
        map[xy_idx(x, 0)] = TileType::Water;
        map[xy_idx(x, map_height)] = TileType::Water;
    }
    for y in 0..=map_height {
        map[xy_idx(0, y)] = TileType::Water;
        map[xy_idx(map_width, y)] = TileType::Water; 
    }

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, map_width);
        let y = rng.roll_dice(1, map_height);

        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Water;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    // let map_height: i32 = MAPHEIGHT as i32 - 1;
    let map_width: i32 = MAPWIDTH as i32 - 1;

    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Grasslands => {
                ctx.set(x, y, RGB::named(DARKGREEN), RGB::from_f32(0.0, 0.0, 0.0), to_cp437('t'));
            }
            TileType::Water => {
                ctx.set(x, y, RGB::named(BLUE), RGB::from_f32(0.0, 0.0, 0.0), to_cp437('~'));
            }
            TileType::Forest => {}
        }

        // move the coordinates
        x += 1;
        if x > map_width {
            x = 0;
            y += 1;
        }
    }
}
