//! Made by: Thomas Gardner, 2022
//!
//! Most of the code for this comes froms the rust roguelike tutorial linked below, and it was pulled from src/player.rs
//! and tweaked to work for units rather than for moving player units
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::spawner::*;
use crate::{
    handle_move_result, teleport_player, xy_idx, FailedMoveReason, Map, Moving, Player,
    PlayerOrder, Position, RunState, State, Unit, Viewshed, World,
};
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

/// Attempts to move a unit, checking if the tile the unit will end up on is blocked or not
fn try_move_unit(
    delta_x: i32,
    delta_y: i32,
    ecs: &mut World,
) -> Result<(i32, i32), FailedMoveReason> {
    let mut positions = ecs.write_storage::<Position>();
    let mut units = ecs.write_storage::<Unit>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let moving_marker = ecs.read_storage::<Moving>();
    let map = ecs.fetch::<Map>();

    if let Some((_unit, pos, viewshed, _moving)) =
        (&mut units, &mut positions, &mut viewsheds, &moving_marker)
            .join()
            .next()
    {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if !map.blocked[destination_idx] {
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

/// Used for removing the moving marker from a unit struct so it won't move the next time a unit gets moved
/// Returns a position if it was successful to teleport the player to the location of the recently unmarked unit
fn unmark_moving_unit(ecs: &mut World) -> Option<Position> {
    let entities = ecs.entities();
    let units = ecs.read_storage::<Unit>();
    let positions = ecs.read_storage::<Position>();
    let mut moving_marker = ecs.write_storage::<Moving>();

    let mut curr_pos = None;

    for (entity, _unit, pos) in (&entities, &units, &positions).join() {
        match moving_marker.remove(entity) {
            None => {}
            Some(_move) => {
                curr_pos = Some(Position { x: pos.x, y: pos.y });
            }
        }
    }
    curr_pos
}

/// Lets the player move a unit around, claim a tile, build a fort, or exit back to cursor mode
pub fn unit_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => return RunState::MoveUnit,
        Some(key) => match key {
            VirtualKeyCode::A => handle_move_result(try_move_unit(-1, 0, &mut gs.ecs)),
            VirtualKeyCode::D => handle_move_result(try_move_unit(1, 0, &mut gs.ecs)),
            VirtualKeyCode::W => handle_move_result(try_move_unit(0, -1, &mut gs.ecs)),
            VirtualKeyCode::S => handle_move_result(try_move_unit(0, 1, &mut gs.ecs)),
            VirtualKeyCode::G => claim_tile(&mut gs.ecs),
            VirtualKeyCode::B => {
                return build_fort(&mut gs.ecs);
            }
            VirtualKeyCode::I => {
                // Maybe come back to this; I don't think it could be done but probably better to err on the side of caution
                match unmark_moving_unit(&mut gs.ecs) {
                    None => {
                        panic!("ERROR: Failed to unmark moving unit")
                    }
                    Some(pos) => {
                        teleport_player(pos, &mut gs.ecs);
                        return RunState::Paused;
                    }
                }
            }
            _ => {}
        },
    }
    RunState::MoveUnit
}

/// Grabs the currently moving unit and claims the tile it's currently on if it isn't claimed already
fn claim_tile(ecs: &mut World) {
    let units = ecs.read_storage::<Unit>();
    let positions = ecs.read_storage::<Position>();
    let moving = ecs.read_storage::<Moving>();
    let players = ecs.read_storage::<Player>();
    let entities = ecs.entities();
    let mut map = ecs.fetch_mut::<Map>();

    for (_unit, pos, _move) in (&units, &positions, &moving).join() {
        for (_player_entity, player) in (&entities, &players).join() {
            let idx = xy_idx(pos.x, pos.y);
            if map.claimed_tiles[idx] == PlayerOrder::NoPlayer {
                map.claimed_tiles[idx] = player.order;
            }
        }
    }
}

/// Gets the curret location of a unit and if it's claimed by the current player, builds a fort there
fn build_fort(ecs: &mut World) -> RunState {
    let mut player_order: Option<PlayerOrder> = None;
    let mut new_fort_pos: Option<(i32, i32)> = None;

    /*
        Scoping this to prevent errors from the borrow checker since I'm moving ecs into unit,
        and inserting the unit into the world. Got the idea from the rust roguelike tutorial

        Section: User Interface; Notifying of Deaths
        Link: https://bfnightly.bracketproductions.com/rustbook/chapter_8.html
    */
    {
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        let positions = ecs.read_storage::<Position>();
        let units = ecs.read_storage::<Unit>();
        let moving_units = ecs.read_storage::<Moving>();
        let mut map = ecs.fetch_mut::<Map>();

        for (player, _entity) in (&players, &entities).join() {
            player_order = Some(player.order);
        }

        if let Some(ref owner) = player_order {
            for (_unit, pos, _moving) in (&units, &positions, &moving_units).join() {
                let idx = xy_idx(pos.x, pos.y);
                if map.claimed_tiles[idx] == *owner {
                    new_fort_pos = Some((pos.x, pos.y));
                }

                for x in pos.x - 1..=pos.x + 1 {
                    for y in pos.y - 1..=pos.y + 1 {
                        let idx = xy_idx(x, y);
                        map.claimed_tiles[idx] = *owner;
                    }
                }
            }
        }
    }

    if let Some(pos) = new_fort_pos {
        if let Some(player) = player_order {
            let mut fort_counter = 0;
            {
                let mut players = ecs.write_storage::<Player>();
                let entities = ecs.entities();

                for (player, _entity) in (&mut players, &entities).join() {
                    player.fort_count += 1;
                    fort_counter = player.fort_count;
                }
            }
            let name = format!("Fort{}", fort_counter);
            let fort = fort(ecs, pos, name, player);
            ecs.insert(fort);
        }
    }
    RunState::MoveUnit
}
