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
use crate::{xy_idx, State, Map , World, Position, Player, TileType};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
		if map.tiles[destination_idx] != TileType::Ice && 
           map.revealed_tiles[destination_idx] {
            let mut ppos = ecs.write_resource::<Point>();
            pos.x = min(map.width, max(0, pos.x + delta_x));
            pos.y = min(map.height, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> Option<u32> {
    // Player movement
    match ctx.key {
        None => { None }, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::A => { try_move_player(-1, 0, &mut gs.ecs); None },
            VirtualKeyCode::D => { try_move_player(1, 0, &mut gs.ecs); None },
            VirtualKeyCode::W => { try_move_player(0, -1, &mut gs.ecs); None },
            VirtualKeyCode::S => { try_move_player(0, 1, &mut gs.ecs); None },
			VirtualKeyCode::C => Some(5_u32),
			VirtualKeyCode::Escape => std::process::exit(0),
            _ => { None }
        },
    }
}
