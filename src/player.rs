//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use bracket_lib::prelude::*;
use std::cmp::{max, min};
use specs::prelude::*;
use crate::{MAPHEIGHT, MAPWIDTH, xy_idx, State, Map, Viewshed, World, Position, Player, TileType};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();
	
	let map_width: i32 = MAPWIDTH as i32 - 1;
	let map_height: i32 = MAPHEIGHT as i32 - 1;

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if (map.tiles[destination_idx] != TileType::Water) && 
		   (map.tiles[destination_idx] != TileType::Mountain) &&
		   (map.tiles[destination_idx] != TileType::Ice) {
            let mut ppos = ecs.write_resource::<Point>();
            pos.x = min(map_width, max(0, pos.x + delta_x));
            pos.y = min(map_height, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;

			viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {}, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
			VirtualKeyCode::Escape => std::process::exit(0),
            _ => {}
        },
    }
}
