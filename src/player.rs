//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project.
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::{
    error_handling, xy_idx, FailedMoveReason, Map, Player, Position, RunState, State, TileType,
    World,
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
    let map = ecs.fetch::<Map>();

    if let Some((_player, pos)) = (&mut players, &mut positions).join().next() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Ice && map.revealed_tiles[destination_idx] {
            let mut ppos = ecs.write_resource::<Point>();
            pos.x = min(map.width, max(0, pos.x + delta_x));
            pos.y = min(map.height, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;

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
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let mut ppos = ecs.write_resource::<Point>();
        pos.x = min(map.width, max(0, unit_pos.x));
        pos.y = min(map.height, max(0, unit_pos.y));
        ppos.x = pos.x;
        ppos.y = pos.y;
    }
}

/// Grabs input for the cursor to let it be moved around or for it to open a menu or close the game
pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => return RunState::MoveCursor,
        Some(key) => match key {
            VirtualKeyCode::A => {
                let res = try_move_player(-1, 0, &mut gs.ecs);
                error_handling::handle_move_result(&mut gs.ecs, res, gs.verbose);
            }
            VirtualKeyCode::D => {
                let res = try_move_player(1, 0, &mut gs.ecs);
                error_handling::handle_move_result(&mut gs.ecs, res, gs.verbose);
            }
            VirtualKeyCode::W => {
                let res = try_move_player(0, -1, &mut gs.ecs);
                error_handling::handle_move_result(&mut gs.ecs, res, gs.verbose);
            }
            VirtualKeyCode::S => {
                let res = try_move_player(0, 1, &mut gs.ecs);
                error_handling::handle_move_result(&mut gs.ecs, res, gs.verbose);
            }
            VirtualKeyCode::I => return RunState::ShowUnits,
            VirtualKeyCode::F => return RunState::ShowForts,
            VirtualKeyCode::Escape => std::process::exit(0),
            _ => return RunState::MoveCursor,
        },
    }
    RunState::MoveCursor
}
