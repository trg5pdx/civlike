//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::{
    handle_move_result, xy_idx, FailedMoveReason, Map, Player, Position, RunState, State, TileType,
    Viewshed, World,
};
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

/// Attempts to move the cursor in the world, checks if the place the cursor will be at is the border or not
fn try_move_player(
    delta_x: i32,
    delta_y: i32,
    ecs: &mut World,
) -> Result<(i32, i32), FailedMoveReason> {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    if let Some((_player, pos, viewshed)) =
        (&mut players, &mut positions, &mut viewsheds).join().next()
    {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Ice && map.revealed_tiles[destination_idx] {
            let mut ppos = ecs.write_resource::<Point>();
            pos.x = min(map.width, max(0, pos.x + delta_x));
            pos.y = min(map.height, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;
            viewshed.dirty = true;

            return Ok((pos.x, pos.y));
        } else {
            return Err(FailedMoveReason::TileBlocked);
        }
    }
    Err(FailedMoveReason::UnableToGrabEntity)
}

/// Teleports the player to the location of the stopped unit so the player can look around the area
pub fn teleport_player(unit_pos: Position, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let mut ppos = ecs.write_resource::<Point>();
        pos.x = min(map.width, max(0, unit_pos.x));
        pos.y = min(map.height, max(0, unit_pos.y));
        ppos.x = pos.x;
        ppos.y = pos.y;

        viewshed.dirty = true;
    }
}

/// Grabs input for the cursor to let it be moved around or for it to open a menu or close the game
pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::A => handle_move_result(try_move_player(-1, 0, &mut gs.ecs)),
            VirtualKeyCode::D => handle_move_result(try_move_player(1, 0, &mut gs.ecs)),
            VirtualKeyCode::W => handle_move_result(try_move_player(0, -1, &mut gs.ecs)),
            VirtualKeyCode::S => handle_move_result(try_move_player(0, 1, &mut gs.ecs)),
            VirtualKeyCode::I => return RunState::ShowUnits,
            VirtualKeyCode::F => return RunState::ShowForts,
            VirtualKeyCode::Escape => std::process::exit(0),
            _ => return RunState::Paused,
        },
    }
    RunState::MoveCursor
}
