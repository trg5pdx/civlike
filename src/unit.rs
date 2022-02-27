//! Made by: Thomas Gardner, 2022
//!
//! Most of the code for this comes froms the rust roguelike tutorial linked below, and it was pulled from src/player.rs
//! and tweaked to work for units rather than for moving player units
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::{
    teleport_player, xy_idx, Map, Moving, OwnedBy, Position, RunState, State, Unit, UnitControl,
    Viewshed, World,
};
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

// Doing this a dumb way, I copy pasted this from try_move_player, go back and fix this you idiot
pub fn try_move_unit(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut units = ecs.write_storage::<Unit>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let moving_marker = ecs.read_storage::<Moving>();
    let map = ecs.fetch::<Map>();

    for (_unit, pos, viewshed, _moving) in
        (&mut units, &mut positions, &mut viewsheds, &moving_marker).join()
    {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if !map.blocked[destination_idx] {
            let mut ppos = ecs.write_resource::<Point>();
            pos.x = min(map.width, max(0, pos.x + delta_x));
            pos.y = min(map.height, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

/// Used for removing the moving marker from a unit struct so it won't move the next time a unit gets moved
pub fn unmark_moving_unit(ecs: &mut World) -> Option<Position> {
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

pub fn unit_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    // Unit actions
    match ctx.key {
        None => return RunState::MoveUnit, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::A => try_move_unit(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_unit(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_unit(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_unit(0, 1, &mut gs.ecs),
            VirtualKeyCode::C => {
                // Maybe come back to this; I don't think it could be done but probably better to err on the side of caution
                let pos = unmark_moving_unit(&mut gs.ecs).unwrap();
                teleport_player(pos, &mut gs.ecs);
                return RunState::Paused;
            } // Need to come back and teleport cursor to unit's last location
            _ => {}
        },
    }
    RunState::MoveUnit
}

pub struct UnitOwnershipSystem {}

// Using the roguelike tutorial section on the inventory system as a guide for setting
// up the ability for players to control multiple units
impl<'a> System<'a> for UnitOwnershipSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteStorage<'a, UnitControl>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, OwnedBy>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (_player_entity, mut unit_control, _positions, mut owner) = data;

        for owned in unit_control.join() {
            owner
                .insert(
                    owned.unit,
                    OwnedBy {
                        owner: owned.owned_by,
                    },
                )
                .expect("unable to own");
        }

        unit_control.clear();
    }
}

pub fn get_unit(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let units = ecs.read_storage::<Unit>();
    let positions = ecs.read_storage::<Position>();

    let mut target_unit: Option<Entity> = None;

    for (unit_entity, _unit, position) in (&entities, &units, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_unit = Some(unit_entity);
        }
    }

    match target_unit {
        None => println!("no unit"),
        Some(unit) => {
            let mut controlled_by = ecs.write_storage::<UnitControl>();
            controlled_by
                .insert(
                    *player_entity,
                    UnitControl {
                        owned_by: *player_entity,
                        unit,
                    },
                )
                .expect("Unable to add unit");
        }
    }
}
